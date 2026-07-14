import { afterEach, describe, expect, it, vi } from "vitest";
import type { Command } from "../command";
import { CommandBuffer } from "../command";
import {
  appendChild,
  createInstance,
  createReconciler,
  createTextInstance,
  finalizeInitialChildren,
  insertBefore,
  prepareUpdate,
  removeChild,
  resetAfterCommit,
} from "../index";
import { CommandRuntime } from "../runtime";

afterEach(() => {
  vi.restoreAllMocks();
});

describe("CommandRuntime + CommandBuffer integration", () => {
  it("publishes commands through subscribe", () => {
    const runtime = new CommandRuntime();
    const received: Command[][] = [];
    runtime.subscribe((cmds) => received.push(cmds));
    runtime.commandBuffer.push({ type: "CreateNode", id: "1", kind: "Box" });
    runtime.commandBuffer.push({ type: "CreateNode", id: "2", kind: "Text" });
    runtime.flush();
    expect(received).toHaveLength(1);
    expect(received[0]).toHaveLength(2);
  });

  it("drains buffer when flush is called", () => {
    const runtime = new CommandRuntime();
    runtime.commandBuffer.push({ type: "Shutdown" });
    expect(runtime.commandBuffer.isEmpty).toBe(false);
    runtime.flush();
    expect(runtime.commandBuffer.isEmpty).toBe(true);
  });

  it("frame loop processes commands and notifies subscribers", async () => {
    vi.useFakeTimers();
    const runtime = new CommandRuntime({ frameIntervalMs: 50 });
    const received: Command[][] = [];
    runtime.subscribe((cmds) => received.push(cmds));
    runtime.startFrameLoop();
    runtime.commandBuffer.push({ type: "Shutdown" });
    vi.advanceTimersByTime(100);
    runtime.stopFrameLoop();
    expect(received.length).toBeGreaterThan(0);
    vi.useRealTimers();
  });

  it("multiple subscribers all receive commands", () => {
    const runtime = new CommandRuntime();
    const received1: Command[][] = [];
    const received2: Command[][] = [];
    runtime.subscribe((cmds) => received1.push(cmds));
    runtime.subscribe((cmds) => received2.push(cmds));
    runtime.commandBuffer.push({ type: "Shutdown" });
    runtime.flush();
    expect(received1).toHaveLength(1);
    expect(received2).toHaveLength(1);
  });

  it("unsubscribe removes only that subscriber", () => {
    const runtime = new CommandRuntime();
    const received1: Command[][] = [];
    const received2: Command[][] = [];
    const unsub1 = runtime.subscribe((cmds) => received1.push(cmds));
    runtime.subscribe((cmds) => received2.push(cmds));
    unsub1();
    runtime.commandBuffer.push({ type: "Shutdown" });
    runtime.flush();
    expect(received1).toHaveLength(0);
    expect(received2).toHaveLength(1);
  });

  it("dispose clears buffer, subscribers, and frame callbacks", () => {
    const runtime = new CommandRuntime();
    runtime.commandBuffer.push({ type: "CreateNode", id: "1", kind: "Box" });
    runtime.subscribe(() => {});
    runtime.onFrame(() => {});
    runtime.dispose();
    expect(runtime.commandBuffer.isEmpty).toBe(true);
  });
});

describe("Reconciler + CommandBuffer integration", () => {
  it("emits CreateNode and SetStyle for styled instances", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    reconciler.createInstance("Box", { style: { bold: true, fg: "red" } });
    const commands = buffer.drain();
    expect(commands.find((c) => c.type === "CreateNode")).toBeDefined();
    expect(commands.find((c) => c.type === "SetStyle")).toBeDefined();
  });

  it("emits CreateNode and SetText for text instances", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    reconciler.createTextInstance("hello");
    const commands = buffer.drain();
    expect(commands.find((c) => c.type === "CreateNode")).toBeDefined();
    expect(commands.find((c) => c.type === "SetText")).toBeDefined();
  });

  it("maintains correct parent-child relationships after appendChild", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const parent = reconciler.createInstance("Box", {});
    const child = reconciler.createInstance("Text", {});
    buffer.clear();
    reconciler.appendChild(parent, child);
    expect(child.parent).toBe(parent);
    expect(parent.children).toContain(child);
    const commands = buffer.drain();
    expect(commands.some((c) => c.type === "AppendChild")).toBe(true);
  });

  it("insertBefore reorders children correctly", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const parent = reconciler.createInstance("Box", {});
    const child1 = reconciler.createInstance("Text", {});
    const child2 = reconciler.createInstance("Text", {});
    reconciler.appendChild(parent, child1);
    buffer.clear();
    reconciler.insertBefore(parent, child2, child1);
    expect(parent.children[0]).toBe(child2);
    expect(parent.children[1]).toBe(child1);
  });

  it("removeChild removes from parent and clears parent ref", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const parent = reconciler.createInstance("Box", {});
    const child = reconciler.createInstance("Text", {});
    reconciler.appendChild(parent, child);
    buffer.clear();
    reconciler.removeChild(parent, child);
    expect(child.parent).toBeNull();
    expect(parent.children).not.toContain(child);
  });

  it("commitUpdate with style emits SetStyle and merges props", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const instance = reconciler.createInstance("Box", {});
    buffer.clear();
    reconciler.commitUpdate(instance, { style: { bold: true }, padding: 2 });
    const commands = buffer.drain();
    expect(commands.some((c) => c.type === "SetStyle")).toBe(true);
    expect(instance.props["padding"]).toBe(2);
  });

  it("commitUpdate without style does not emit SetStyle", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const instance = reconciler.createInstance("Box", {});
    buffer.clear();
    reconciler.commitUpdate(instance, { padding: 2 });
    const commands = buffer.drain();
    expect(commands.some((c) => c.type === "SetStyle")).toBe(false);
  });

  it("commitTextUpdate updates text and emits SetText", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const text = reconciler.createTextInstance("hello");
    buffer.clear();
    reconciler.commitTextUpdate(text, "world");
    const commands = buffer.drain();
    expect(commands.some((c) => c.type === "SetText")).toBe(true);
    expect(text.text).toBe("world");
  });

  it("commitTextUpdate does not emit SetText if text instance has no id", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const text = createTextInstance("hello");
    buffer.clear();
    reconciler.commitTextUpdate(text, "world");
    const commands = buffer.drain();
    expect(commands.some((c) => c.type === "SetText")).toBe(false);
  });
});

