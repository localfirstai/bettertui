/**
 * engine-frame.bench.ts
 *
 * End-to-end single-frame render benchmark that exercises the napi bridge
 * (`bettertui_engine.node`) + Rust layout (`taffy.rs`) + framebuffer diff
 * (`framebuffer.rs` + `dirty_diff.rs`).
 *
 * Reference: `.opencode/references/opentui/packages/core/src/benchmark/render-traversal-benchmark.ts`
 * (928 LoC) and `layout-benchmark.ts` (2,547 LoC).
 *
 * Preconditions: `@bettertui/core` must be built (`pnpm --filter @bettertui/core build`)
 * so `bettertui_engine.node` is loadable from `packages/core/dist/`.
 */

import { type Command, CommandBuffer, type NapiEngine, createEngine } from "@bettertui/core";
import { bench, describe } from "vitest";

function makeEngine(): NapiEngine {
  return createEngine(120, 40);
}

function buildGrid(buffer: CommandBuffer, rows: number, cols: number): void {
  buffer.push({ type: "BeginFrame", frameId: 1 });
  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < cols; c++) {
      const id = `n_${r}_${c}`;
      buffer.push({ type: "CreateNode", id, kind: "Box" });
      buffer.push({ type: "SetText", id, text: "x" });
    }
  }
  buffer.push({ type: "CommitFrame", frameId: 1 });
}

function drainAndRender(engine: NapiEngine, buffer: CommandBuffer): void {
  const commands = buffer.drain();
  engine.processCommands(JSON.stringify(commands));
  engine.beginFrame();
  engine.render();
  engine.commitFrame();
}

describe("NativeEngine — single frame", () => {
  bench(
    "empty frame (no commands)",
    () => {
      const engine = makeEngine();
      engine.beginFrame();
      engine.render();
      engine.commitFrame();
      engine.shutdown();
    },
    { iterations: 20, time: 500 },
  );

  bench(
    "10-node frame",
    () => {
      const engine = makeEngine();
      const buffer = new CommandBuffer();
      buildGrid(buffer, 2, 5);
      drainAndRender(engine, buffer);
      engine.shutdown();
    },
    { iterations: 20, time: 500 },
  );

  bench(
    "100-node frame",
    () => {
      const engine = makeEngine();
      const buffer = new CommandBuffer();
      buildGrid(buffer, 10, 10);
      drainAndRender(engine, buffer);
      engine.shutdown();
    },
    { iterations: 10, time: 1000 },
  );

  bench(
    "1000-node frame",
    () => {
      const engine = makeEngine();
      const buffer = new CommandBuffer();
      buildGrid(buffer, 50, 20);
      drainAndRender(engine, buffer);
      engine.shutdown();
    },
    { iterations: 5, time: 2000 },
  );
});

describe("NativeEngine — incremental re-render", () => {
  bench(
    "re-render same tree (dirty diff should be cheap)",
    () => {
      const engine = makeEngine();
      const buffer = new CommandBuffer();
      buildGrid(buffer, 10, 10);
      drainAndRender(engine, buffer);
      // second render, no command changes — exercises dirty_diff fast path
      engine.beginFrame();
      engine.render();
      engine.commitFrame();
      engine.shutdown();
    },
    { iterations: 10, time: 1000 },
  );

  bench(
    "re-render after text mutation on 1% of nodes",
    () => {
      const engine = makeEngine();
      const buffer = new CommandBuffer();
      buildGrid(buffer, 10, 10);
      drainAndRender(engine, buffer);
      // mutate 1 of 100 nodes
      buffer.push({ type: "SetText", id: "n_0_0", text: "y" });
      drainAndRender(engine, buffer);
      engine.shutdown();
    },
    { iterations: 10, time: 1000 },
  );
});

describe("NativeEngine — JSON bridge overhead", () => {
  // isolates the cost of processCommands JSON serialisation + napi boundary
  bench(
    "processCommands 100-entry buffer (no render)",
    () => {
      const engine = makeEngine();
      const buffer = new CommandBuffer();
      buildGrid(buffer, 10, 10);
      const commands = buffer.drain() as Command[];
      engine.processCommands(JSON.stringify(commands));
      engine.shutdown();
    },
    { iterations: 20, time: 500 },
  );
});
