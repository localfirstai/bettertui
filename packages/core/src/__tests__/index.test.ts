import { afterEach, describe, expect, it, vi } from "vitest";
import type { Command } from "../command";
import {
  CommandBuffer,
  CommandRuntime,
  appendChild,
  commitTextUpdate,
  commitUpdate,
  createInstance,
  createReconciler,
  createTextInstance,
  finalizeInitialChildren,
  generateId,
  insertBefore,
  prepareUpdate,
  removeChild,
  resetAfterCommit,
} from "../index";

afterEach(() => {
  vi.restoreAllMocks();
});

describe("CommandBuffer", () => {
  it("starts empty", () => {
    const buffer = new CommandBuffer();
    expect(buffer.length).toBe(0);
    expect(buffer.isEmpty).toBe(true);
    expect(buffer.drain()).toEqual([]);
  });

  it("pushes and drains commands", () => {
    const buffer = new CommandBuffer();
    buffer.push({ type: "Shutdown" });
    expect(buffer.length).toBe(1);
    expect(buffer.isEmpty).toBe(false);
    const commands = buffer.drain();
    expect(commands).toHaveLength(1);
    expect(commands[0]).toEqual({ type: "Shutdown" });
    expect(buffer.length).toBe(0);
    expect(buffer.isEmpty).toBe(true);
  });

  it("peek returns commands without removing", () => {
    const buffer = new CommandBuffer();
    buffer.push({ type: "Shutdown" });
    const peeked = buffer.peek();
    expect(peeked).toHaveLength(1);
    expect(buffer.length).toBe(1);
  });

  it("clear removes all commands", () => {
    const buffer = new CommandBuffer();
    buffer.push({ type: "Shutdown" });
    buffer.push({ type: "Shutdown" });
    buffer.clear();
    expect(buffer.isEmpty).toBe(true);
  });

  it("handles multiple command types", () => {
    const buffer = new CommandBuffer();
    buffer.push({ type: "CreateNode", id: "1", kind: "Box" });
    buffer.push({ type: "AppendChild", parent: "1", child: "2" });
    buffer.push({ type: "SetStyle", id: "1", style: { bold: true } });
    expect(buffer.length).toBe(3);
    const commands = buffer.drain();
    expect(commands[0]?.type).toBe("CreateNode");
    expect(commands[1]?.type).toBe("AppendChild");
    expect(commands[2]?.type).toBe("SetStyle");
  });
});

describe("generateId", () => {
  it("generates unique ids", () => {
    const id1 = generateId();
    const id2 = generateId();
    expect(id1).not.toBe(id2);
  });

  it("generates string ids", () => {
    const id = generateId();
    expect(typeof id).toBe("string");
  });
});

