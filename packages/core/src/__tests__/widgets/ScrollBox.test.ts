import type { KeyEvent, MouseEvent } from "@bettertui/shared";
import { describe, expect, it } from "vitest";
import { ScrollBox } from "../../widgets/ScrollBox";

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

describe("ScrollBox", () => {
  it("constructs with default options", () => {
    const sb = new ScrollBox();
    expect(sb.options.width).toBeUndefined();
    expect(sb.options.height).toBeUndefined();
  });

  it("constructs with width and height", () => {
    const sb = new ScrollBox({ width: "100%", height: 200 });
    expect(sb.options.width).toBe("100%");
    expect(sb.options.height).toBe(200);
  });

  it("constructs with scroll flags", () => {
    const sb = new ScrollBox({ scrollX: true, scrollY: true });
    expect(sb.options.scrollX).toBe(true);
    expect(sb.options.scrollY).toBe(true);
  });

  it("renderCommands creates ScrollBox node", () => {
    const sb = new ScrollBox();
    const cmds = sb.renderCommands("sb1");
    expect(cmds[0]?.type).toBe("CreateNode");
    if (cmds[0]?.type === "CreateNode") {
      expect(cmds[0].kind).toBe("ScrollBox");
    }
  });

  it("renderCommands includes size commands when set", () => {
    const sb = new ScrollBox({ width: "auto", height: 100 });
    const cmds = sb.renderCommands("sb1");
    expect(cmds.some((c) => c.type === "SetWidth")).toBe(true);
    expect(cmds.some((c) => c.type === "SetHeight")).toBe(true);
  });

  it("renderCommands omits size commands when not set", () => {
    const sb = new ScrollBox();
    const cmds = sb.renderCommands("sb1");
    expect(cmds.some((c) => c.type === "SetWidth")).toBe(false);
    expect(cmds.some((c) => c.type === "SetHeight")).toBe(false);
  });

  it("handleKey returns false for unknown keys", () => {
    const sb = new ScrollBox();
    expect(sb.handleKey(key("a"))).toBe(false);
  });

  it("scrolls down on ArrowDown", () => {
    const sb = new ScrollBox({ scrollY: true });
    sb.setContentSize(80, 100);
    sb.setViewSize(80, 10);
    expect(sb.offsetY).toBe(0);
    sb.handleKey(key("down"));
    expect(sb.offsetY).toBe(1);
  });

  it("scrolls up on ArrowUp", () => {
    const sb = new ScrollBox({ scrollY: true });
    sb.setContentSize(80, 100);
    sb.setViewSize(80, 10);
    sb.scrollTo(0, 5);
    sb.handleKey(key("up"));
    expect(sb.offsetY).toBe(4);
  });

  it("does not scroll below 0", () => {
    const sb = new ScrollBox({ scrollY: true });
    sb.setContentSize(80, 100);
    sb.setViewSize(80, 10);
    sb.handleKey(key("up"));
    expect(sb.offsetY).toBe(0);
  });

  it("scrolls to home on Home key", () => {
    const sb = new ScrollBox({ scrollY: true });
    sb.setContentSize(80, 100);
    sb.setViewSize(80, 10);
    sb.scrollTo(0, 20);
    sb.handleKey(key("home"));
    expect(sb.offsetY).toBe(0);
  });

  it("calls onScroll callback when scrolling", () => {
    let lastOffsetY = -1;
    const sb = new ScrollBox({
      scrollY: true,
      onScroll: (_, y) => {
        lastOffsetY = y;
      },
    });
    sb.setContentSize(80, 100);
    sb.setViewSize(80, 10);
    sb.handleKey(key("down"));
    expect(lastOffsetY).toBe(1);
  });

  it("does not handle horizontal keys when scrollX is false", () => {
    const sb = new ScrollBox({ scrollX: false });
    sb.setContentSize(200, 100);
    sb.setViewSize(80, 10);
    expect(sb.handleKey(key("right"))).toBe(false);
    expect(sb.offsetX).toBe(0);
  });

  it("scrolls horizontally when scrollX is true", () => {
    const sb = new ScrollBox({ scrollX: true });
    sb.setContentSize(200, 100);
    sb.setViewSize(80, 10);
    sb.handleKey(key("right"));
    expect(sb.offsetX).toBe(1);
  });

  it("handleMouse scroll_down scrolls Y by 3", () => {
    const sb = new ScrollBox();
    sb.setContentSize(80, 100);
    sb.setViewSize(80, 10);
    const ev: MouseEvent = {
      button: "scroll_down",
      position: { x: 0, y: 0 },
      ctrl: false,
      shift: false,
      alt: false,
    };
    sb.handleMouse(ev);
    expect(sb.offsetY).toBe(3);
  });

  it("handleMouse scroll_up scrolls Y negative", () => {
    const sb = new ScrollBox();
    sb.setContentSize(80, 100);
    sb.setViewSize(80, 10);
    const ev: MouseEvent = {
      button: "scroll_down",
      position: { x: 0, y: 0 },
      ctrl: false,
      shift: false,
      alt: false,
    };
    sb.handleMouse(ev);
    sb.handleMouse(ev);
    const evUp: MouseEvent = {
      button: "scroll_up",
      position: { x: 0, y: 0 },
      ctrl: false,
      shift: false,
      alt: false,
    };
    sb.handleMouse(evUp);
    expect(sb.offsetY).toBe(3);
  });

  it("handleMouse scroll_down scrolls X when scrollX enabled", () => {
    const sb = new ScrollBox({ scrollX: true, scrollY: false });
    sb.setContentSize(200, 10);
    sb.setViewSize(80, 10);
    const ev: MouseEvent = {
      button: "scroll_down",
      position: { x: 0, y: 0 },
      ctrl: false,
      shift: false,
      alt: false,
    };
    sb.handleMouse(ev);
    expect(sb.offsetX).toBe(3);
  });
});
