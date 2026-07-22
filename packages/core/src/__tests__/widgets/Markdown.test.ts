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

  it("renderCommands creates a Box container node", () => {
    const md = new Markdown({ content: "hi" });
    const cmds = md.renderCommands("m1");
    expect(cmds[0]?.type).toBe("CreateNode");
    if (cmds[0]?.type === "CreateNode") {
      // Container is now a Box with column direction (rich rendering)
      expect(cmds[0].kind).toBe("Box");
    }
  });

  it("renderCommands includes SetText when content exists", () => {
    const md = new Markdown({ content: "hello world" });
    const cmds = md.renderCommands("m1");
    expect(cmds.some((c) => c.type === "SetText")).toBe(true);
  });

  it("renderCommands returns only container for empty content", () => {
    const md = new Markdown();
    const cmds = md.renderCommands("m1");
    // Container node + SetFlexDirection, no children
    expect(cmds.length).toBeLessThanOrEqual(2);
    expect(cmds.every((c) => c.type !== "AppendChild")).toBe(true);
  });

  it("renders heading with bold", () => {
    const md = new Markdown({ content: "# Title" });
    const cmds = md.renderCommands("m1");
    const boldCmd = cmds.find((c) => c.type === "SetBold");
    expect(boldCmd).toBeDefined();
  });

  it("renders H1 with white foreground", () => {
    const md = new Markdown({ content: "# H1 Title" });
    const cmds = md.renderCommands("m1");
    const fgCmd = cmds.find((c) => c.type === "SetForeground");
    expect(fgCmd).toBeDefined();
    if (fgCmd?.type === "SetForeground") {
      expect(fgCmd.color).toBe("white");
    }
  });

  it("renders H2 with cyan foreground", () => {
    const md = new Markdown({ content: "## Subtitle" });
    const cmds = md.renderCommands("m1");
    const fgCmd = cmds.find((c) => c.type === "SetForeground");
    expect(fgCmd).toBeDefined();
    if (fgCmd?.type === "SetForeground") {
      expect(fgCmd.color).toBe("cyan");
    }
  });

  it("renders horizontal rule as dim text", () => {
    const md = new Markdown({ content: "---" });
    const cmds = md.renderCommands("m1");
    const dimCmd = cmds.find((c) => c.type === "SetDim");
    expect(dimCmd).toBeDefined();
    const textCmd = cmds.find((c) => c.type === "SetText");
    expect(textCmd).toBeDefined();
    if (textCmd?.type === "SetText") {
      expect(textCmd.text).toContain("─");
    }
  });

  it("renders blockquote with │ prefix", () => {
    const md = new Markdown({ content: "> This is a quote" });
    const cmds = md.renderCommands("m1");
    const textCmd = cmds.find((c) => c.type === "SetText" && c.text.startsWith("│"));
    expect(textCmd).toBeDefined();
  });

  it("renders unordered list with bullet prefix", () => {
    const md = new Markdown({ content: "- Item one\n- Item two" });
    const cmds = md.renderCommands("m1");
    const bulletCmd = cmds.find((c) => c.type === "SetText" && c.text === "• ");
    expect(bulletCmd).toBeDefined();
  });

  it("renders ordered list with number prefix", () => {
    const md = new Markdown({ content: "1. First\n2. Second" });
    const cmds = md.renderCommands("m1");
    const numCmd = cmds.find((c) => c.type === "SetText" && c.text === "1. ");
    expect(numCmd).toBeDefined();
  });

  it("renders bold inline text", () => {
    const md = new Markdown({ content: "Hello **world** bye" });
    const cmds = md.renderCommands("m1");
    const boldCmd = cmds.find((c) => c.type === "SetBold");
    expect(boldCmd).toBeDefined();
  });

  it("renders italic inline text", () => {
    const md = new Markdown({ content: "Hello *world* bye" });
    const cmds = md.renderCommands("m1");
    const italicCmd = cmds.find((c) => c.type === "SetItalic");
    expect(italicCmd).toBeDefined();
  });

  it("renders inline code with background", () => {
    const md = new Markdown({ content: "Use `const x` here" });
    const cmds = md.renderCommands("m1");
    const bgCmd = cmds.find((c) => c.type === "SetBackground");
    expect(bgCmd).toBeDefined();
  });

  it("renders strikethrough", () => {
    const md = new Markdown({ content: "~~deleted~~" });
    const cmds = md.renderCommands("m1");
    const stCmd = cmds.find((c) => c.type === "SetStrikethrough");
    expect(stCmd).toBeDefined();
  });

  it("renders code fence as Box with background", () => {
    const md = new Markdown({ content: "```\nconst x = 1\n```" });
    const cmds = md.renderCommands("m1");
    const bgCmd = cmds.find((c) => c.type === "SetBackground");
    expect(bgCmd).toBeDefined();
  });

  it("update changes content", () => {
    const md = new Markdown({ content: "old" });
    md.update({ content: "new" });
    expect(md.content).toBe("new");
  });

  it("renders paragraph with child text nodes", () => {
    const md = new Markdown({ content: "Hello world" });
    const cmds = md.renderCommands("m1");
    const appendCmds = cmds.filter((c) => c.type === "AppendChild");
    expect(appendCmds.length).toBeGreaterThan(0);
  });
});
