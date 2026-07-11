import { describe, expect, it } from "vitest";
import {
  CommandBuffer,
  Runtime,
  appendChild,
  commitTextUpdate,
  commitUpdate,
  createInstance,
  createReconciler,
  createTextInstance,
  finalizeInitialChildren,
  generateId,
  insertBefore,
  removeChild,
  resetAfterCommit,
} from "../index";

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

  it("removeChild removes child and clears parent", () => {
    const parent = createInstance("Box", {});
    const child = createInstance("Text", {});
    appendChild(parent, child);
    removeChild(parent, child);
    expect(child.parent).toBeNull();
    expect(parent.children).not.toContain(child);
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

  it("commitUpdate merges payload into instance props", () => {
    const instance = createInstance("Box", { padding: 1 });
    commitUpdate(instance, { padding: 2, margin: 3 });
    expect(instance.props.padding).toBe(2);
    expect(instance.props.margin).toBe(3);
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
});

describe("Runtime", () => {
  it("creates with default buffer", () => {
    const runtime = new Runtime();
    expect(runtime.commandBuffer).toBeInstanceOf(CommandBuffer);
    expect(runtime.isRunning).toBe(false);
  });

  it("creates with custom buffer", () => {
    const buffer = new CommandBuffer();
    const runtime = new Runtime(buffer);
    expect(runtime.commandBuffer).toBe(buffer);
  });

  it("subscribe and flush sends commands", () => {
    const runtime = new Runtime();
    const received: unknown[][] = [];
    runtime.subscribe((cmds) => received.push(cmds));
    runtime.commandBuffer.push({ type: "Shutdown" });
    runtime.flush();
    expect(received).toHaveLength(1);
    expect(received[0]).toHaveLength(1);
  });

  it("unsubscribe stops receiving", () => {
    const runtime = new Runtime();
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
    const runtime = new Runtime();
    runtime.commandBuffer.push({ type: "Shutdown" });
    runtime.dispose();
    expect(runtime.commandBuffer.isEmpty).toBe(true);
  });

  it("startFrameLoop and stopFrameLoop", () => {
    const runtime = new Runtime();
    runtime.startFrameLoop(100);
    expect(runtime.isRunning).toBe(true);
    runtime.stopFrameLoop();
    expect(runtime.isRunning).toBe(false);
  });

  it("onFrame registers callback", () => {
    const runtime = new Runtime();
    const unsub = runtime.onFrame(() => {});
    expect(typeof unsub).toBe("function");
    unsub();
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

  it("createTextInstance emits CreateNode and SetText", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const text = reconciler.createTextInstance("hello");
    const commands = buffer.drain();
    expect(commands.some((c) => c.type === "CreateNode")).toBe(true);
    expect(commands.some((c) => c.type === "SetText")).toBe(true);
    expect(text.text).toBe("hello");
  });
});