describe("Full tree construction integration", () => {
  it("builds a multi-level tree with commands at each level", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);

    const root = reconciler.createInstance("Box", { style: { bold: true } });
    const child1 = reconciler.createInstance("Text", { padding: 1 });
    const child2 = reconciler.createInstance("Text", {});
    const text1 = reconciler.createTextInstance("Hello");
    const text2 = reconciler.createTextInstance("World");

    reconciler.appendChild(root, child1);
    reconciler.appendChild(child1, text1);
    reconciler.appendChild(root, child2);
    reconciler.appendChild(child2, text2);

    const commands = buffer.drain();
    expect(commands.filter((c) => c.type === "CreateNode")).toHaveLength(5);
    expect(commands.filter((c) => c.type === "AppendChild")).toHaveLength(4);
    expect(commands.filter((c) => c.type === "SetText")).toHaveLength(2);
    expect(commands.filter((c) => c.type === "SetStyle")).toHaveLength(1);

    expect(root.children).toHaveLength(2);
    expect(child1.children).toHaveLength(1);
    expect(child2.children).toHaveLength(1);
    expect(text1.parent).toBe(child1);
    expect(text2.parent).toBe(child2);
  });

  it("handles insertBefore within a full tree", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);

    const parent = reconciler.createInstance("Box", {});
    const first = reconciler.createInstance("Text", {});
    const last = reconciler.createInstance("Text", {});
    reconciler.appendChild(parent, first);
    reconciler.appendChild(parent, last);
    buffer.clear();

    const middle = reconciler.createInstance("Text", {});
    reconciler.insertBefore(parent, middle, last);
    expect(parent.children[0]).toBe(first);
    expect(parent.children[1]).toBe(middle);
    expect(parent.children[2]).toBe(last);
  });

  it("handles removeChild within a full tree", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);

    const parent = reconciler.createInstance("Box", {});
    const child = reconciler.createInstance("Text", {});
    reconciler.appendChild(parent, child);
    buffer.clear();

    reconciler.removeChild(parent, child);
    expect(parent.children).toHaveLength(0);
    expect(child.parent).toBeNull();
  });
});

describe("CommandRuntime lifecycle with Reconciler", () => {
  it("processes commands through runtime flush", () => {
    const runtime = new CommandRuntime();
    const buffer = runtime.commandBuffer;
    const reconciler = createReconciler(buffer);

    reconciler.createInstance("Box", { style: { bold: true } });
    expect(buffer.isEmpty).toBe(false);

    const received: Command[][] = [];
    runtime.subscribe((cmds) => received.push(cmds));
    runtime.flush();

    expect(buffer.isEmpty).toBe(true);
    expect(received.length).toBeGreaterThan(0);
  });

  it("runtime frame loop drives reconciler operations", async () => {
    vi.useFakeTimers();
    const runtime = new CommandRuntime({ frameIntervalMs: 50 });
    const buffer = runtime.commandBuffer;
    const reconciler = createReconciler(buffer);

    const received: Command[][] = [];
    runtime.subscribe((cmds) => received.push(cmds));
    runtime.startFrameLoop();

    reconciler.createInstance("Box", {});
    reconciler.createTextInstance("testing");

    vi.advanceTimersByTime(100);
    runtime.stopFrameLoop();

    expect(received.length).toBeGreaterThan(0);
    const allCmds = received.flat();
    expect(allCmds.some((c) => c.type === "CreateNode")).toBe(true);
    expect(allCmds.some((c) => c.type === "SetText")).toBe(true);
    vi.useRealTimers();
  });
});

