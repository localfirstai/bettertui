/**
 * reconciler.bench.ts (extended)
 *
 * Scaled-up version of `packages/benchmark/src/reconciler.bench.ts` (115 LoC).
 * The sibling package measures createReconciler at the 100-node scale; this
 * file pushes the same harness to 1k / 10k nodes and adds tree-mutation
 * patterns (move, remove, re-insert) that the existing micro-bench doesn't
 * cover.
 *
 * Reference: the layout + render-traversal benchmarks
 * (`.opencode/references/opentui/packages/core/src/benchmark/layout-benchmark.ts` and
 * `render-traversal-benchmark.ts`)
 * operate against the Zig-side renderer directly; the BetterTUI equivalent
 * routes through `createReconciler` + `CommandBuffer`, which is the
 * abstraction level framework adapters will use.
 *
 * Preconditions: none (pure-TS).
 */

import { CommandBuffer, type Instance, type TextInstance, createReconciler } from "@bettertui/core";
import { bench, describe } from "vitest";

function makeChildren(n: number): Instance[] {
  const buffer = new CommandBuffer();
  const reconciler = createReconciler(buffer);
  const out: Instance[] = [];
  for (let i = 0; i < n; i++) {
    out.push(reconciler.createInstance("Box", { id: i }));
  }
  return out;
}

describe("createReconciler — scaled mount", () => {
  bench(
    "create + append 1k children",
    () => {
      const buffer = new CommandBuffer();
      const reconciler = createReconciler(buffer);
      const root = reconciler.createInstance("Box", {});
      for (let i = 0; i < 1000; i++) {
        const child = reconciler.createInstance("Text", { id: i });
        reconciler.appendChild(root, child);
      }
    },
    { iterations: 20, time: 1500 },
  );

  bench(
    "create + append 10k children",
    () => {
      const buffer = new CommandBuffer();
      const reconciler = createReconciler(buffer);
      const root = reconciler.createInstance("Box", {});
      for (let i = 0; i < 10_000; i++) {
        const child = reconciler.createInstance("Text", { id: i });
        reconciler.appendChild(root, child);
      }
    },
    { iterations: 5, time: 2500 },
  );

  bench(
    "create 1k text instances",
    () => {
      const buffer = new CommandBuffer();
      const reconciler = createReconciler(buffer);
      for (let i = 0; i < 1000; i++) {
        reconciler.createTextInstance(`text-${i}`);
      }
    },
    { iterations: 20, time: 1500 },
  );
});

describe("createReconciler — mutation patterns", () => {
  bench(
    "remove 50% of 1k children",
    () => {
      const buffer = new CommandBuffer();
      const reconciler = createReconciler(buffer);
      const root = reconciler.createInstance("Box", {});
      const children: Instance[] = [];
      for (let i = 0; i < 1000; i++) {
        const child = reconciler.createInstance("Text", { id: i });
        reconciler.appendChild(root, child);
        children.push(child);
      }
      buffer.clear();
      // remove every other child
      for (let i = 0; i < children.length; i += 2) {
        reconciler.removeChild(root, children[i]);
      }
    },
    { iterations: 10, time: 2000 },
  );

  bench(
    "commitUpdate on 1k instances",
    () => {
      const buffer = new CommandBuffer();
      const reconciler = createReconciler(buffer);
      const children = makeChildren(1000);
      buffer.clear();
      for (const child of children) {
        reconciler.commitUpdate(child, { updated: true });
      }
    },
    { iterations: 10, time: 2000 },
  );

  bench(
    "insertBefore interleaved on 500 children",
    () => {
      const buffer = new CommandBuffer();
      const reconciler = createReconciler(buffer);
      const root = reconciler.createInstance("Box", {});
      const children: Instance[] = [];
      for (let i = 0; i < 500; i++) {
        const child = reconciler.createInstance("Text", { id: i });
        reconciler.appendChild(root, child);
        children.push(child);
      }
      buffer.clear();
      for (let i = 0; i < children.length - 1; i += 2) {
        const newNode = reconciler.createInstance("Text", { id: `ins-${i}` });
        reconciler.insertBefore(root, newNode, children[i + 1]);
      }
    },
    { iterations: 10, time: 2000 },
  );

  bench(
    "commitTextUpdate on 1k text instances",
    () => {
      const buffer = new CommandBuffer();
      const reconciler = createReconciler(buffer);
      const texts: TextInstance[] = [];
      for (let i = 0; i < 1000; i++) {
        texts.push(reconciler.createTextInstance(`t-${i}`));
      }
      buffer.clear();
      for (const t of texts) {
        reconciler.commitTextUpdate(t, "updated");
      }
    },
    { iterations: 10, time: 2000 },
  );
});

describe("createReconciler — drain cost", () => {
  bench(
    "drain 10k-command buffer",
    () => {
      const buffer = new CommandBuffer();
      const reconciler = createReconciler(buffer);
      const root = reconciler.createInstance("Box", {});
      for (let i = 0; i < 10_000; i++) {
        const child = reconciler.createInstance("Text", { id: i });
        reconciler.appendChild(root, child);
      }
      const drained = buffer.drain();
      if (drained.length === 0) throw new Error("drain returned empty");
    },
    { iterations: 5, time: 2500 },
  );
});