describe("tree operations", () => {
  it("createInstance creates an instance with props", () => {
    const instance = createInstance("Box", { padding: 1, style: { bold: true } });
    expect(instance.type).toBe("Box");
    expect(instance.props).toEqual({ padding: 1 });
    expect(instance.style).toEqual({ bold: true });
    expect(instance.children).toEqual([]);
    expect(instance.parent).toBeNull();
  });

  it("createInstance handles empty props", () => {
    const instance = createInstance("Box", {});
    expect(instance.type).toBe("Box");
    expect(instance.props).toEqual({});
    expect(instance.style).toEqual({});
    expect(instance.children).toEqual([]);
  });

  it("createTextInstance creates a text instance", () => {
    const text = createTextInstance("hello");
    expect(text.type).toBe("#text");
    expect(text.text).toBe("hello");
    expect(text.parent).toBeNull();
  });

  it("appendChild sets parent and adds child", () => {
    const parent = createInstance("Box", {});
    const child = createInstance("Text", {});
    appendChild(parent, child);
    expect(child.parent).toBe(parent);
    expect(parent.children).toContain(child);
  });

  it("appendChild with text instance sets parent", () => {
    const parent = createInstance("Box", {});
    const text = createTextInstance("hello");
    appendChild(parent, text);
    expect(text.parent).toBe(parent);
  });

  it("removeChild removes child and clears parent", () => {
    const parent = createInstance("Box", {});
    const child = createInstance("Text", {});
    appendChild(parent, child);
    removeChild(parent, child);
    expect(child.parent).toBeNull();
    expect(parent.children).not.toContain(child);
  });

  it("removeChild when child not in children does nothing", () => {
    const parent = createInstance("Box", {});
    const child = createInstance("Text", {});
    const originalLength = parent.children.length;
    removeChild(parent, child);
    expect(parent.children.length).toBe(originalLength);
    expect(child.parent).toBeNull();
  });

  it("insertBefore inserts at correct position", () => {
    const parent = createInstance("Box", {});
    const child1 = createInstance("Text", { id: "1" });
    const child2 = createInstance("Text", { id: "2" });
    appendChild(parent, child1);
    insertBefore(parent, child2, child1);
    expect(parent.children[0]).toBe(child2);
    expect(parent.children[1]).toBe(child1);
  });

  it("insertBefore appends when reference not found", () => {
    const parent = createInstance("Box", {});
    const child1 = createInstance("Text", { id: "1" });
    const child2 = createInstance("Text", { id: "2" });
    const child3 = createInstance("Text", { id: "3" });
    appendChild(parent, child1);
    insertBefore(parent, child2, child3);
    expect(parent.children).toContain(child2);
    expect(parent.children.length).toBe(2);
  });

  it("insertBefore with text instance", () => {
    const parent = createInstance("Box", {});
    const child1 = createInstance("Text", {});
    const text = createTextInstance("hello");
    appendChild(parent, child1);
    insertBefore(parent, text, child1);
    expect(parent.children[0]).toBe(text);
    expect(text.parent).toBe(parent);
  });

  it("commitUpdate merges payload into instance props", () => {
    const instance = createInstance("Box", { padding: 1 });
    commitUpdate(instance, { padding: 2, margin: 3 });
    expect(instance.props["padding"]).toBe(2);
    expect(instance.props["margin"]).toBe(3);
  });

  it("commitTextUpdate updates text", () => {
    const text = createTextInstance("hello");
    commitTextUpdate(text, "world");
    expect(text.text).toBe("world");
  });

  it("finalizeInitialChildren returns false", () => {
    const instance = createInstance("Box", {});
    expect(finalizeInitialChildren(instance)).toBe(false);
  });

  it("resetAfterCommit is a no-op", () => {
    expect(() => resetAfterCommit()).not.toThrow();
  });

  it("prepareUpdate filters children/style/layout from payload", () => {
    const instance = createInstance("Box", {});
    const result = prepareUpdate(
      instance,
      "Box",
      {},
      {
        children: [],
        style: { bold: true },
        layout: { width: 100 },
        customProp: "value",
      },
    );
    expect(result).toEqual({ customProp: "value" });
  });

  it("prepareUpdate returns empty payload when only children/style/layout", () => {
    const instance = createInstance("Box", {});
    const result = prepareUpdate(
      instance,
      "Box",
      {},
      {
        children: [],
        style: {},
        layout: {},
      },
    );
    expect(result).toEqual({});
  });
});

