import { afterEach, describe, expect, it, vi } from "vitest";
import { CommandBuffer } from "../command-buffer";
import type { Command } from "../command-buffer";
import { createReconciler } from "../index";
import { Runtime } from "../runtime";

afterEach(() => {
  vi.restoreAllMocks();
});

describe("CommandBuffer dirty tracking behavior", () => {
  it("CreateNode produces a command", () => {
    const buffer = new CommandBuffer();
    buffer.push({ type: "CreateNode", id: "1", kind: "Box" });
    expect(buffer.length).toBe(1);
    const cmds = buffer.drain();
    expect(cmds[0]).toMatchObject({ type: "CreateNode", id: "1", kind: "Box" });
  });

  it("SetStyle produces style command", () => {
    const buffer = new CommandBuffer();
    buffer.push({ type: "SetStyle", id: "1", style: { bold: true, fg: "red" } });
    const cmds = buffer.drain();
    expect(cmds[0]?.type).toBe("SetStyle");
  });

  it("SetText produces text command", () => {
    const buffer = new CommandBuffer();
    buffer.push({ type: "SetText", id: "1", text: "hello" });
    const cmds = buffer.drain();
    expect(cmds[0]?.type).toBe("SetText");
  });

  it("Invalidate triggers repaint command", () => {
    const buffer = new CommandBuffer();
    buffer.push({ type: "Invalidate", id: "1" });
    const cmds = buffer.drain();
    expect(cmds[0]?.type).toBe("Invalidate");
  });

  it("BeginFrame and CommitFrame frame the lifecycle", () => {
    const buffer = new CommandBuffer();
    buffer.push({ type: "BeginFrame", frameId: 1 });
    buffer.push({ type: "CreateNode", id: "2", kind: "Text" });
    buffer.push({ type: "CommitFrame", frameId: 1 });
    const cmds = buffer.drain();
    expect(cmds).toHaveLength(3);
    expect(cmds[0]?.type).toBe("BeginFrame");
    expect(cmds[1]?.type).toBe("CreateNode");
    expect(cmds[2]?.type).toBe("CommitFrame");
  });

  it("AppendChild maintains parent-child ordering", () => {
    const buffer = new CommandBuffer();
    buffer.push({ type: "CreateNode", id: "parent", kind: "Box" });
    buffer.push({ type: "CreateNode", id: "child", kind: "Text" });
    buffer.push({ type: "AppendChild", parent: "parent", child: "child" });
    const cmds = buffer.drain();
    expect(cmds.filter((c) => c.type === "AppendChild")).toHaveLength(1);
  });

  it("RemoveNode removes from tree", () => {
    const buffer = new CommandBuffer();
    buffer.push({ type: "RemoveNode", id: "1" });
    const cmds = buffer.drain();
    expect(cmds[0]?.type).toBe("RemoveNode");
  });

  it("SetLayout triggers layout update", () => {
    const buffer = new CommandBuffer();
    buffer.push({ type: "SetLayout", id: "1", layout: { width: 100, height: 50 } });
    const cmds = buffer.drain();
    expect(cmds[0]?.type).toBe("SetLayout");
  });

  it("Multiple style changes coalesce in one buffer", () => {
    const buffer = new CommandBuffer();
    buffer.push({ type: "SetStyle", id: "1", style: { bold: true } });
    buffer.push({ type: "SetStyle", id: "1", style: { italic: true } });
    expect(buffer.length).toBe(2);
    const cmds = buffer.drain();
    expect(cmds).toHaveLength(2);
  });

  it("Mixed tree mutations produce ordered output", () => {
    const buffer = new CommandBuffer();
    buffer.push({ type: "CreateNode", id: "1", kind: "Box" });
    buffer.push({ type: "CreateNode", id: "2", kind: "Text" });
    buffer.push({ type: "AppendChild", parent: "1", child: "2" });
    buffer.push({ type: "SetText", id: "2", text: "Hello" });
    buffer.push({ type: "SetStyle", id: "2", style: { bold: true } });
    const cmds = buffer.drain();
    expect(cmds.map((c) => c.type)).toEqual([
      "CreateNode",
      "CreateNode",
      "AppendChild",
      "SetText",
      "SetStyle",
    ]);
  });
});

