//! Tests for the benchmark module.

use std::time::Duration;

use bettertui_engine::benchmark::{BenchmarkHarness, BenchmarkResult, OpCounter};

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
