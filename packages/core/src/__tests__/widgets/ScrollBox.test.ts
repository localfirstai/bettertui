import { describe, expect, it } from "vitest";
import { ScrollBox } from "../../widgets/ScrollBox";

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

  it("handleKey returns false", () => {
    const sb = new ScrollBox();
    expect(sb.handleKey()).toBe(false);
  });
});
