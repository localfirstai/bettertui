//! Pixel-graphics protocol emitters: Kitty graphics, Sixel, and iTerm2 inline
//! images.
//!
//! [`GraphicsContext`](crate::graphics::GraphicsContext) draws into the cell
//! grid; this module is orthogonal — it produces the terminal escape sequences
//! that transmit *pixel* images (RGB/RGBA blobs or pre-encoded PNG bytes) to
//! terminals that support one of the graphics protocols. It follows the
//! `graphicsWrite` / `graphicsSixelWrite` / `graphicsItermWrite` /
//! `graphicsQuery` surface.
//!
//! The emitters are pure: they take image data and return the bytes to write to
//! the terminal. Capability detection (which protocol a terminal supports) lives
//! in [`crate::terminal::capabilities`].

use base64::Engine as _;

/// Pixel image format for a graphics payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    /// 24-bit RGB, 3 bytes per pixel.
    Rgb,
    /// 32-bit RGBA, 4 bytes per pixel.
    Rgba,
    /// Pre-encoded PNG file bytes.
    Png,
}

impl ImageFormat {
    /// The Kitty `f=` format code (24=RGB, 32=RGBA, 100=PNG).
    fn kitty_code(self) -> u32 {
        match self {
            Self::Rgb => 24,
            Self::Rgba => 32,
            Self::Png => 100,
        }
    }
}

/// Which pixel-graphics protocol a terminal speaks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsProtocol {
    Kitty,
    Sixel,
    Iterm2,
}

/// A pixel image to transmit.
#[derive(Debug, Clone)]
pub struct GraphicsImage<'a> {
    pub format: ImageFormat,
    /// Pixel width. For PNG this is informational (the terminal reads the PNG
    /// header); for raw RGB/RGBA it is required to interpret the buffer.
    pub width: u32,
    pub height: u32,
    /// Raw pixel bytes (RGB/RGBA) or PNG file bytes.
    pub data: &'a [u8],
}

// ─── Kitty graphics protocol ─────────────────────────────────────────────────

/// Chunk size (in base64 characters) for Kitty graphics transmission. The Kitty
/// spec recommends chunks of at most 4096 base64 bytes per escape.
const KITTY_CHUNK: usize = 4096;

/// Builds the Kitty graphics-protocol sequence(s) to transmit and display
/// `image` with the given numeric `id` (for later reference/deletion).
///
/// Emits `APC _G <control> ; <base64-chunk> ST` escapes, splitting the payload
/// into `KITTY_CHUNK`-sized pieces with the `m=1`/`m=0` continuation flag.
pub fn kitty_write(image: &GraphicsImage, id: u32) -> Vec<u8> {
    let encoded = base64::engine::general_purpose::STANDARD.encode(image.data);
    let bytes = encoded.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() + 64);

    if bytes.is_empty() {
        return out;
    }

    let chunks: Vec<&[u8]> = bytes.chunks(KITTY_CHUNK).collect();
    let last = chunks.len() - 1;

    for (i, chunk) in chunks.iter().enumerate() {
        out.extend_from_slice(b"\x1b_G");
        if i == 0 {
            // First chunk carries the full control block: action=transmit+display,
            // format, dimensions, id.
            let more = if last == 0 { 0 } else { 1 };
            let control =
                format!("a=T,f={},s={},v={},i={},m={}", image.format.kitty_code(), image.width, image.height, id, more);
            out.extend_from_slice(control.as_bytes());
        } else {
            let more = if i == last { 0 } else { 1 };
            out.extend_from_slice(format!("m={more}").as_bytes());
        }
        out.push(b';');
        out.extend_from_slice(chunk);
        out.extend_from_slice(b"\x1b\\");
    }

    out
}

/// Builds the Kitty sequence that deletes the image with `id` (`a=d,d=i,i=<id>`).
pub fn kitty_delete(id: u32) -> Vec<u8> {
    format!("\x1b_Ga=d,d=i,i={id}\x1b\\").into_bytes()
}

/// Builds the Kitty sequence that deletes all transmitted images (`a=d,d=A`).
pub fn kitty_delete_all() -> Vec<u8> {
    b"\x1b_Ga=d,d=A\x1b\\".to_vec()
}

// ─── Sixel ───────────────────────────────────────────────────────────────────

