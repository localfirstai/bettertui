import type { KeyEvent, MouseEvent } from "@bettertui/shared";
import { describe, expect, it } from "vitest";
import { Dialog } from "../../widgets/Dialog";

function key(name: string): KeyEvent {
  return {
    key: name,
    code: name,
    ctrl: false,
    shift: false,
    alt: false,
    meta: false,
    eventType: "press",
    source: "raw",
  };
}

describe("Dialog", () => {
  it("constructs closed by default", () => {
    const d = new Dialog();
    expect(d.isOpen).toBe(false);
  });

  it("constructs open when option set", () => {
    const d = new Dialog({ open: true });
    expect(d.isOpen).toBe(true);
  });

  it("open() sets isOpen=true", () => {
    const d = new Dialog();
    d.open();
    expect(d.isOpen).toBe(true);
  });

  it("close() sets isOpen=false", () => {
    const d = new Dialog({ open: true });
    d.close();
    expect(d.isOpen).toBe(false);
  });

  it("close() calls onClose callback", () => {
    let closed = false;
    const d = new Dialog({
      open: true,
      onClose: () => {
        closed = true;
      },
    });
    d.close();
    expect(closed).toBe(true);
  });

  it("renderCommands hides content when closed", () => {
    const d = new Dialog({ open: false });
    const cmds = d.renderCommands("dlg1");
    const setHidden = cmds.find((c) => c.type === "SetHidden" && "value" in c && c.value === true);
    expect(setHidden).toBeDefined();
  });

  it("renderCommands shows content when open", () => {
    const d = new Dialog({ open: true });
    const cmds = d.renderCommands("dlg1");
    const setHidden = cmds.find((c) => c.type === "SetHidden");
    expect(setHidden).toBeUndefined();
  });

  it("renderCommands includes title when set", () => {
    const d = new Dialog({ open: true, title: "My Dialog" });
    const cmds = d.renderCommands("dlg1");
    const titleCreate = cmds.find(
      (c) => c.type === "CreateNode" && "id" in c && c.id === "dlg1-title",
    );
    expect(titleCreate).toBeDefined();
  });

  it("handleKey Escape closes dialog when closeOnEsc not false", () => {
    const d = new Dialog({ open: true });
    const result = d.handleKey(key("escape"));
    expect(result).toBe(true);
    expect(d.isOpen).toBe(false);
  });

  it("handleKey Escape does not close when closeOnEsc=false", () => {
    const d = new Dialog({ open: true, closeOnEsc: false });
    const result = d.handleKey(key("escape"));
    expect(result).toBe(false);
    expect(d.isOpen).toBe(true);
  });

  it("handleKey returns false when closed", () => {
    const d = new Dialog({ open: false });
    expect(d.handleKey(key("escape"))).toBe(false);
  });

  it("update can change open state", () => {
    const d = new Dialog({ open: false });
    d.update({ open: true });
    expect(d.isOpen).toBe(true);
  });

  it("handleMouse left-click closes when closeOnClickOutside=true", () => {
    const d = new Dialog({ open: true, closeOnClickOutside: true });
    const ev: MouseEvent = {
      button: "left",
      position: { x: 5, y: 5 },
      ctrl: false,
      shift: false,
      alt: false,
    };
    const result = d.handleMouse(ev);
    expect(result).toBe(true);
    expect(d.isOpen).toBe(false);
  });

  it("handleMouse does not close when closeOnClickOutside not set", () => {
    const d = new Dialog({ open: true });
    const ev: MouseEvent = {
      button: "left",
      position: { x: 5, y: 5 },
      ctrl: false,
      shift: false,
      alt: false,
    };
    const result = d.handleMouse(ev);
    expect(result).toBe(false);
    expect(d.isOpen).toBe(true);
  });

  it("handleMouse returns false when closed", () => {
    const d = new Dialog({ open: false, closeOnClickOutside: true });
    const ev: MouseEvent = {
      button: "left",
      position: { x: 5, y: 5 },
      ctrl: false,
      shift: false,
      alt: false,
    };
    expect(d.handleMouse(ev)).toBe(false);
  });
});
