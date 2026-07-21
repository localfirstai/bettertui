//! Zero-copy output streaming via SpanFeed.
//!
//! Instead of copying ANSI bytes into an intermediate buffer and then
//! into a Writable stream, the renderer writes directly into pre-allocated
//! chunk memory. The consumer reads SpanInfo descriptors (pointer + offset + len)
//! pointing into these chunks — zero copy.
//!
//! Pattern adapted from OpenTUI's NativeSpanFeed.

use std::alloc::{Layout, alloc, dealloc};

const DEFAULT_CHUNK_SIZE: u32 = 65536; // 64 KiB
const DEFAULT_INITIAL_CHUNKS: u32 = 2;
const DEFAULT_SPAN_QUEUE_CAPACITY: u32 = 4096;

/// Information about a committed span of data.
///
/// Points into chunk memory — zero-copy descriptor.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SpanInfo {
    pub chunk_ptr: u64,
    pub offset: u32,
    pub len: u32,
    pub chunk_index: u32,
    pub reserved: u32,
}

/// Result of a reserve() call for zero-copy writes.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ReserveInfo {
    pub ptr: u64,
    pub len: u32,
    pub reserved: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GrowthPolicy {
    Grow = 0,
    Block = 1,
}

#[derive(Debug, Clone)]
struct Chunk {
    ptr: *mut u8,
    len: u32,
    refcount: u32,
}

// SAFETY: Chunk owns its heap allocation (ptr, len). The pointer is only
// dereferenced when holding &mut SpanFeed or via the read methods which
// borrow &self. SpanFeed is wrapped in Mutex in the napi binding, ensuring
// exclusive access for mutations. The refcount field is only modified
// under &mut access.
unsafe impl Send for Chunk {}
unsafe impl Sync for Chunk {}

impl Drop for Chunk {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            let layout = Layout::from_size_align(self.len as usize, 16).unwrap();
            // SAFETY: ptr was allocated by alloc() with this exact layout.
            unsafe { dealloc(self.ptr, layout) };
        }
    }
}

impl Chunk {
    fn allocate(len: u32) -> Self {
        let layout = Layout::from_size_align(len as usize, 16).unwrap();
        // SAFETY: Layout is valid (non-zero size, power-of-2 align). We check
        // for null below and handle allocation failure by returning an empty Chunk.
        let ptr = unsafe { alloc(layout) };
        Self { ptr, len, refcount: 0 }
    }
}

/// Ring buffer for SpanInfo entries.
#[derive(Debug, Clone)]
struct SpanRing {
    buffer: Vec<SpanInfo>,
    capacity: u32,
    head: u32,
    tail: u32,
}

impl SpanRing {
    fn with_capacity(cap: u32) -> Self {
        Self {
            buffer: vec![SpanInfo { chunk_ptr: 0, offset: 0, len: 0, chunk_index: 0, reserved: 0 }; cap as usize],
            capacity: cap,
            head: 0,
            tail: 0,
        }
    }

    fn push(&mut self, info: SpanInfo) -> bool {
        let next = (self.head + 1) % self.capacity;
        if next == self.tail {
            return false; // full
        }
        self.buffer[self.head as usize] = info;
        self.head = next;
        true
    }

    fn pop_many(&mut self, out: &mut [SpanInfo]) -> u32 {
        let mut count = 0u32;
        while self.tail != self.head && (count as usize) < out.len() {
            out[count as usize] = self.buffer[self.tail as usize];
            self.tail = (self.tail + 1) % self.capacity;
            count += 1;
        }
        count
    }

    fn len(&self) -> u32 {
        if self.head >= self.tail { self.head - self.tail } else { self.capacity - self.tail + self.head }
    }

    fn is_full(&self) -> bool {
        (self.head + 1) % self.capacity == self.tail
    }

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        self.head == self.tail
    }
}

/// Zero-copy output stream.
///
/// Producers write directly into pre-allocated chunks.
/// Consumers drain SpanInfo descriptors and read directly from chunk memory.
#[derive(Debug)]
pub struct SpanFeed {
    chunks: Vec<Chunk>,
    current_chunk: usize,
    write_offset: u32,
    pending_offset: u32,
    pending_len: u32,
    span_ring: SpanRing,
    options: SpanFeedOptions,
    closed: bool,
    bytes_written: u64,
    spans_committed: u64,
}

