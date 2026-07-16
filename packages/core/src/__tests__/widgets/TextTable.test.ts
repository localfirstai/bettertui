import { describe, expect, it } from "vitest";
import { TextTable } from "../../widgets/TextTable";

describe("TextTable", () => {
  it("constructs with default options", () => {
    const tt = new TextTable();
    expect(tt.options.rows).toBeUndefined();
  });

  it("constructs with headers and rows", () => {
    const tt = new TextTable({
      headers: ["Name", "Age"],
      rows: [
        ["Alice", "30"],
        ["Bob", "25"],
      ],
    });
    expect(tt.options.headers).toHaveLength(2);
    expect(tt.options.rows).toHaveLength(2);
  });

  it("renderCommands creates Box node", () => {
    const tt = new TextTable();
    const cmds = tt.renderCommands("tt1");
    expect(cmds[0]?.type).toBe("CreateNode");
  });

  it("renderCommands renders header row when headers provided", () => {
    const tt = new TextTable({ headers: ["A", "B"] });
    const cmds = tt.renderCommands("tt1");
    expect(cmds.some((c) => c.type === "SetBold")).toBe(true);
  });

  it("renderCommands skips header when showHeader is false", () => {
    const tt = new TextTable({ headers: ["A"], showHeader: false });
    const cmds = tt.renderCommands("tt1");
    expect(cmds.some((c) => c.type === "SetBold")).toBe(false);
  });

  it("renderCommands renders data rows", () => {
    const tt = new TextTable({
      rows: [
        ["x", "y"],
        ["z", "w"],
      ],
    });
    const cmds = tt.renderCommands("tt1");
    const appendCmds = cmds.filter((c) => c.type === "AppendChild");
    expect(appendCmds.length).toBeGreaterThan(1);
  });

  it("update replaces rows", () => {
    const tt = new TextTable({ rows: [["a"]] });
    tt.update({ rows: [["b"], ["c"]] });
    expect(tt.options.rows).toHaveLength(2);
  });
});
