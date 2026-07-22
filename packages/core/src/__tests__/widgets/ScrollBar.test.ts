import type { KeyEvent } from "@bettertui/shared";
import { describe, expect, it } from "vitest";
import { ScrollBar } from "../../widgets/ScrollBar";

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

describe("ScrollBar", () => {
  it("constructs with default options", () => {
    const sb = new ScrollBar();
    expect(sb.options.orientation).toBeUndefined();
  });

  it("constructs with vertical orientation", () => {
    const sb = new ScrollBar({ orientation: "vertical" });
    expect(sb.options.orientation).toBe("vertical");
  });

  it("has position 0 by default", () => {
    const sb = new ScrollBar();
    expect(sb.position).toBe(0);
  });

  it("constructs with initial position", () => {
    const sb = new ScrollBar({ position: 50 });
    expect(sb.position).toBe(50);
  });

  it("renderCommands creates ScrollBar node", () => {
    const sb = new ScrollBar();
    const cmds = sb.renderCommands("sb1");
    expect(cmds[0]?.type).toBe("CreateNode");
    if (cmds[0]?.type === "CreateNode") {
      expect(cmds[0].kind).toBe("ScrollBar");
    }
  });

  it("renderCommands sets width=1 for vertical orientation", () => {
    const sb = new ScrollBar({ orientation: "vertical" });
    const cmds = sb.renderCommands("sb1");
    expect(cmds.some((c) => c.type === "SetWidth")).toBe(true);
    expect(cmds.some((c) => c.type === "SetHeight")).toBe(true);
  });

  it("renderCommands sets height=1 for horizontal orientation", () => {
    const sb = new ScrollBar({ orientation: "horizontal" });
    const cmds = sb.renderCommands("sb1");
    expect(cmds.some((c) => c.type === "SetHeight")).toBe(true);
  });

  it("handleKey returns false for unknown keys", () => {
    const sb = new ScrollBar();
    expect(sb.handleKey(key("a"))).toBe(false);
  });

  it("vertical scrollbar moves down on ArrowDown", () => {
    const sb = new ScrollBar({ orientation: "vertical", position: 0 });
    sb.handleKey(key("down"));
    expect(sb.position).toBe(5);
  });

  it("vertical scrollbar moves up on ArrowUp", () => {
    const sb = new ScrollBar({ orientation: "vertical", position: 20 });
    sb.handleKey(key("up"));
    expect(sb.position).toBe(15);
  });

  it("vertical scrollbar clamps to 0", () => {
    const sb = new ScrollBar({ orientation: "vertical", position: 2 });
    sb.handleKey(key("up"));
    expect(sb.position).toBe(0);
  });

  it("horizontal scrollbar moves right on ArrowRight", () => {
    const sb = new ScrollBar({ orientation: "horizontal", position: 0 });
    sb.handleKey(key("right"));
    expect(sb.position).toBe(5);
  });

  it("calls onChange when position changes", () => {
    let lastPos = -1;
    const sb = new ScrollBar({
      orientation: "vertical",
      position: 0,
      onChange: (p) => {
        lastPos = p;
      },
    });
    sb.handleKey(key("down"));
    expect(lastPos).toBe(5);
  });

  it("setPosition clamps to valid range", () => {
    const sb = new ScrollBar({ trackSize: 100, thumbSize: 10 });
    sb.setPosition(200); // beyond max
    expect(sb.position).toBe(90); // max = 100 - 10
  });
});