/// Builds a Sixel sequence for an RGB/RGBA image.
///
/// Produces `DCS q <palette> <sixel-data> ST`. This is a straightforward
/// (non-dithered) encoder that builds a color registry from the distinct colors
/// in the image; it is intended for small UI images/icons, not photo-quality
/// output. Returns an empty vec if the image is not raw RGB/RGBA.
pub fn sixel_write(image: &GraphicsImage) -> Vec<u8> {
    let bpp = match image.format {
        ImageFormat::Rgb => 3,
        ImageFormat::Rgba => 4,
        ImageFormat::Png => return Vec::new(),
    };
    let (w, h) = (image.width as usize, image.height as usize);
    if w == 0 || h == 0 || image.data.len() < w * h * bpp {
        return Vec::new();
    }

    // Build a palette of distinct colors (capped at 256 registers).
    let mut palette: Vec<(u8, u8, u8)> = Vec::new();
    let mut indices = vec![0u16; w * h];
    for (i, px) in indices.iter_mut().enumerate() {
        let o = i * bpp;
        let color = (image.data[o], image.data[o + 1], image.data[o + 2]);
        let idx = match palette.iter().position(|&c| c == color) {
            Some(p) => p,
            None if palette.len() < 256 => {
                palette.push(color);
                palette.len() - 1
            }
            None => 0, // palette full: fall back to the first color
        };
        *px = idx as u16;
    }

    let mut out = Vec::new();
    out.extend_from_slice(b"\x1bPq"); // DCS q — start sixel

    // Emit the palette: `#<n>;2;<r%>;<g%>;<b%>` (Sixel uses 0..100 components).
    for (n, &(r, g, b)) in palette.iter().enumerate() {
        let rp = (r as u32 * 100) / 255;
        let gp = (g as u32 * 100) / 255;
        let bp = (b as u32 * 100) / 255;
        out.extend_from_slice(format!("#{n};2;{rp};{gp};{bp}").as_bytes());
    }

    // Emit pixels in 6-row bands.
    let mut y = 0;
    while y < h {
        let band = (h - y).min(6);
        for (n, _) in palette.iter().enumerate() {
            out.extend_from_slice(format!("#{n}").as_bytes());
            for x in 0..w {
                let mut bits = 0u8;
                for row in 0..band {
                    if indices[(y + row) * w + x] as usize == n {
                        bits |= 1 << row;
                    }
                }
                out.push(0x3f + bits); // sixel data byte
            }
            out.push(b'$'); // carriage return (overlay next color on same band)
        }
        out.push(b'-'); // line feed to next band
        y += band;
    }

    out.extend_from_slice(b"\x1b\\"); // ST — end sixel
    out
}

// ─── iTerm2 inline images ────────────────────────────────────────────────────

/// Builds an iTerm2 inline-image sequence (`OSC 1337 ; File=... : <base64> ST`).
///
/// iTerm2 expects the *file* bytes (e.g. PNG) base64-encoded. `name` is an
/// optional display filename. `width`/`height` are optional cell dimensions
/// (`auto` when `None`).
pub fn iterm_write(file_bytes: &[u8], name: Option<&str>, width: Option<u32>, height: Option<u32>) -> Vec<u8> {
    let encoded = base64::engine::general_purpose::STANDARD.encode(file_bytes);
    let mut args = format!("size={};inline=1", file_bytes.len());
    if let Some(n) = name {
        let enc_name = base64::engine::general_purpose::STANDARD.encode(n.as_bytes());
        args = format!("name={enc_name};{args}");
    }
    if let Some(w) = width {
        args.push_str(&format!(";width={w}"));
    }
    if let Some(h) = height {
        args.push_str(&format!(";height={h}"));
    }
    format!("\x1b]1337;File={args}:{encoded}\x07").into_bytes()
}

// ─── Query ───────────────────────────────────────────────────────────────────