describe("CommandBuffer edge cases", () => {
  it("handles large number of commands", () => {
    const buffer = new CommandBuffer();
    const count = 10000;
    for (let i = 0; i < count; i++) {
      buffer.push({ type: "CreateNode", id: String(i), kind: "Box" });
    }
    expect(buffer.length).toBe(count);
    const drained = buffer.drain();
    expect(drained).toHaveLength(count);
  });

  it("drain returns a copy and clears the buffer", () => {
    const buffer = new CommandBuffer();
    buffer.push({ type: "Shutdown" });
    const first = buffer.drain();
    const second = buffer.drain();
    expect(first).toHaveLength(1);
    expect(second).toHaveLength(0);
  });

  it("works with all command types", () => {
    const buffer = new CommandBuffer();
    const commands: Command[] = [
      { type: "CreateNode", id: "1", kind: "Box" },
      { type: "RemoveNode", id: "1" },
      { type: "AppendChild", parent: "1", child: "2" },
      { type: "InsertBefore", reference: "1", child: "2" },
      { type: "MoveNode", node: "1", newParent: "2" },
      { type: "ReplaceNode", old: "1", new: "2" },
      { type: "DetachNode", id: "1" },
      { type: "SetText", id: "1", text: "hello" },
      { type: "SetStyle", id: "1", style: { bold: true } },
      { type: "SetLayout", id: "1", layout: { width: 100 } },
      { type: "SetAttribute", id: "1", key: "key", value: "val" },
      { type: "RemoveAttribute", id: "1", key: "key" },
      { type: "BeginFrame", frameId: 1 },
      { type: "CommitFrame", frameId: 1 },
      { type: "Invalidate", id: "1" },
      { type: "Shutdown" },
    ];
    for (const cmd of commands) {
      buffer.push(cmd);
    }
    expect(buffer.length).toBe(16);
    const drained = buffer.drain();
    expect(drained).toHaveLength(16);
  });
});

describe("CommandRuntime edge cases", () => {
  it("flush with no subscribers does not throw", () => {
    const runtime = new CommandRuntime();
    runtime.commandBuffer.push({ type: "Shutdown" });
    expect(() => runtime.flush()).not.toThrow();
  });

  it("onFrame callback can be called multiple times", () => {
    const runtime = new CommandRuntime();
    const fn = vi.fn();
    const unsub1 = runtime.onFrame(fn);
    const unsub2 = runtime.onFrame(fn);
    expect(typeof unsub1).toBe("function");
    expect(typeof unsub2).toBe("function");
    unsub1();
    unsub2();
  });

  it("subscribe with multiple registrations works", () => {
    const runtime = new CommandRuntime();
    const fn = vi.fn();
    runtime.subscribe(fn);
    runtime.subscribe(fn);
    runtime.commandBuffer.push({ type: "Shutdown" });
    runtime.flush();
    expect(fn).toHaveBeenCalledTimes(2);
  });

  it("drain works after dispose returns empty", () => {
    const runtime = new CommandRuntime();
    runtime.commandBuffer.push({ type: "Shutdown" });
    runtime.dispose();
    const drained = runtime.drain();
    expect(drained).toHaveLength(0);
  });

  it("startFrameLoop uses default interval when not specified", () => {
    const runtime = new CommandRuntime();
    runtime.startFrameLoop();
    expect(runtime.isRunning).toBe(true);
    runtime.stopFrameLoop();
  });

  it("startFrameLoop with custom interval stores it", () => {
    const runtime = new CommandRuntime();
    runtime.startFrameLoop(100);
    expect(runtime.isRunning).toBe(true);
    runtime.stopFrameLoop();
  });
});

describe("Direct tree operations edge cases", () => {
  it("appendChild with text instance to non-Instance fails silently", () => {
    const parent = createInstance("Box", {});
    const text = createTextInstance("hello");
    appendChild(parent, text);
    expect(text.parent).toBe(parent);
  });

  it("removeChild on empty children does not error", () => {
    const parent = createInstance("Box", {});
    const child = createInstance("Text", {});
    expect(() => removeChild(parent, child)).not.toThrow();
  });

  it("insertBefore at end when reference not found", () => {
    const parent = createInstance("Box", {});
    const child = createInstance("Text", {});
    const ref = createInstance("Text", {});
    insertBefore(parent, child, ref);
    expect(parent.children).toContain(child);
  });

  it("prepareUpdate returns empty payload when no meaningful changes", () => {
    const instance = createInstance("Box", {});
    const result = prepareUpdate(instance, "Box", {}, {});
    expect(result).toEqual({});
  });

  it("finalizeInitialChildren always returns false", () => {
    const instance = createInstance("Box", {});
    expect(finalizeInitialChildren(instance)).toBe(false);
  });

  it("resetAfterCommit is a no-op", () => {
    expect(() => resetAfterCommit()).not.toThrow();
  });
});