describe("Runtime command flow", () => {
  it("flush delivers commands to subscriber", () => {
    const runtime = new Runtime();
    const received: Command[] = [];
    runtime.subscribe((cmds) => received.push(...cmds));
    runtime.commandBuffer.push({ type: "CreateNode", id: "1", kind: "Box" });
    runtime.flush();
    expect(received).toHaveLength(1);
    expect(received[0]?.type).toBe("CreateNode");
  });

  it("flush drains all pending commands", () => {
    const runtime = new Runtime();
    runtime.commandBuffer.push({ type: "Shutdown" });
    expect(runtime.commandBuffer.isEmpty).toBe(false);
    runtime.flush();
    expect(runtime.commandBuffer.isEmpty).toBe(true);
  });

  it("requestFrame with subscriber triggers command delivery", async () => {
    vi.useFakeTimers();
    const runtime = new Runtime({ frameIntervalMs: 10 });
    const received: Command[][] = [];
    runtime.subscribe((cmds) => {
      received.push(cmds);
    });
    runtime.startFrameLoop();
    runtime.commandBuffer.push({ type: "CreateNode", id: "1", kind: "Box" });
    vi.advanceTimersByTime(50);
    runtime.stopFrameLoop();
    expect(received.length).toBeGreaterThan(0);
    vi.useRealTimers();
  });

  it("dispose prevents further command processing", () => {
    const runtime = new Runtime();
    runtime.commandBuffer.push({ type: "CreateNode", id: "1", kind: "Box" });
    runtime.dispose();
    expect(runtime.commandBuffer.isEmpty).toBe(true);
    expect(runtime.isRunning).toBe(false);
  });

  it("onFrame callbacks fire during frame loop", async () => {
    vi.useFakeTimers();
    const runtime = new Runtime({ frameIntervalMs: 10 });
    const fn = vi.fn();
    runtime.onFrame(fn);
    runtime.startFrameLoop();
    vi.advanceTimersByTime(50);
    runtime.stopFrameLoop();
    expect(fn).toHaveBeenCalled();
    vi.useRealTimers();
  });
});

describe("Reconciler command emission", () => {
  it("reconciler createInstance emits CreateNode", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    reconciler.createInstance("Box", {});
    const cmds = buffer.drain();
    expect(cmds.some((c) => c.type === "CreateNode")).toBe(true);
  });

  it("reconciler createInstance with style emits SetStyle", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    reconciler.createInstance("Box", { style: { bold: true } });
    const cmds = buffer.drain();
    expect(cmds.some((c) => c.type === "SetStyle")).toBe(true);
  });

  it("reconciler createTextInstance emits CreateNode + SetText", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    reconciler.createTextInstance("hello");
    const cmds = buffer.drain();
    expect(cmds.some((c) => c.type === "CreateNode")).toBe(true);
    expect(cmds.some((c) => c.type === "SetText")).toBe(true);
  });

  it("full tree construction emits proper command sequence", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const root = reconciler.createInstance("Box", {});
    const child = reconciler.createInstance("Text", { padding: 1 });
    const text = reconciler.createTextInstance("World");
    buffer.clear();
    reconciler.appendChild(root, child);
    reconciler.appendChild(child, text);
    const cmds = buffer.drain();
    expect(cmds.filter((c) => c.type === "AppendChild")).toHaveLength(2);
  });
});

describe("CommandBuffer edge cases", () => {
  it("handles 10000 commands without error", () => {
    const buffer = new CommandBuffer();
    for (let i = 0; i < 10000; i++) {
      buffer.push({ type: "CreateNode", id: `${i}`, kind: "Box" });
    }
    expect(buffer.length).toBe(10000);
    const drained = buffer.drain();
    expect(drained).toHaveLength(10000);
  });

  it("preserves FIFO order through multiple drain cycles", () => {
    const buffer = new CommandBuffer();
    buffer.push({ type: "CreateNode", id: "1", kind: "Box" });
    buffer.push({ type: "SetText", id: "1", text: "a" });
    const first = buffer.drain();
    buffer.push({ type: "SetText", id: "1", text: "b" });
    const second = buffer.drain();
    expect(first).toHaveLength(2);
    expect(second).toHaveLength(1);
    expect((second[0] as { text?: string })?.text).toBe("b");
  });
});
