export interface BenchmarkResult {
  name: string;
  iterations: number;
  opsPerSecond: number;
  avgDurationMs: number;
  memoryAllocatedBytes: number;
}

export async function runBenchmark(
  name: string,
  fn: () => void | Promise<void>,
  iterations = 1000,
): Promise<BenchmarkResult> {
  const startMemory = process.memoryUsage().heapUsed;
  const startTime = performance.now();

  for (let i = 0; i < iterations; i++) {
    await fn();
  }

  const endTime = performance.now();
  const endMemory = process.memoryUsage().heapUsed;

  const totalDurationMs = endTime - startTime;
  const avgDurationMs = totalDurationMs / iterations;
  const opsPerSecond = Math.round((iterations / totalDurationMs) * 1000);
  const memoryAllocatedBytes = Math.max(0, endMemory - startMemory);

  return {
    name,
    iterations,
    opsPerSecond,
    avgDurationMs,
    memoryAllocatedBytes,
  };
}
