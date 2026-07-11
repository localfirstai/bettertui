import { CommandBuffer, createReconciler } from "@bettertui/core";
import { bench, describe } from "vitest";

describe("createReconciler", () => {
  bench("create reconciler", () => {
    const buffer = new CommandBuffer();
    createReconciler(buffer);
  });

  bench("reconciler: createInstance", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    reconciler.createInstance("Box", { padding: 1 });
  });

  bench("reconciler: createTextInstance", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    reconciler.createTextInstance("hello");
  });

  bench("reconciler: appendChild", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const parent = reconciler.createInstance("Box", {});
    const child = reconciler.createInstance("Text", {});
    buffer.clear();
    reconciler.appendChild(parent, child);
  });

  bench("reconciler: removeChild", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const parent = reconciler.createInstance("Box", {});
    const child = reconciler.createInstance("Text", {});
    reconciler.appendChild(parent, child);
    buffer.clear();
    reconciler.removeChild(parent, child);
  });

  bench("reconciler: prepareUpdate", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const instance = reconciler.createInstance("Box", { padding: 1 });
    reconciler.prepareUpdate(instance, "Box", { padding: 1 }, { padding: 2 });
  });

  bench("reconciler: commitUpdate", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const instance = reconciler.createInstance("Box", { padding: 1 });
    buffer.clear();
    reconciler.commitUpdate(instance, { padding: 2 });
  });

  bench("reconciler: commitTextUpdate", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const text = reconciler.createTextInstance("hello");
    buffer.clear();
    reconciler.commitTextUpdate(text, "world");
  });

  bench("reconciler: resetAfterCommit", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    reconciler.resetAfterCommit();
  });
});

describe("reconciler: batch operations", () => {
  bench("create 100 nodes", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    for (let i = 0; i < 100; i++) {
      reconciler.createInstance("Box", { id: i });
    }
  });

  bench("create 100 text nodes", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    for (let i = 0; i < 100; i++) {
      reconciler.createTextInstance(`text-${i}`);
    }
  });

  bench("create tree and attach children", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const root = reconciler.createInstance("Box", {});
    for (let i = 0; i < 50; i++) {
      const child = reconciler.createInstance("Text", { id: i });
      reconciler.appendChild(root, child);
    }
  });

  bench("full lifecycle: create, attach, update, remove", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const root = reconciler.createInstance("Box", {});
    const children = [];
    for (let i = 0; i < 20; i++) {
      const child = reconciler.createInstance("Text", { id: i });
      reconciler.appendChild(root, child);
      children.push(child);
    }
    for (const child of children) {
      reconciler.commitUpdate(child, { updated: true });
    }
    for (const child of children) {
      reconciler.removeChild(root, child);
    }
  });
});
