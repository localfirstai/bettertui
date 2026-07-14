use std::time::{Duration, Instant};

/// Configurable benchmarking harness for lightweight micro-benchmarks.
///
/// Usage:
/// ```
/// use bettertui_benchmark::BenchmarkHarness;
///
/// let mut harness = BenchmarkHarness::new("my_bench")
///     .with_iterations(1000)
///     .with_warmup(100);
///
/// harness.run(|| {
///     // code to benchmark
///     let x = 2 + 2;
///     std::hint::black_box(x);
/// });
///
/// let result = harness.result();
/// // Access: result.mean, result.throughput, etc.
/// ```
pub struct BenchmarkHarness {
    name: String,
    iterations: usize,
    warmup_iterations: usize,
    /// Tracked operations count (useful for throughput).
    ops: u64,
    duration: Duration,
    ran: bool,
}

impl Default for BenchmarkHarness {
    fn default() -> Self {
        Self::new("benchmark")
    }
}

impl BenchmarkHarness {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            iterations: 10_000,
            warmup_iterations: 1_000,
            ops: 0,
            duration: Duration::ZERO,
            ran: false,
        }
    }

    pub fn with_iterations(mut self, n: usize) -> Self {
        self.iterations = n;
        self
    }

    pub fn with_warmup(mut self, n: usize) -> Self {
        self.warmup_iterations = n;
        self
    }

    /// Run the benchmark. Warmup first, then timed iterations.
    pub fn run<F: FnMut()>(&mut self, mut f: F) {
        // Warmup
        for _ in 0..self.warmup_iterations {
            f();
        }

        // Timed run
        let start = Instant::now();
        for _ in 0..self.iterations {
            f();
        }
        self.duration = start.elapsed();
        self.ran = true;
    }

    /// Same as `run` but records the number of operations for throughput.
    pub fn run_with_ops<F: FnMut() -> u64>(&mut self, mut f: F) {
        for _ in 0..self.warmup_iterations {
            f();
        }

        let start = Instant::now();
        for _ in 0..self.iterations {
            self.ops += f();
        }
        self.duration = start.elapsed();
        self.ran = true;
    }

    pub fn result(&self) -> BenchmarkResult {
        assert!(self.ran, "must call run() before result()");
        let per_iter = self.duration / self.iterations as u32;
        let throughput = if self.ops > 0 {
            self.ops as f64 / self.duration.as_secs_f64()
        } else {
            0.0
        };
        BenchmarkResult {
            name: self.name.clone(),
            total_iterations: self.iterations,
            total_duration: self.duration,
            mean: per_iter,
            min: per_iter,
            max: per_iter,
            throughput,
            ops: self.ops,
        }
    }

    pub fn report(&self) -> String {
        let r = self.result();
        r.format()
    }
}

/// Statistics from a benchmark run.
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Benchmark name.
    pub name: String,
    /// Total iterations run.
    pub total_iterations: usize,
    /// Total wall-clock time.
    pub total_duration: Duration,
    /// Mean time per iteration.
    pub mean: Duration,
    /// Min time per iteration.
    pub min: Duration,
    /// Max time per iteration.
    pub max: Duration,
    /// Throughput in ops/second (0 if not measured).
    pub throughput: f64,
    /// Total operations counted.
    pub ops: u64,
}

impl BenchmarkResult {
    pub fn format(&self) -> String {
        let throughput_str = if self.throughput > 0.0 {
            format!(
                ", throughput: {:.2} ops/s ({:.2} Mops/s)",
                self.throughput,
                self.throughput / 1_000_000.0
            )
        } else {
            String::new()
        };

        format!(
            "[{}] {} iters in {:.2?} — mean: {:.2?}{}",
            self.name, self.total_iterations, self.total_duration, self.mean, throughput_str,
        )
    }
}

/// Operation counter for throughput measurement.
///
/// Wraps a `u64` counter and provides helpers for use inside benchmark loops:
///
/// ```
/// use bettertui_benchmark::OpCounter;
///
/// let mut counter = OpCounter::new();
/// let items = vec![1, 2, 3, 4, 5];
/// counter.count_items(items.iter());
/// assert_eq!(counter.get(), 5);
/// ```
#[derive(Debug, Clone, Default)]
pub struct OpCounter(u64);

impl OpCounter {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn reset(&mut self) {
        self.0 = 0;
    }

    pub fn increment(&mut self, by: u64) {
        self.0 = self.0.wrapping_add(by);
    }

    pub fn get(&self) -> u64 {
        self.0
    }

    pub fn count_item<T>(&mut self, _item: &T) {
        self.0 = self.0.wrapping_add(1);
    }

    pub fn count_items<T>(&mut self, items: impl IntoIterator<Item = T>) {
        for item in items {
            self.count_item(&item);
        }
    }
}
