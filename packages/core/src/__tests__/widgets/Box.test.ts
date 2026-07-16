import { describe, expect, it } from "vitest";
import { Box } from "../../widgets/Box";

describe("Box", () => {
  it("constructs with default options", () => {
    const box = new Box();
    expect(box.options).toEqual({});
    expect(box.id).toBeDefined();
  });

  it("constructs with custom options", () => {
    const box = new Box({ width: 100, height: 50, border: true, title: "Test" });
    expect(box.options.width).toBe(100);
    expect(box.options.height).toBe(50);
    expect(box.options.border).toBe(true);
    expect(box.options.title).toBe("Test");
  });

  it("renderCommands returns CreateNode for Box kind", () => {
    const box = new Box();
    const cmds = box.renderCommands("box1");
    expect(cmds[0]?.type).toBe("CreateNode");
    if (cmds[0]?.type === "CreateNode") {
      expect(cmds[0].id).toBe("box1");
      expect(cmds[0].kind).toBe("Box");
    }
  });

  it("renderCommands includes layout commands for set options", () => {
    const box = new Box({ width: 100, height: 50, flexDirection: "row" });
    const cmds = box.renderCommands("b1");
    expect(cmds.some((c) => c.type === "SetWidth")).toBe(true);
    expect(cmds.some((c) => c.type === "SetHeight")).toBe(true);
    expect(cmds.some((c) => c.type === "SetFlexDirection")).toBe(true);
  });

  it("renderCommands includes border style commands when border is true", () => {
    const box = new Box({ border: true, borderStyle: "rounded" });
    const cmds = box.renderCommands("b1");
    expect(cmds.some((c) => c.type === "CreateNode")).toBe(true);
  });

  it("update merges new options", () => {
    const box = new Box({ width: 100 });
    box.update({ height: 200, border: true });
    expect(box.options.width).toBe(100);
    expect(box.options.height).toBe(200);
    expect(box.options.border).toBe(true);
  });
});
