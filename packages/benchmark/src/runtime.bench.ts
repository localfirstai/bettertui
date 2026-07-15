import {
  CommandBuffer,
  CommandRuntime,
  appendChild,
  commitTextUpdate,
  commitUpdate,
  createInstance,
  createTextInstance,
  insertBefore,
  removeChild,
} from "@bettertui/core";
import { bench, describe } from "vitest";

describe("CommandRuntime", () => {
  bench("create Runtime", () => {
    new CommandRuntime();
  });

  bench("create Runtime with custom buffer", () => {
    new CommandRuntime(new CommandBuffer());
  });

  bench("subscribe and flush", () => {
    const runtime = new CommandRuntime();
    const unsub = runtime.subscribe(() => {});
    runtime.commandBuffer.push({ type: "Shutdown" });
    runtime.flush();
    unsub();
  });

  bench("subscribe, push 100, flush", () => {
    const runtime = new CommandRuntime();
    const unsub = runtime.subscribe(() => {});
    for (let i = 0; i < 100; i++) {
      runtime.commandBuffer.push({
        type: "CreateNode",
        id: `node-${i}`,
        kind: "Box",
      });
    }
    runtime.flush();
    unsub();
  });

  bench("multiple subscribers", () => {
    const runtime = new CommandRuntime();
    const unsubs = Array.from({ length: 10 }, () => runtime.subscribe(() => {}));
    runtime.commandBuffer.push({ type: "Shutdown" });
    runtime.flush();
    for (const unsub of unsubs) unsub();
  });

  bench("dispose", () => {
    const runtime = new CommandRuntime();
    runtime.dispose();
  });
});

describe("tree operations", () => {
  bench("createInstance", () => {
    createInstance("Box", { padding: 1, style: { bold: true } });
  });

  bench("createTextInstance", () => {
    createTextInstance("hello world");
  });

  bench("appendChild", () => {
    const parent = createInstance("Box", {});
    const child = createInstance("Text", {});
    appendChild(parent, child);
  });

  bench("removeChild", () => {
    const parent = createInstance("Box", {});
    const child = createInstance("Text", {});
    appendChild(parent, child);
    removeChild(parent, child);
  });

  bench("insertBefore", () => {
    const parent = createInstance("Box", {});
    const child1 = createInstance("Text", { id: "1" });
    const child2 = createInstance("Text", { id: "2" });
    appendChild(parent, child1);
    insertBefore(parent, child2, child1);
  });

  bench("commitUpdate", () => {
    const instance = createInstance("Box", { padding: 1, margin: 0 });
    commitUpdate(instance, { padding: 2, margin: 3 });
  });

  bench("commitTextUpdate", () => {
    const text = createTextInstance("hello");
    commitTextUpdate(text, "world");
  });

  bench("deep tree (10 levels)", () => {
    let root = createInstance("Box", {});
    for (let i = 0; i < 10; i++) {
      const child = createInstance("Box", { depth: i });
      appendChild(root, child);
      root = child;
    }
  });

  bench("wide tree (100 children)", () => {
    const parent = createInstance("Box", {});
    for (let i = 0; i < 100; i++) {
      const child = createInstance("Text", { id: `child-${i}` });
      appendChild(parent, child);
    }
  });
});
