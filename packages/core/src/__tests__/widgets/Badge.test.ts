import { describe, expect, it } from "vitest";
import { Badge } from "../../widgets/Badge";

describe("Badge", () => {
  it("constructs with label", () => {
    const b = new Badge({ label: "NEW" });
    expect(b.options.label).toBe("NEW");
  });

  it("renderCommands creates Text node", () => {
    const b = new Badge({ label: "OK" });
    const cmds = b.renderCommands("b1");
    expect(cmds[0]?.type).toBe("CreateNode");
    if (cmds[0]?.type === "CreateNode") {
      expect(cmds[0].kind).toBe("Text");
    }
  });

  it("renderCommands sets text with padding spaces", () => {
    const b = new Badge({ label: "TEST" });
    const cmds = b.renderCommands("b1");
    const setText = cmds.find((c) => c.type === "SetText");
    expect(setText).toBeDefined();
    if (setText?.type === "SetText") {
      expect(setText.text).toBe(" TEST ");
    }
  });

  it("renderCommands sets bold", () => {
    const b = new Badge({ label: "X" });
    const cmds = b.renderCommands("b1");
    expect(cmds.some((c) => c.type === "SetBold")).toBe(true);
  });

  it("uses success variant colors", () => {
    const b = new Badge({ label: "OK", variant: "success" });
    const cmds = b.renderCommands("b1");
    const setFg = cmds.find((c) => c.type === "SetForeground");
    expect(setFg).toBeDefined();
  });

  it("accepts custom color override", () => {
    const b = new Badge({ label: "X", color: "#ff0000" });
    const cmds = b.renderCommands("b1");
    const setFg = cmds.find(
      (c) => c.type === "SetForeground" && "color" in c && c.color === "#ff0000",
    );
    expect(setFg).toBeDefined();
  });
});