describe("CommandRuntime", () => {
  it("creates with default buffer", () => {
    const runtime = new CommandRuntime();
    expect(runtime.commandBuffer).toBeInstanceOf(CommandBuffer);
    expect(runtime.isRunning).toBe(false);
  });

  it("creates with custom buffer", () => {
    const buffer = new CommandBuffer();
    const runtime = new CommandRuntime(buffer);
    expect(runtime.commandBuffer).toBe(buffer);
  });

  it("creates with RuntimeOptions frameIntervalMs", () => {
    const runtime = new CommandRuntime({ frameIntervalMs: 50 });
    expect(runtime.isRunning).toBe(false);
    expect(runtime.commandBuffer).toBeInstanceOf(CommandBuffer);
  });

  it("creates with autoStart starts frame loop", () => {
    const runtime = new CommandRuntime({ autoStart: true });
    expect(runtime.isRunning).toBe(true);
    runtime.stopFrameLoop();
  });

  it("drain delegates to buffer", () => {
    const runtime = new CommandRuntime();
    runtime.commandBuffer.push({ type: "Shutdown" });
    const commands = runtime.drain();
    expect(commands).toHaveLength(1);
  });

  it("flush with no commands does not call subscribers", () => {
    const runtime = new CommandRuntime();
    const fn = vi.fn();
    runtime.subscribe(fn);
    runtime.flush();
    expect(fn).not.toHaveBeenCalled();
  });

  it("subscribe and flush sends commands", () => {
    const runtime = new CommandRuntime();
    const received: unknown[][] = [];
    runtime.subscribe((cmds) => received.push(cmds));
    runtime.commandBuffer.push({ type: "Shutdown" });
    runtime.flush();
    expect(received).toHaveLength(1);
    expect(received[0]).toHaveLength(1);
  });

  it("unsubscribe stops receiving", () => {
    const runtime = new CommandRuntime();
    const received: unknown[][] = [];
    const unsub = runtime.subscribe((cmds) => received.push(cmds));
    runtime.commandBuffer.push({ type: "Shutdown" });
    runtime.flush();
    expect(received).toHaveLength(1);
    unsub();
    runtime.commandBuffer.push({ type: "Shutdown" });
    runtime.flush();
    expect(received).toHaveLength(1);
  });

  it("dispose cleans up", () => {
    const runtime = new CommandRuntime();
    runtime.commandBuffer.push({ type: "Shutdown" });
    runtime.dispose();
    expect(runtime.commandBuffer.isEmpty).toBe(true);
  });

  it("dispose clears subscribers and frame callbacks", () => {
    const runtime = new CommandRuntime();
    const subFn = vi.fn();
    const frameFn = vi.fn();
    runtime.subscribe(subFn);
    runtime.onFrame(frameFn);
    runtime.dispose();
    runtime.commandBuffer.push({ type: "Shutdown" });
    runtime.flush();
    expect(subFn).not.toHaveBeenCalled();
  });

  it("startFrameLoop and stopFrameLoop", () => {
    const runtime = new CommandRuntime();
    runtime.startFrameLoop(100);
    expect(runtime.isRunning).toBe(true);
    runtime.stopFrameLoop();
    expect(runtime.isRunning).toBe(false);
  });

  it("double startFrameLoop is no-op", () => {
    const runtime = new CommandRuntime();
    runtime.startFrameLoop(100);
    runtime.startFrameLoop(50);
    expect(runtime.isRunning).toBe(true);
    runtime.stopFrameLoop();
  });

  it("stopFrameLoop when not running is no-op", () => {
    const runtime = new CommandRuntime();
    expect(() => runtime.stopFrameLoop()).not.toThrow();
  });

  it("onFrame registers and unsubscribes callback", () => {
    const runtime = new CommandRuntime();
    const unsub = runtime.onFrame(() => {});
    expect(typeof unsub).toBe("function");
    unsub();
  });

  it("requestFrame when not running is no-op", () => {
    const runtime = new CommandRuntime();
    const fn = vi.fn();
    runtime.subscribe(fn);
    runtime.requestFrame();
    expect(fn).not.toHaveBeenCalled();
  });

  it("requestFrame when running flushes commands", () => {
    const runtime = new CommandRuntime();
    runtime.startFrameLoop(1000);
    const fn = vi.fn();
    runtime.subscribe(fn);
    runtime.commandBuffer.push({ type: "Shutdown" });
    runtime.requestFrame();
    expect(fn).toHaveBeenCalledWith([{ type: "Shutdown" }]);
    runtime.stopFrameLoop();
  });

  it("onFrame callback receives delta during frame loop", async () => {
    const runtime = new CommandRuntime();
    const deltas: number[] = [];
    runtime.onFrame((delta) => deltas.push(delta));
    runtime.startFrameLoop(10);
    await new Promise((resolve) => setTimeout(resolve, 30));
    runtime.stopFrameLoop();
    expect(deltas.length).toBeGreaterThan(0);
    for (const d of deltas) {
      expect(d).toBeGreaterThan(0);
    }
  });

  it("frame loop calls flush", async () => {
    const runtime = new CommandRuntime();
    const fn = vi.fn();
    runtime.subscribe(fn);
    runtime.startFrameLoop(10);
    runtime.commandBuffer.push({ type: "Shutdown" });
    await new Promise((resolve) => setTimeout(resolve, 25));
    runtime.stopFrameLoop();
    expect(fn).toHaveBeenCalled();
  });

  it("onFrame unsubscribe removes callback", () => {
    const runtime = new CommandRuntime();
    const fn = vi.fn();
    const unsub = runtime.onFrame(fn);
    unsub();
    runtime.startFrameLoop(10);
    runtime.stopFrameLoop();
  });

  it("tick exits early when frame loop is stopped", async () => {
    const runtime = new CommandRuntime();
    const fn = vi.fn();
    runtime.onFrame(fn);
    runtime.startFrameLoop(10);
    runtime.stopFrameLoop();
    await new Promise((resolve) => setTimeout(resolve, 25));
    const callsAfterStop = fn.mock.calls.length;
    await new Promise((resolve) => setTimeout(resolve, 30));
    expect(fn.mock.calls.length).toBe(callsAfterStop);
  });

  it("drains and flushes frame loop during tick", async () => {
    const runtime = new CommandRuntime();
    const commands: Command[][] = [];
    runtime.subscribe((cmds) => commands.push(cmds));
    runtime.startFrameLoop(10);
    runtime.commandBuffer.push({ type: "Shutdown" });
    await new Promise((resolve) => setTimeout(resolve, 25));
    runtime.stopFrameLoop();
    expect(commands.length).toBeGreaterThan(0);
  });
});

