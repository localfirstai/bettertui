import { afterEach, describe, expect, it, vi } from "vitest";
import type { Command } from "../command";
import {
  CommandBuffer,
  Runtime,
  appendChild,
  commitTextUpdate,
  commitUpdate,
  createInstance,
  createReconciler,
  createTextInstance,
  generateId,
  prepareUpdate,
  removeChild,
} from "../index";

afterEach(() => {
  vi.restoreAllMocks();
});

describe("generateId edge cases", () => {
  it("generates monotonically increasing ids", () => {
    const ids = Array.from({ length: 100 }, () => generateId());
    for (let i = 1; i < ids.length; i++) {
      expect(Number(ids[i])).toBeGreaterThan(Number(ids[i - 1]));
    }
  });

  it("generates unique ids in batch", () => {
    const ids = new Set(Array.from({ length: 1000 }, () => generateId()));
    expect(ids.size).toBe(1000);
  });
});

describe("CommandBuffer edge cases", () => {
  it("handles push after drain", () => {
    const buffer = new CommandBuffer();
    buffer.push({ type: "Shutdown" });
    buffer.drain();
    buffer.push({ type: "CreateNode", id: "1", kind: "Box" });
    expect(buffer.length).toBe(1);
    expect(buffer.drain()[0]?.type).toBe("CreateNode");
  });

  it("handles interleaved push and peek", () => {
    const buffer = new CommandBuffer();
    buffer.push({ type: "Shutdown" });
    expect(buffer.peek()).toHaveLength(1);
    buffer.push({ type: "Shutdown" });
    expect(buffer.peek()).toHaveLength(2);
    expect(buffer.length).toBe(2);
  });

  it("clear on empty buffer is a no-op", () => {
    const buffer = new CommandBuffer();
    expect(() => buffer.clear()).not.toThrow();
    expect(buffer.isEmpty).toBe(true);
  });

  it("drain on empty buffer returns empty array", () => {
    const buffer = new CommandBuffer();
    const result = buffer.drain();
    expect(result).toEqual([]);
  });

  it("peek on empty buffer returns empty array", () => {
    const buffer = new CommandBuffer();
    expect(buffer.peek()).toEqual([]);
  });

  it("push preserves command order", () => {
    const buffer = new CommandBuffer();
    const commands: Command[] = [
      { type: "CreateNode", id: "1", kind: "Box" },
      { type: "AppendChild", parent: "1", child: "2" },
      { type: "SetStyle", id: "1", style: { bold: true } },
      { type: "BeginFrame", frameId: 1 },
      { type: "CommitFrame", frameId: 1 },
      { type: "Shutdown" },
    ];
    for (const cmd of commands) {
      buffer.push(cmd);
    }
    const drained = buffer.drain();
    expect(drained).toHaveLength(commands.length);
    for (let i = 0; i < commands.length; i++) {
      expect(drained[i]?.type).toBe(commands[i]?.type);
    }
  });
});

describe("Runtime edge cases", () => {
  it("flush on empty buffer does not call subscribers", () => {
    const runtime = new Runtime();
    const fn = vi.fn();
    runtime.subscribe(fn);
    runtime.flush();
    expect(fn).not.toHaveBeenCalled();
  });

  it("dispose while running stops the frame loop", () => {
    const runtime = new Runtime({ autoStart: true });
    expect(runtime.isRunning).toBe(true);
    runtime.dispose();
    expect(runtime.isRunning).toBe(false);
  });

  it("dispose multiple times is safe", () => {
    const runtime = new Runtime();
    runtime.dispose();
    expect(() => runtime.dispose()).not.toThrow();
  });

  it("unsubscribe removes only the correct subscriber", () => {
    const runtime = new Runtime();
    const fns = Array.from({ length: 5 }, () => vi.fn());
    const unsubs = fns.map((fn) => runtime.subscribe(fn));
    unsubs[2]?.();
    runtime.commandBuffer.push({ type: "Shutdown" });
    runtime.flush();
    expect(fns[2]).not.toHaveBeenCalled();
    for (let i = 0; i < 5; i++) {
      if (i !== 2) {
        expect(fns[i]).toHaveBeenCalledTimes(1);
      }
    }
  });

  it("multiple unsubscribes of same subscriber are safe", () => {
    const runtime = new Runtime();
    const fn = vi.fn();
    const unsub = runtime.subscribe(fn);
    unsub();
    unsub();
    runtime.commandBuffer.push({ type: "Shutdown" });
    runtime.flush();
    expect(fn).not.toHaveBeenCalled();
  });

  it("onFrame unsubscribe removes callback", () => {
    const runtime = new Runtime();
    const fn = vi.fn();
    const unsub = runtime.onFrame(fn);
    unsub();
    runtime.startFrameLoop(10);
    runtime.stopFrameLoop();
    expect(fn).not.toHaveBeenCalled();
  });

  it("subscriber receives batched commands from single flush", () => {
    const runtime = new Runtime();
    const received: Command[][] = [];
    runtime.subscribe((cmds) => received.push(cmds));
    runtime.commandBuffer.push({ type: "CreateNode", id: "1", kind: "Box" });
    runtime.commandBuffer.push({ type: "SetText", id: "1", text: "hello" });
    runtime.commandBuffer.push({ type: "Shutdown" });
    runtime.flush();
    expect(received).toHaveLength(1);
    expect(received[0]).toHaveLength(3);
  });

  it("requestFrame on stopped runtime is no-op", () => {
    const runtime = new Runtime();
    const fn = vi.fn();
    runtime.subscribe(fn);
    runtime.requestFrame();
    expect(fn).not.toHaveBeenCalled();
  });

  it("constructor with autoStart starts frame loop immediately", () => {
    const runtime = new Runtime({ autoStart: true });
    expect(runtime.isRunning).toBe(true);
    runtime.dispose();
  });

  it("constructor with custom frameIntervalMs", () => {
    const runtime = new Runtime({ frameIntervalMs: 100 });
    expect(runtime.isRunning).toBe(false);
    runtime.startFrameLoop();
    expect(runtime.isRunning).toBe(true);
    runtime.dispose();
  });
});