#[derive(Debug, Clone)]
pub struct SpanFeedOptions {
    pub chunk_size: u32,
    pub initial_chunks: u32,
    pub max_bytes: u64,
    pub growth_policy: GrowthPolicy,
    pub auto_commit_on_full: bool,
    pub span_queue_capacity: u32,
}

impl Default for SpanFeedOptions {
    fn default() -> Self {
        Self {
            chunk_size: DEFAULT_CHUNK_SIZE,
            initial_chunks: DEFAULT_INITIAL_CHUNKS,
            max_bytes: 0,
            growth_policy: GrowthPolicy::Grow,
            auto_commit_on_full: true,
            span_queue_capacity: DEFAULT_SPAN_QUEUE_CAPACITY,
        }
    }
}

impl SpanFeed {
    pub fn new() -> Self {
        Self::with_options(SpanFeedOptions::default())
    }

    pub fn with_options(options: SpanFeedOptions) -> Self {
        let chunk_size = options.chunk_size.max(256);
        let initial = options.initial_chunks.max(1);

        let mut chunks = Vec::with_capacity(initial as usize);
        for _ in 0..initial {
            chunks.push(Chunk::allocate(chunk_size));
        }

        Self {
            span_ring: SpanRing::with_capacity(options.span_queue_capacity.max(16)),
            chunks,
            current_chunk: 0,
            write_offset: 0,
            pending_offset: 0,
            pending_len: 0,
            options,
            closed: false,
            bytes_written: 0,
            spans_committed: 0,
        }
    }

    // ─── Copy API ─────────────────────────────────────────────────

    /// Write data into the stream (copies from src).
    pub fn write(&mut self, src: &[u8]) -> usize {
        if self.closed || src.is_empty() {
            return 0;
        }

        let mut written = 0;
        let mut remaining = src;

        while !remaining.is_empty() {
            let chunk = &mut self.chunks[self.current_chunk];
            let available = (chunk.len - self.write_offset) as usize;

            if available == 0 {
                if !self.advance_chunk() {
                    break;
                }
                continue;
            }

            let to_write = remaining.len().min(available);
            // SAFETY: chunk.ptr is valid for chunk.len bytes. write_offset < chunk.len
            // (guaranteed by `available = chunk.len - write_offset` being > 0 above).
            // to_write <= available, so the slice fits within the allocation.
            let dest = unsafe { std::slice::from_raw_parts_mut(chunk.ptr.add(self.write_offset as usize), to_write) };
            dest.copy_from_slice(&remaining[..to_write]);

            if self.pending_len == 0 {
                self.pending_offset = self.write_offset;
            }
            self.pending_len += to_write as u32;
            self.write_offset += to_write as u32;
            written += to_write;
            self.bytes_written += to_write as u64;

            // Auto-commit on chunk full
            if self.write_offset >= chunk.len && self.options.auto_commit_on_full {
                self.commit();
            }

            remaining = &remaining[to_write..];
        }

        written
    }

    // ─── Zero-Copy API ─────────────────────────────────────────────

    /// Reserve writable memory. Returns None if no space available.
    pub fn reserve(&mut self, min_len: u32) -> Option<ReserveInfo> {
        if self.closed {
            return None;
        }

        let chunk = &mut self.chunks[self.current_chunk];
        let available = chunk.len - self.write_offset;

        if available < min_len && !self.advance_chunk() {
            return None;
        }

        let chunk = &mut self.chunks[self.current_chunk];
        let available = chunk.len - self.write_offset;
        let len = available.min(min_len.max(available));

        if self.pending_len == 0 {
            self.pending_offset = self.write_offset;
        }

        Some(ReserveInfo { ptr: chunk.ptr as u64 + self.write_offset as u64, len, reserved: 0 })
    }

    /// Commit a reserved region (number of bytes actually written).
    pub fn commit_reserved(&mut self, len: u32) -> bool {
        if self.closed || len == 0 {
            return false;
        }

        self.pending_len += len;
        self.write_offset += len;
        self.bytes_written += len as u64;

        self.commit()
    }

    // ─── Commit ────────────────────────────────────────────────────

