import { existsSync, mkdtempSync, readFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterAll, describe, expect, it } from "vitest";
import {
  createEngine,
  loggerFlush,
  loggerGetDiagnostics,
  loggerGetLevel,
  loggerInit,
  loggerSetLevel,
  loggerSetModuleFilter,
} from "../platform/binding";
import { type DiagnosticSnapshot, cacheHitRatio } from "../platform/logger";

// These tests exercise the real native logger through the napi binding.
// The tracing subscriber is process-global and can only be installed once,
// so `loggerInit` is called a single time here (writing to a temp log file)
// and the remaining tests drive the runtime-mutable surface (level, module
// filter, diagnostics, flush) plus assert that file logging actually wrote.

const logDir = mkdtempSync(join(tmpdir(), "bettertui-logger-"));
const logFile = join(logDir, "engine.log");

describe("logger native integration", () => {
  it("initializes with an explicit file and reports the configured level", () => {
    // color/timestamp off keeps stderr output quiet during the run.
    loggerInit({ level: "info", color: false, timestamp: false, module: true, file: logFile });
    expect(loggerGetLevel()).toBe("info");
  });

  it("writes log records to the configured file", () => {
    // The init above already emitted a "Logger initialized" record; drive a
    // render so more records flow, then assert the file exists with content.
    const engine = createEngine(80, 24);
    engine.beginFrame();
    engine.commitFrame();
    engine.renderFull();
    loggerFlush();
    engine.shutdown();

    expect(existsSync(logFile)).toBe(true);
    const contents = readFileSync(logFile, "utf8");
    expect(contents.length).toBeGreaterThan(0);
    expect(contents).toContain("Logger initialized");
  });

  it("changes the log level at runtime", () => {
    loggerSetLevel("trace");
    expect(loggerGetLevel()).toBe("trace");

    loggerSetLevel("warn");
    expect(loggerGetLevel()).toBe("warn");

    loggerSetLevel("error");
    expect(loggerGetLevel()).toBe("error");
  });

  it("accepts module filter include/exclude lists without throwing", () => {
    expect(() =>
      loggerSetModuleFilter(["bettertui_engine::render"], ["bettertui_engine::pty"]),
    ).not.toThrow();
    // Passing nothing clears the filter.
    expect(() => loggerSetModuleFilter()).not.toThrow();
  });

  it("returns a fully-shaped diagnostic snapshot", () => {
    const d = loggerGetDiagnostics();
    const keys: Array<keyof DiagnosticSnapshot> = [
      "renderCalls",
      "renderBytes",
      "eventDispatches",
      "layoutComputations",
      "cacheHits",
      "cacheMisses",
      "allocations",
      "averageFrameTime",
      "fps",
    ];
    for (const key of keys) {
      expect(typeof d[key]).toBe("number");
      expect(Number.isNaN(d[key])).toBe(false);
    }
  });

  it("increments render/layout counters when the engine renders", () => {
    const before = loggerGetDiagnostics();

    const engine = createEngine(80, 24);
    const root = engine.root();
    const box = engine.createNode("Box");
    engine.appendChild(root, box);
    engine.beginFrame();
    engine.commitFrame();
    engine.renderFull();
    engine.render();
    engine.render();

    const after = loggerGetDiagnostics();
    expect(after.renderCalls).toBeGreaterThan(before.renderCalls);
    expect(after.renderBytes).toBeGreaterThan(before.renderBytes);
    expect(after.layoutComputations).toBeGreaterThanOrEqual(before.layoutComputations);
    engine.shutdown();
  });

  it("computes cache hit ratio from a snapshot", () => {
    expect(
      cacheHitRatio({
        renderCalls: 0,
        renderBytes: 0,
        eventDispatches: 0,
        layoutComputations: 0,
        cacheHits: 3,
        cacheMisses: 1,
        allocations: 0,
        averageFrameTime: 0,
        fps: 0,
      }),
    ).toBeCloseTo(0.75);

    // No accesses → 0, not NaN.
    expect(
      cacheHitRatio({
        renderCalls: 0,
        renderBytes: 0,
        eventDispatches: 0,
        layoutComputations: 0,
        cacheHits: 0,
        cacheMisses: 0,
        allocations: 0,
        averageFrameTime: 0,
        fps: 0,
      }),
    ).toBe(0);
  });

  it("flushes without throwing", () => {
    expect(() => loggerFlush()).not.toThrow();
  });

  afterAll(() => {
    loggerFlush();
  });
});
