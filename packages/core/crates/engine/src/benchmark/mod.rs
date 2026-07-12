//! Benchmark harness for measuring engine performance.
//!
//! Provides timing utilities, operation counters, and structured benchmarks
//! for measuring rendering, layout, diff, and other engine operations.

use std::time::{Duration, Instant};

/// A single benchmark measurement.
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Name of the benchmark.
    pub name: String,
    /// Number of iterations performed.
    pub iterations: usize,
    /// Total elapsed time.
    pub elapsed: Duration,
    /// Average time per iteration.
    pub avg_per_iter: Duration,
    /// Minimum iteration time.
    pub min: Duration,
    /// Maximum iteration time.
    pub max: Duration,
}

impl BenchmarkResult {
    /// Returns iterations per second.
    pub fn ops_per_sec(&self) -> f64 {
        if self.elapsed.is_zero() {
            return 0.0;
        }
        self.iterations as f64 / self.elapsed.as_secs_f64()
    }

    /// Returns a formatted summary string.
    pub fn summary(&self) -> String {
        format!(
            "{}: {} iterations in {:?} ({:.1} ops/s, avg {:?}, min {:?}, max {:?})",
            self.name,
            self.iterations,
            self.elapsed,
            self.ops_per_sec(),
            self.avg_per_iter,
            self.min,
            self.max,
        )
    }
}

/// Tracks operation counts during a benchmark.
#[derive(Debug, Default)]
pub struct OpCounter {
    count: u64,
}

impl OpCounter {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn reset(&mut self) {
        self.count = 0;
    }
}

/// A benchmark harness for running and collecting performance measurements.
#[derive(Debug)]
pub struct BenchmarkHarness {
    /// Stored results from completed benchmarks.
    results: Vec<BenchmarkResult>,
    /// Default number of warmup iterations.
    warmup_iterations: usize,
    /// Default minimum duration per benchmark.
    min_duration: Duration,
}

impl Default for BenchmarkHarness {
    fn default() -> Self {
        Self::new()
    }
}

impl BenchmarkHarness {
    /// Creates a new BenchmarkHarness.
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            warmup_iterations: 100,
            min_duration: Duration::from_millis(100),
        }
    }

    /// Sets the number of warmup iterations.
    pub fn with_warmup(mut self, iterations: usize) -> Self {
        self.warmup_iterations = iterations;
        self
    }

    /// Sets the minimum duration per benchmark.
    pub fn with_min_duration(mut self, duration: Duration) -> Self {
        self.min_duration = duration;
        self
    }

    /// Runs a benchmark with the given name and closure.
    ///
    /// The closure is called repeatedly until enough time has elapsed
    /// to produce a stable measurement.
    pub fn bench(&mut self, name: &str, mut f: impl FnMut()) {
        // Warmup
        for _ in 0..self.warmup_iterations {
            f();
        }

        let mut iterations = 0;
        let mut times = Vec::new();
        let start = Instant::now();

        loop {
            let iter_start = Instant::now();
            f();
            let iter_elapsed = iter_start.elapsed();
            times.push(iter_elapsed);
            iterations += 1;

            let total_elapsed = start.elapsed();
            if total_elapsed >= self.min_duration && iterations >= 10 {
                break;
            }
            // Safety valve: don't run forever
            if iterations >= 1_000_000 {
                break;
            }
        }

        let elapsed = start.elapsed();
        let min = times.iter().copied().min().unwrap_or_default();
        let max = times.iter().copied().max().unwrap_or_default();
        let avg_per_iter = elapsed / iterations as u32;

        let result = BenchmarkResult {
            name: name.to_string(),
            iterations,
            elapsed,
            avg_per_iter,
            min,
            max,
        };

        self.results.push(result);
    }

    /// Returns all stored benchmark results.
    pub fn results(&self) -> &[BenchmarkResult] {
        &self.results
    }

    /// Returns a specific benchmark result by name.
    pub fn find(&self, name: &str) -> Option<&BenchmarkResult> {
        self.results.iter().find(|r| r.name == name)
    }

    /// Clears all stored results.
    pub fn clear(&mut self) {
        self.results.clear();
    }

    /// Prints a summary of all results.
    pub fn summary(&self) -> String {
        let mut output = String::new();
        output.push_str("Benchmark Results:\n");
        output.push_str(&"=".repeat(80));
        output.push('\n');
        for result in &self.results {
            output.push_str(&result.summary());
            output.push('\n');
        }
        output.push_str(&"=".repeat(80));
        output.push('\n');
        output.push_str(&format!("Total benchmarks: {}\n", self.results.len()));
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bench_runs_multiple_iterations() {
        let mut harness = BenchmarkHarness::new()
            .with_warmup(0)
            .with_min_duration(Duration::from_millis(10));
        let mut count = 0;
        harness.bench("test", || {
            count += 1;
        });
        assert!(count > 10);
        let result = harness.find("test").unwrap();
        assert!(result.iterations > 10);
    }

    #[test]
    fn bench_result_ops_per_sec() {
        let result = BenchmarkResult {
            name: "test".to_string(),
            iterations: 1000,
            elapsed: Duration::from_secs(1),
            avg_per_iter: Duration::from_micros(1),
            min: Duration::from_micros(1),
            max: Duration::from_micros(2),
        };
        assert_eq!(result.ops_per_sec(), 1000.0);
    }

    #[test]
    fn bench_result_summary() {
        let result = BenchmarkResult {
            name: "test".to_string(),
            iterations: 100,
            elapsed: Duration::from_millis(50),
            avg_per_iter: Duration::from_micros(500),
            min: Duration::from_micros(400),
            max: Duration::from_micros(600),
        };
        let summary = result.summary();
        assert!(summary.contains("test"));
        assert!(summary.contains("100"));
    }

    #[test]
    fn harness_results() {
        let mut harness = BenchmarkHarness::new()
            .with_warmup(0)
            .with_min_duration(Duration::from_millis(10));
        harness.bench("a", || {});
        harness.bench("b", || {});
        assert_eq!(harness.results().len(), 2);
    }

    #[test]
    fn harness_find() {
        let mut harness = BenchmarkHarness::new()
            .with_warmup(0)
            .with_min_duration(Duration::from_millis(10));
        harness.bench("target", || {});
        assert!(harness.find("target").is_some());
        assert!(harness.find("missing").is_none());
    }

    #[test]
    fn harness_clear() {
        let mut harness = BenchmarkHarness::new()
            .with_warmup(0)
            .with_min_duration(Duration::from_millis(10));
        harness.bench("test", || {});
        assert_eq!(harness.results().len(), 1);
        harness.clear();
        assert_eq!(harness.results().len(), 0);
    }

    #[test]
    fn op_counter() {
        let mut counter = OpCounter::new();
        assert_eq!(counter.count(), 0);
        counter.increment();
        counter.increment();
        assert_eq!(counter.count(), 2);
        counter.reset();
        assert_eq!(counter.count(), 0);
    }

    #[test]
    fn summary_output() {
        let mut harness = BenchmarkHarness::new()
            .with_warmup(0)
            .with_min_duration(Duration::from_millis(10));
        harness.bench("test", || {});
        let output = harness.summary();
        assert!(output.contains("Benchmark Results"));
        assert!(output.contains("test"));
    }
}
