import { describe, expect, it } from "vitest";
import { Textarea } from "../../widgets/Textarea";

describe("Textarea", () => {
  it("constructs with default value", () => {
    const ta = new Textarea();
    expect(ta.value).toBe("");
  });

  it("constructs with initial value and rows", () => {
    const ta = new Textarea({ value: "hello\nworld", rows: 5 });
    expect(ta.value).toBe("hello\nworld");
    expect(ta.options.rows).toBe(5);
  });

  it("renderCommands creates Box node", () => {
    const ta = new Textarea();
    const cmds = ta.renderCommands("ta1");
    expect(cmds.some((c) => c.type === "CreateNode")).toBe(true);
    expect(cmds.some((c) => c.type === "SetHeight")).toBe(true);
  });

  it("renderCommands sets default height to 3", () => {
    const ta = new Textarea();
    const cmds = ta.renderCommands("ta1");
    const heightCmd = cmds.find((c) => c.type === "SetHeight");
    expect(heightCmd).toBeDefined();
    if (heightCmd?.type === "SetHeight") {
      expect(heightCmd.value).toBe(3);
    }
  });

  it("handleKey appends character", () => {
    const ta = new Textarea({ value: "hi" });
    const result = ta.handleKey({
      key: "!",
      code: "Bang",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(result).toBe(true);
    expect(ta.value).toBe("hi!");
  });

  it("handleKey inserts newline on return", () => {
    const ta = new Textarea({ value: "line1" });
    ta.handleKey({
      key: "return",
      code: "Enter",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(ta.value).toBe("line1\n");
  });

  it("handleKey handles backspace", () => {
    const ta = new Textarea({ value: "hello" });
    ta.handleKey({
      key: "backspace",
      code: "Backspace",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(ta.value).toBe("hell");
  });

  it("handleKey ignores ctrl combinations", () => {
    const ta = new Textarea({ value: "hi" });
    const result = ta.handleKey({
      key: "c",
      code: "KeyC",
      ctrl: true,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(result).toBe(false);
  });

  it("handleKey Ctrl+Left jumps word left", () => {
    const ta = new Textarea({ value: "hello world" });
    // cursor at end after construction
    ta.handleKey({
      key: "left",
      code: "ArrowLeft",
      ctrl: true,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    // should jump back one word ("world")
    expect(ta.cursorPosition).toBeLessThan(11);
  });

  it("handleKey Ctrl+Right jumps word right", () => {
    const ta = new Textarea({ value: "hello world" });
    // move to start
    ta.handleKey({
      key: "home",
      code: "Home",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    ta.handleKey({
      key: "right",
      code: "ArrowRight",
      ctrl: true,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(ta.cursorPosition).toBeGreaterThan(0);
  });

  it("handleKey Ctrl+K kills to end of line", () => {
    const ta = new Textarea({ value: "hello\nworld" });
    // cursor at end (pos 11, on "world" line). Home → start of "world" (pos 6).
    ta.handleKey({
      key: "home",
      code: "Home",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    ta.handleKey({
      key: "k",
      code: "KeyK",
      ctrl: true,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(ta.value).toBe("hello\n");
  });

  it("handleKey Ctrl+U kills to start of line", () => {
    const ta = new Textarea({ value: "hello\nworld" });
    // cursor at end (pos 11, on "world" line). Ctrl+U kills from lineStart(6) to pos(11).
    ta.handleKey({
      key: "u",
      code: "KeyU",
      ctrl: true,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(ta.value).toBe("hello\n");
  });

  it("handleKey no-ops when disabled", () => {
    const ta = new Textarea({ value: "test", disabled: true });
    const result = ta.handleKey({
      key: "a",
      code: "KeyA",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(result).toBe(false);
    expect(ta.value).toBe("test");
  });
});
