import { describe, expect, it } from "vitest";
import { ScrollBar } from "../../widgets/ScrollBar";

describe("ScrollBar", () => {
  it("constructs with default options", () => {
    const sb = new ScrollBar();
    expect(sb.options.orientation).toBeUndefined();
  });

  it("constructs with vertical orientation", () => {
    const sb = new ScrollBar({ orientation: "vertical" });
    expect(sb.options.orientation).toBe("vertical");
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

  it("handleKey returns false", () => {
    const sb = new ScrollBar();
    expect(sb.handleKey()).toBe(false);
  });
});
