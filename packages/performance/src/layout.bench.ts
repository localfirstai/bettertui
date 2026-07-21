/**
 * layout.bench.ts
 *
 * Tree-only layout benchmark via the Rust taffy integration. Bypasses the
 * framebuffer / ANSI encoder to isolate the layout pass cost.
 *
 * OpenTUI counterpart: `.opencode/references/opentui/packages/core/src/benchmark/layout-benchmark.ts`
 * (2,547 LoC) — that benchmark drives Yoga Wasm directly via the TS-side
 * `OptimizedBuffer`. Here we route through `NativeEngine.processCommands` so
 * the napi cost is included; the rendering step is skipped.
 *
 * Preconditions: `@bettertui/core` must be built.
 */

import { type Command, CommandBuffer, type NapiEngine, createEngine } from "@bettertui/core";
import { bench, describe } from "vitest";

interface TreeNode {
  id: string;
  children: TreeNode[];
}

function makeTree(depth: number, branching: number, counter: { n: number }): TreeNode {
  const id = `t_${counter.n++}`;
  if (depth === 0) return { id, children: [] };
  const children: TreeNode[] = [];
  for (let i = 0; i < branching; i++) {
    children.push(makeTree(depth - 1, branching, counter));
  }
  return { id, children };
}

function emitLayoutCommands(buffer: CommandBuffer, node: TreeNode, parentId?: string): void {
  buffer.push({ type: "CreateNode", id: node.id, kind: "Box" });
  buffer.push({
    type: "SetFlexDirection",
    id: node.id,
    direction: "column",
  });
  buffer.push({ type: "SetFlexGrow", id: node.id, value: 1 });
  buffer.push({ type: "SetFlexShrink", id: node.id, value: 1 });
  if (parentId) {
    buffer.push({ type: "AppendChild", parent: parentId, child: node.id });
  }
  for (const child of node.children) {
    emitLayoutCommands(buffer, child, node.id);
  }
}

function setup(
  engine: NapiEngine,
  buffer: CommandBuffer,
  depth: number,
  branching: number,
): number {
  const counter = { n: 0 };
  const tree = makeTree(depth, branching, counter);
  buffer.push({ type: "BeginFrame", frameId: 1 });
  emitLayoutCommands(buffer, tree);
  buffer.push({ type: "CommitFrame", frameId: 1 });
  const commands = buffer.drain() as Command[];
  engine.processCommands(JSON.stringify(commands));
  return counter.n;
}

describe("Layout — taffy integration via processCommands", () => {
  bench(
    "linear chain of 100 nodes (depth=100, branching=1)",
    () => {
      const engine = createEngine(80, 24);
      const buffer = new CommandBuffer();
      setup(engine, buffer, 100, 1);
      engine.beginFrame();
      engine.render();
      engine.commitFrame();
      engine.shutdown();
    },
    { iterations: 10, time: 1000 },
  );

  bench(
    "bushy tree depth=3 branching=5 (~125 nodes)",
    () => {
      const engine = createEngine(80, 24);
      const buffer = new CommandBuffer();
      setup(engine, buffer, 3, 5);
      engine.beginFrame();
      engine.render();
      engine.commitFrame();
      engine.shutdown();
    },
    { iterations: 10, time: 1000 },
  );

  bench(
    "bushy tree depth=4 branching=4 (~350 nodes)",
    () => {
      const engine = createEngine(120, 40);
      const buffer = new CommandBuffer();
      setup(engine, buffer, 4, 4);
      engine.beginFrame();
      engine.render();
      engine.commitFrame();
      engine.shutdown();
    },
    { iterations: 5, time: 1500 },
  );

  bench(
    "flat sibling list of 1000 nodes (depth=1, branching=1000)",
    () => {
      const engine = createEngine(200, 50);
      const buffer = new CommandBuffer();
      setup(engine, buffer, 1, 1000);
      engine.beginFrame();
      engine.render();
      engine.commitFrame();
      engine.shutdown();
    },
    { iterations: 3, time: 2000 },
  );
});

describe("Layout — JSON serialisation alone", () => {
  bench(
    "emit + serialise 1000-node layout command stream",
    () => {
      const buffer = new CommandBuffer();
      const counter = { n: 0 };
      const tree = makeTree(1, 1000, counter);
      buffer.push({ type: "BeginFrame", frameId: 1 });
      emitLayoutCommands(buffer, tree);
      buffer.push({ type: "CommitFrame", frameId: 1 });
      const commands = buffer.drain() as Command[];
      // Touch the JSON to prevent dead-code elimination
      const s = JSON.stringify(commands);
      if (s.length === 0) throw new Error("unexpected empty payload");
    },
    { iterations: 20, time: 1000 },
  );
});
