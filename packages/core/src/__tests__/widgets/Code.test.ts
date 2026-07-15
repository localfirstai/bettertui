import { describe, expect, it } from "vitest";
import { Code } from "../../widgets/Code";

describe("Code", () => {
  it("constructs with empty content", () => {
    const code = new Code();
    expect(code.content).toBe("");
  });

  it("constructs with content and language", () => {
    const code = new Code("const x = 1;", { language: "typescript" });
    expect(code.content).toBe("const x = 1;");
    expect(code.options.language).toBe("typescript");
  });

  it("renderCommands creates Code node", () => {
    const code = new Code("fn main()");
    const cmds = code.renderCommands("c1");
    const createCmd = cmds.find((c) => c.type === "CreateNode");
    expect(createCmd).toBeDefined();
    if (createCmd?.type === "CreateNode") {
      expect(createCmd.kind).toBe("Code");
    }
  });

  it("renderCommands sets background", () => {
    const code = new Code("code");
    const cmds = code.renderCommands("c1");
    expect(cmds.some((c) => c.type === "SetBackground")).toBe(true);
  });

  it("renderCommands sets text when content exists", () => {
    const code = new Code("hello");
    const cmds = code.renderCommands("c1");
    expect(cmds.some((c) => c.type === "SetText")).toBe(true);
  });

  it("renderCommands omits SetText when content is empty", () => {
    const code = new Code();
    const cmds = code.renderCommands("c1");
    expect(cmds.some((c) => c.type === "SetText")).toBe(false);
  });

  it("content setter updates value", () => {
    const code = new Code("old");
    code.content = "new";
    expect(code.content).toBe("new");
  });
});
