import { describe, expect, it } from "vitest";
import { Markdown } from "../../widgets/Markdown";

describe("Markdown", () => {
  it("constructs with default options", () => {
    const md = new Markdown();
    expect(md.content).toBe("");
  });

  it("constructs with content", () => {
    const md = new Markdown({ content: "# Hello" });
    expect(md.content).toBe("# Hello");
  });

  it("content setter updates value", () => {
    const md = new Markdown({ content: "old" });
    md.content = "new";
    expect(md.content).toBe("new");
  });

  it("renderCommands creates Markdown node", () => {
    const md = new Markdown({ content: "hi" });
    const cmds = md.renderCommands("m1");
    expect(cmds[0]?.type).toBe("CreateNode");
    if (cmds[0]?.type === "CreateNode") {
      expect(cmds[0].kind).toBe("Markdown");
    }
  });

  it("renderCommands includes SetText when content exists", () => {
    const md = new Markdown({ content: "**bold**" });
    const cmds = md.renderCommands("m1");
    expect(cmds.some((c) => c.type === "SetText")).toBe(true);
  });

  it("renderCommands omits SetText for empty content", () => {
    const md = new Markdown();
    const cmds = md.renderCommands("m1");
    expect(cmds.length).toBe(1);
  });

  it("update changes content", () => {
    const md = new Markdown({ content: "old" });
    md.update({ content: "new" });
    expect(md.content).toBe("new");
  });
});