describe("Reconciler error handling", () => {
  it("commitUpdate with empty payload does not emit SetStyle", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const instance = reconciler.createInstance("Box", {});
    buffer.clear();
    reconciler.commitUpdate(instance, {});
    expect(buffer.isEmpty).toBe(true);
  });

  it("commitUpdate with only non-style props does not emit SetStyle", () => {
    const buffer = new CommandBuffer();
    const reconciler = createReconciler(buffer);
    const instance = reconciler.createInstance("Box", {});
    buffer.clear();
    reconciler.commitUpdate(instance, { padding: 2, margin: 3 });
    expect(buffer.isEmpty).toBe(true);
  });
});

describe("Tree operation edge cases", () => {
  it("appendChild changes parent reference", () => {
    const parent1 = createInstance("Box", {});
    const parent2 = createInstance("Box", {});
    const child = createInstance("Text", {});
    appendChild(parent1, child);
    expect(child.parent).toBe(parent1);
    appendChild(parent2, child);
    expect(child.parent).toBe(parent2);
    expect(parent2.children).toContain(child);
  });

  it("removeChild that was never appended does not throw", () => {
    const parent = createInstance("Box", {});
    const child = createInstance("Text", {});
    expect(() => removeChild(parent, child)).not.toThrow();
    expect(parent.children).toHaveLength(0);
  });

  it("commitUpdate overwrites previous values", () => {
    const instance = createInstance("Box", { padding: 1, margin: 0 });
    commitUpdate(instance, { padding: 5 });
    expect(instance.props["padding"]).toBe(5);
    expect(instance.props["margin"]).toBe(0);
  });

  it("commitTextUpdate called multiple times", () => {
    const text = createTextInstance("hello");
    commitTextUpdate(text, "world");
    commitTextUpdate(text, "foo");
    expect(text.text).toBe("foo");
  });

  it("prepareUpdate returns null-equivalent when nothing meaningful changes", () => {
    const instance = createInstance("Box", {});
    const result = prepareUpdate(instance, "Box", {}, { children: [], style: {}, layout: {} });
    expect(result).toEqual({});
  });

  it("prepareUpdate filters out style with empty object", () => {
    const instance = createInstance("Box", {});
    const result = prepareUpdate(instance, "Box", {}, { style: {}, customProp: 42 });
    expect(result).toEqual({ customProp: 42 });
  });
});

describe("Runtime frame lifecycle", () => {
  it("stopFrameLoop on non-running runtime is safe", () => {
    const runtime = new Runtime();
    expect(() => runtime.stopFrameLoop()).not.toThrow();
  });

  it("startFrameLoop then immediate stop stops cleanly", () => {
    const runtime = new Runtime();
    runtime.startFrameLoop(10);
    runtime.stopFrameLoop();
    expect(runtime.isRunning).toBe(false);
  });

  it("restart frame loop after stop", () => {
    const runtime = new Runtime();
    runtime.startFrameLoop(10);
    runtime.stopFrameLoop();
    runtime.startFrameLoop(20);
    expect(runtime.isRunning).toBe(true);
    runtime.stopFrameLoop();
  });

  it("tick with frame callbacks receives delta", async () => {
    vi.useFakeTimers();
    const runtime = new Runtime({ frameIntervalMs: 50 });
    const deltas: number[] = [];
    runtime.onFrame((delta) => deltas.push(delta));
    runtime.startFrameLoop();
    vi.advanceTimersByTime(100);
    runtime.stopFrameLoop();
    expect(deltas.length).toBeGreaterThanOrEqual(1);
    vi.useRealTimers();
  });
});

describe("Command string-to-number serialization", () => {
  it("creates valid JSON for all command types", () => {
    const buffer = new CommandBuffer();
    buffer.push({ type: "CreateNode", id: "42", kind: "Box" });
    buffer.push({ type: "AppendChild", parent: "1", child: "2" });
    buffer.push({ type: "SetStyle", id: "3", style: { bold: true, fg: "red" } });
    buffer.push({ type: "SetText", id: "4", text: "hello" });
    buffer.push({ type: "Shutdown" });

    const commands = buffer.drain();
    const idKeys = new Set([
      "id",
      "parent",
      "child",
      "reference",
      "node",
      "newParent",
      "old",
      "new",
    ]);
    const converted = commands.map((cmd) => {
      const out: Record<string, unknown> = { type: cmd.type };
      for (const [key, value] of Object.entries(cmd)) {
        if (key === "type") continue;
        if (idKeys.has(key) && typeof value === "string") {
          out[key] = Number(value);
        } else {
          out[key] = value;
        }
      }
      return out;
    });

    const json = JSON.stringify(converted);
    const parsed = JSON.parse(json);
    expect(parsed[0]?.id).toBe(42);
    expect(typeof parsed[0]?.id).toBe("number");
    expect(parsed[1]?.parent).toBe(1);
    expect(parsed[1]?.child).toBe(2);
    expect(parsed[2]?.style?.bold).toBe(true);
    expect(parsed[4]?.type).toBe("Shutdown");
  });

  it("handles NaN and Infinity command properties gracefully", () => {
    const buffer = new CommandBuffer();
    buffer.push({ type: "SetLayout", id: "1", layout: { width: Number.NaN } });
    const commands = buffer.drain();
    const json = JSON.stringify(commands);
    expect(json).toContain("null");
  });
});