describe("createReconciler", () => {
  it("creates a reconciler with all methods", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    expect(typeof reconciler.createInstance).toBe("function");
    expect(typeof reconciler.createTextInstance).toBe("function");
    expect(typeof reconciler.appendChild).toBe("function");
    expect(typeof reconciler.removeChild).toBe("function");
    expect(typeof reconciler.insertBefore).toBe("function");
    expect(typeof reconciler.prepareUpdate).toBe("function");
    expect(typeof reconciler.commitUpdate).toBe("function");
    expect(typeof reconciler.commitTextUpdate).toBe("function");
    expect(typeof reconciler.finalizeInitialChildren).toBe("function");
    expect(typeof reconciler.resetAfterCommit).toBe("function");
  });

  it("createInstance emits CreateNode command", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const instance = reconciler.createInstance("Box", { style: { bold: true } });
    const commands = buffer.drain();
    expect(commands.some((c) => c.type === "CreateNode")).toBe(true);
    expect(instance.type).toBe("Box");
  });

  it("createInstance without style does not emit SetStyle", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    reconciler.createInstance("Box", {});
    const commands = buffer.drain();
    expect(commands.some((c) => c.type === "SetStyle")).toBe(false);
  });

  it("createInstance with style emits SetStyle", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    reconciler.createInstance("Box", { style: { bold: true } });
    const commands = buffer.drain();
    expect(commands.some((c) => c.type === "SetStyle")).toBe(true);
  });

  it("appendChild emits AppendChild command", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const parent = reconciler.createInstance("Box", {});
    const child = reconciler.createInstance("Text", {});
    buffer.clear();
    reconciler.appendChild(parent, child);
    const commands = buffer.drain();
    expect(commands.some((c) => c.type === "AppendChild")).toBe(true);
  });

  it("removeChild emits RemoveNode command", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const parent = reconciler.createInstance("Box", {});
    const child = reconciler.createInstance("Text", {});
    reconciler.appendChild(parent, child);
    buffer.clear();
    reconciler.removeChild(parent, child);
    const commands = buffer.drain();
    expect(commands.some((c) => c.type === "RemoveNode")).toBe(true);
  });

  it("insertBefore emits InsertBefore command", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const parent = reconciler.createInstance("Box", {});
    const child1 = reconciler.createInstance("Text", {});
    const child2 = reconciler.createInstance("Text", {});
    reconciler.appendChild(parent, child1);
    buffer.clear();
    reconciler.insertBefore(parent, child2, child1);
    const commands = buffer.drain();
    expect(commands.some((c) => c.type === "InsertBefore")).toBe(true);
  });

  it("commitUpdate with style emits SetStyle", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const instance = reconciler.createInstance("Box", {});
    buffer.clear();
    reconciler.commitUpdate(instance, { style: { bold: true } });
    const commands = buffer.drain();
    expect(commands.some((c) => c.type === "SetStyle")).toBe(true);
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

  it("commitTextUpdate emits SetText when id exists", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const text = reconciler.createTextInstance("hello");
    buffer.clear();
    reconciler.commitTextUpdate(text, "world");
    const commands = buffer.drain();
    expect(commands.some((c) => c.type === "SetText")).toBe(true);
  });

  it("commitTextUpdate without id does not emit SetText", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const text = createTextInstance("hello");
    buffer.clear();
    reconciler.commitTextUpdate(text, "world");
    const commands = buffer.drain();
    expect(commands.some((c) => c.type === "SetText")).toBe(false);
  });

  it("createTextInstance emits CreateNode and SetText", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const text = reconciler.createTextInstance("hello");
    const commands = buffer.drain();
    expect(commands.some((c) => c.type === "CreateNode")).toBe(true);
    expect(commands.some((c) => c.type === "SetText")).toBe(true);
    expect(text.text).toBe("hello");
  });

  it("appendChild with text instance works", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const parent = reconciler.createInstance("Box", {});
    const text = reconciler.createTextInstance("hello");
    buffer.clear();
    reconciler.appendChild(parent, text);
    const commands = buffer.drain();
    expect(commands.some((c) => c.type === "AppendChild")).toBe(true);
  });

  it("appendChild with plain text instance covers id fallback branch", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const parent = reconciler.createInstance("Box", {});
    const text = createTextInstance("hello");
    buffer.clear();
    reconciler.appendChild(parent, text);
    const commands = buffer.drain();
    expect(commands.some((c) => c.type === "AppendChild")).toBe(true);
  });

  it("removeChild with plain text instance covers id fallback branch", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const parent = reconciler.createInstance("Box", {});
    const text = createTextInstance("hello");
    reconciler.appendChild(parent, text);
    buffer.clear();
    reconciler.removeChild(parent, text);
    const commands = buffer.drain();
    expect(commands.some((c) => c.type === "RemoveNode")).toBe(true);
  });

  it("insertBefore with plain text instance covers id fallback branch", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const parent = reconciler.createInstance("Box", {});
    const child = reconciler.createInstance("Text", {});
    const text = createTextInstance("hello");
    reconciler.appendChild(parent, child);
    buffer.clear();
    reconciler.insertBefore(parent, text, child);
    const commands = buffer.drain();
    expect(commands.some((c) => c.type === "InsertBefore")).toBe(true);
  });

  it("insertBefore with both plain text instances covers both id fallback branches", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const parent = reconciler.createInstance("Box", {});
    const text1 = createTextInstance("hello");
    const text2 = createTextInstance("world");
    reconciler.appendChild(parent, text1);
    buffer.clear();
    reconciler.insertBefore(parent, text2, text1);
    const commands = buffer.drain();
    expect(commands.some((c) => c.type === "InsertBefore")).toBe(true);
  });

  it("prepareUpdate delegates correctly", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const instance = reconciler.createInstance("Box", {});
    const result = reconciler.prepareUpdate(instance, "Box", {}, { custom: "value" });
    expect(result).toEqual({ custom: "value" });
  });

  it("finalizeInitialChildren delegates", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const instance = reconciler.createInstance("Box", {});
    expect(reconciler.finalizeInitialChildren(instance)).toBe(false);
  });

  it("resetAfterCommit delegates", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    expect(() => reconciler.resetAfterCommit()).not.toThrow();
  });
});