/// Returns the escape sequence(s) to probe which graphics protocols a terminal
/// supports, and which detected [`GraphicsProtocol`]s to expect from the
/// response. Kitty is probed with a tiny transmit + query (`a=q`); Sixel via the
/// Primary Device Attributes request (a `4` in the DA1 response means Sixel).
pub fn graphics_query() -> Vec<u8> {
    let mut out = Vec::new();
    // Kitty: query support for image id=1 without displaying it.
    out.extend_from_slice(b"\x1b_Gi=1,a=q;\x1b\\");
    // Sixel: Primary Device Attributes (response includes `4` when supported).
    out.extend_from_slice(b"\x1b[c");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rgb_image<'a>(data: &'a [u8], w: u32, h: u32) -> GraphicsImage<'a> {
        GraphicsImage { format: ImageFormat::Rgb, width: w, height: h, data }
    }

    #[test]
    fn kitty_write_single_chunk() {
        let data = [255u8, 0, 0]; // one red pixel
        let seq = kitty_write(&rgb_image(&data, 1, 1), 7);
        let s = String::from_utf8_lossy(&seq);
        assert!(s.starts_with("\x1b_G"), "starts with APC _G: {s:?}");
        assert!(s.contains("a=T"), "transmit+display action");
        assert!(s.contains("f=24"), "RGB format code");
        assert!(s.contains("i=7"), "carries the id");
        assert!(s.contains("m=0"), "single chunk is final");
        assert!(s.ends_with("\x1b\\"), "ends with ST");
    }

    #[test]
    fn kitty_write_chunks_large_payload() {
        // Enough pixels that base64 exceeds one 4096 chunk.
        let data = vec![1u8; 4096 * 3];
        let seq = kitty_write(&rgb_image(&data, 4096, 1), 1);
        let s = String::from_utf8_lossy(&seq);
        assert!(s.matches("\x1b_G").count() >= 2, "multi-chunk emits multiple escapes");
        assert!(s.contains("m=1"), "non-final chunk sets m=1");
        assert!(s.trim_end_matches("\x1b\\").ends_with(|c: char| c.is_ascii()), "ends cleanly");
    }

    #[test]
    fn kitty_delete_sequences() {
        assert_eq!(kitty_delete(3), b"\x1b_Ga=d,d=i,i=3\x1b\\");
        assert_eq!(kitty_delete_all(), b"\x1b_Ga=d,d=A\x1b\\");
    }

    #[test]
    fn sixel_write_emits_dcs_and_palette() {
        // 2x1 image: red, green.
        let data = [255u8, 0, 0, 0, 255, 0];
        let seq = sixel_write(&GraphicsImage { format: ImageFormat::Rgb, width: 2, height: 1, data: &data });
        let s = String::from_utf8_lossy(&seq);
        assert!(s.starts_with("\x1bPq"), "starts with DCS q: {s:?}");
        assert!(s.contains("#0;2;100;0;0"), "red palette entry");
        assert!(s.contains("#1;2;0;100;0"), "green palette entry");
        assert!(s.ends_with("\x1b\\"), "ends with ST");
    }

    #[test]
    fn sixel_rejects_png() {
        let img = GraphicsImage { format: ImageFormat::Png, width: 1, height: 1, data: &[0, 1, 2] };
        assert!(sixel_write(&img).is_empty());
    }

    #[test]
    fn sixel_rejects_short_buffer() {
        let data = [1u8, 2, 3]; // claims 2x2 RGB but only 1 pixel of data
        let img = GraphicsImage { format: ImageFormat::Rgb, width: 2, height: 2, data: &data };
        assert!(sixel_write(&img).is_empty());
    }

    #[test]
    fn iterm_write_encodes_file() {
        let seq = iterm_write(b"PNGDATA", Some("a.png"), Some(10), None);
        let s = String::from_utf8_lossy(&seq);
        assert!(s.starts_with("\x1b]1337;File="), "OSC 1337 File: {s:?}");
        assert!(s.contains("inline=1"));
        assert!(s.contains("size=7"));
        assert!(s.contains("width=10"));
        assert!(!s.contains("height="), "height omitted when None");
        // "PNGDATA" base64 == "UE5HREFUQQ=="
        assert!(s.contains("UE5HREFUQQ=="), "payload base64: {s:?}");
        assert!(s.ends_with('\x07'), "BEL terminator");
    }

    #[test]
    fn graphics_query_probes_kitty_and_sixel() {
        let seq = graphics_query();
        let s = String::from_utf8_lossy(&seq);
        assert!(s.contains("\x1b_Gi=1,a=q;"), "kitty query");
        assert!(s.contains("\x1b[c"), "DA1 for sixel");
    }
}