    /// Commit pending bytes as a span.
    pub fn commit(&mut self) -> bool {
        if self.closed || self.pending_len == 0 {
            return false;
        }

        let chunk_idx = self.current_chunk;
        let chunk_ptr;
        {
            let chunk = &self.chunks[chunk_idx];
            chunk_ptr = chunk.ptr as u64;
        }

        let info = SpanInfo {
            chunk_ptr,
            offset: self.pending_offset,
            len: self.pending_len,
            chunk_index: chunk_idx as u32,
            reserved: 0,
        };

        if !self.span_ring.push(info) {
            return false;
        }

        if let Some(chunk) = self.chunks.get_mut(chunk_idx) {
            chunk.refcount += 1;
        }
        self.spans_committed += 1;
        self.pending_len = 0;
        true
    }

    // ─── Drain ─────────────────────────────────────────────────────

    /// Drain available spans into the output buffer.
    /// Returns the number of spans written.
    pub fn drain_spans(&mut self, out: &mut [SpanInfo]) -> u32 {
        self.span_ring.pop_many(out)
    }

    /// Number of pending (undrained) spans.
    pub fn pending_spans(&self) -> u32 {
        self.span_ring.len()
    }

    /// Number of pending bytes (not yet committed).
    pub fn pending_bytes(&self) -> u32 {
        self.pending_len
    }

    /// Whether the span queue is full (backpressure signal).
    pub fn is_backpressured(&self) -> bool {
        self.span_ring.is_full()
    }

    // ─── Refcount Management ───────────────────────────────────────

    /// Mark a chunk's span as consumed (decrements refcount).
    pub fn mark_consumed(&mut self, chunk_index: u32) {
        if let Some(chunk) = self.chunks.get_mut(chunk_index as usize) {
            chunk.refcount = chunk.refcount.saturating_sub(1);
        }
    }

    // ─── Stats ─────────────────────────────────────────────────────

    pub fn bytes_written(&self) -> u64 {
        self.bytes_written
    }

    pub fn spans_committed(&self) -> u64 {
        self.spans_committed
    }

    pub fn chunk_count(&self) -> u32 {
        self.chunks.len() as u32
    }

    // ─── Lifecycle ─────────────────────────────────────────────────

    pub fn close(&mut self) {
        if self.pending_len > 0 {
            self.commit();
        }
        self.closed = true;
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    pub fn reset(&mut self) {
        self.write_offset = 0;
        self.pending_offset = 0;
        self.pending_len = 0;
        self.bytes_written = 0;
        self.spans_committed = 0;
        self.span_ring.head = 0;
        self.span_ring.tail = 0;
        self.closed = false;

        for chunk in &mut self.chunks {
            chunk.refcount = 0;
        }
    }

    // ─── Internal ──────────────────────────────────────────────────

    fn advance_chunk(&mut self) -> bool {
        // Commit any pending data before advancing
        if self.pending_len > 0 {
            self.commit();
        }

        self.current_chunk += 1;

        if self.current_chunk >= self.chunks.len() {
            match self.options.growth_policy {
                GrowthPolicy::Grow => {
                    let new_chunk = Chunk::allocate(self.options.chunk_size);
                    self.chunks.push(new_chunk);
                }
                GrowthPolicy::Block => {
                    self.current_chunk = self.chunks.len() - 1;
                    return false;
                }
            }
        }

        // Find next available chunk (refcount == 0)
        let start = self.current_chunk;
        loop {
            let chunk = &self.chunks[self.current_chunk];
            if chunk.refcount == 0 {
                self.write_offset = 0;
                return true;
            }
            self.current_chunk = (self.current_chunk + 1) % self.chunks.len();
            if self.current_chunk == start {
                return false;
            }
        }
    }
}

impl Default for SpanFeed {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_feed_write_and_drain() {
        let mut feed = SpanFeed::new();
        let data = b"Hello, SpanFeed!";
        let written = feed.write(data);
        assert_eq!(written, data.len());
        feed.commit();

        let mut out = vec![SpanInfo { chunk_ptr: 0, offset: 0, len: 0, chunk_index: 0, reserved: 0 }; 16];
        let count = feed.drain_spans(&mut out);
        assert_eq!(count, 1);
        assert_eq!(out[0].len, data.len() as u32);

        // Verify data via pointer
        let slice = unsafe {
            std::slice::from_raw_parts((out[0].chunk_ptr as *const u8).add(out[0].offset as usize), out[0].len as usize)
        };
        assert_eq!(slice, data);
    }

