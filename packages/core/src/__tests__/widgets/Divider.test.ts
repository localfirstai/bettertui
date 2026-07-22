import { describe, expect, it } from "vitest";
import { Divider } from "../../widgets/Divider";

describe("Divider", () => {
  it("constructs with default options", () => {
    const d = new Divider();
    expect(d.options.orientation).toBeUndefined();
  });

  it("renderCommands creates Box node", () => {
    const d = new Divider();
    const cmds = d.renderCommands("d1");
    expect(cmds[0]?.type).toBe("CreateNode");
    if (cmds[0]?.type === "CreateNode") {
      expect(cmds[0].kind).toBe("Box");
    }
  });

  it("horizontal divider sets full width", () => {
    const d = new Divider({ orientation: "horizontal" });
    const cmds = d.renderCommands("d1");
    const setWidth = cmds.find((c) => c.type === "SetWidth");
    expect(setWidth).toBeDefined();
  });

  it("vertical divider sets width=1", () => {
    const d = new Divider({ orientation: "vertical" });
    const cmds = d.renderCommands("d1");
    const setWidth = cmds.find((c) => c.type === "SetWidth" && "value" in c && c.value === 1);
    expect(setWidth).toBeDefined();
  });

  it("renders label in horizontal divider", () => {
    const d = new Divider({ label: "Section" });
    const cmds = d.renderCommands("d1");
    const labelCreate = cmds.find(
      (c) => c.type === "CreateNode" && "id" in c && c.id === "d1-label",
    );
    expect(labelCreate).toBeDefined();
  });

  it("applies color to line", () => {
    const d = new Divider({ color: "blue" });
    const cmds = d.renderCommands("d1");
    expect(cmds.some((c) => c.type === "SetForeground")).toBe(true);
  });
});
