import { describe, expect, it } from "vitest";
import { Input } from "../../widgets/Input";

describe("Input", () => {
  it("constructs with default value", () => {
    const input = new Input();
    expect(input.value).toBe("");
  });

  it("constructs with initial value", () => {
    const input = new Input({ value: "hello" });
    expect(input.value).toBe("hello");
  });

  it("constructs with placeholder", () => {
    const input = new Input({ placeholder: "type here" });
    expect(input.options.placeholder).toBe("type here");
  });

  it("renderCommands creates Input node", () => {
    const input = new Input();
    const cmds = input.renderCommands("i1");
    expect(cmds.some((c) => c.type === "CreateNode")).toBe(true);
  });

  it("renderCommands shows placeholder when value is empty", () => {
    const input = new Input({ placeholder: "type here" });
    const cmds = input.renderCommands("i1");
    const setText = cmds.find((c) => c.type === "SetText");
    expect(setText).toBeDefined();
  });

  it("renderCommands masks text when password is true", () => {
    const input = new Input({ value: "secret", password: true });
    const cmds = input.renderCommands("i1");
    const setText = cmds.find((c) => c.type === "SetText");
    expect(setText).toBeDefined();
    if (setText?.type === "SetText") {
      expect(setText.text).toBe("******");
    }
  });

  it("renderCommands sets dim when disabled", () => {
    const input = new Input({ disabled: true });
    const cmds = input.renderCommands("i1");
    expect(cmds.some((c) => c.type === "SetDim")).toBe(true);
  });

  it("handleKey ignores events when disabled", () => {
    const input = new Input({ disabled: true, value: "test" });
    const result = input.handleKey({
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
    expect(input.value).toBe("test");
  });

  it("handleKey appends character on key press", () => {
    const input = new Input({ value: "he" });
    const result = input.handleKey({
      key: "l",
      code: "KeyL",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(result).toBe(true);
    expect(input.value).toBe("hel");
  });

  it("handleKey handles backspace", () => {
    const input = new Input({ value: "hello" });
    const result = input.handleKey({
      key: "backspace",
      code: "Backspace",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(result).toBe(true);
    expect(input.value).toBe("hell");
  });

  it("handleKey handles return and calls onSubmit", () => {
    let submitted = "";
    const input = new Input({
      value: "test",
      onSubmit: (v) => {
        submitted = v;
      },
    });
    const result = input.handleKey({
      key: "return",
      code: "Enter",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(result).toBe(true);
    expect(submitted).toBe("test");
  });

  it("handleKey ignores ctrl+key combinations", () => {
    const input = new Input({ value: "hi" });
    const result = input.handleKey({
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
    expect(input.value).toBe("hi");
  });

  it("update changes value", () => {
    const input = new Input({ value: "old" });
    input.update({ value: "new" });
    expect(input.value).toBe("new");
  });
});