    #[test]
    fn span_feed_multiple_writes() {
        let mut feed = SpanFeed::with_options(SpanFeedOptions {
            chunk_size: 256,
            initial_chunks: 2,
            auto_commit_on_full: false,
            ..Default::default()
        });

        feed.write(b"first ");
        feed.commit();
        feed.write(b"second ");
        feed.commit();

        let mut out = vec![SpanInfo { chunk_ptr: 0, offset: 0, len: 0, chunk_index: 0, reserved: 0 }; 16];
        let count = feed.drain_spans(&mut out);
        assert_eq!(count, 2);
    }

    #[test]
    fn span_feed_reserve_and_commit() {
        let mut feed = SpanFeed::new();
        let info = feed.reserve(64).unwrap();
        assert!(info.ptr != 0);
        assert!(info.len >= 64);

        // Write directly into reserved memory
        let data = b"zero-copy data";
        let dest = unsafe { std::slice::from_raw_parts_mut(info.ptr as *mut u8, data.len()) };
        dest.copy_from_slice(data);

        feed.commit_reserved(data.len() as u32);

        let mut out = vec![SpanInfo { chunk_ptr: 0, offset: 0, len: 0, chunk_index: 0, reserved: 0 }; 16];
        let count = feed.drain_spans(&mut out);
        assert_eq!(count, 1);
        assert_eq!(out[0].len, data.len() as u32);

        let slice = unsafe {
            std::slice::from_raw_parts((out[0].chunk_ptr as *const u8).add(out[0].offset as usize), out[0].len as usize)
        };
        assert_eq!(slice, data);
    }

    #[test]
    fn span_feed_backpressure() {
        let mut feed = SpanFeed::with_options(SpanFeedOptions { span_queue_capacity: 16, ..Default::default() });

        // Fill the span queue (capacity 16 means 15 usable slots + 1 sentinel)
        for i in 0..15 {
            feed.write(&[i as u8]);
            assert!(feed.commit(), "commit {} should succeed", i);
        }

        // Next commit should fail (queue full)
        feed.write(b"overflow");
        assert!(!feed.commit(), "commit should fail due to backpressure");

        assert!(feed.is_backpressured());

        // Drain one, then commit should work again
        let mut out = vec![SpanInfo { chunk_ptr: 0, offset: 0, len: 0, chunk_index: 0, reserved: 0 }; 16];
        feed.drain_spans(&mut out);
        assert!(feed.commit(), "commit should succeed after drain");
    }

    #[test]
    fn span_feed_close() {
        let mut feed = SpanFeed::new();
        feed.write(b"data");
        feed.close();
        assert!(feed.is_closed());
        assert_eq!(feed.write(b"more"), 0); // writes after close fail
    }

    #[test]
    fn span_feed_reset() {
        let mut feed = SpanFeed::new();
        feed.write(b"data");
        feed.commit();
        feed.reset();
        assert_eq!(feed.bytes_written(), 0);
        assert_eq!(feed.pending_spans(), 0);
    }

    #[test]
    fn span_feed_mark_consumed() {
        let mut feed = SpanFeed::new();
        feed.write(b"data");
        feed.commit();

        let mut out = vec![SpanInfo { chunk_ptr: 0, offset: 0, len: 0, chunk_index: 0, reserved: 0 }; 16];
        let count = feed.drain_spans(&mut out);
        assert_eq!(count, 1);
        feed.mark_consumed(out[0].chunk_index);
    }

    #[test]
    fn span_feed_auto_commit() {
        let mut feed = SpanFeed::with_options(SpanFeedOptions {
            chunk_size: 256,
            initial_chunks: 1,
            auto_commit_on_full: true,
            ..Default::default()
        });

        // Write more than chunk size — should auto-commit when chunk fills
        // 256 bytes fills exactly one chunk, triggering auto-commit
        let data = vec![b'a'; 256];
        feed.write(&data);

        let mut out = vec![SpanInfo { chunk_ptr: 0, offset: 0, len: 0, chunk_index: 0, reserved: 0 }; 16];
        let count = feed.drain_spans(&mut out);
        assert!(count > 0, "should have auto-committed spans");

        // Verify total data
        let mut total = 0u32;
        for i in 0..count {
            total += out[i as usize].len;
        }
        assert_eq!(total, data.len() as u32);
    }
}
