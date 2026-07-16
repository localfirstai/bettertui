import { describe, expect, it } from "vitest";
import { Text } from "../../widgets/Text";

describe("Text", () => {
  it("constructs with empty content", () => {
    const text = new Text();
    expect(text.content).toBe("");
  });

  it("constructs with initial content", () => {
    const text = new Text("hello");
    expect(text.content).toBe("hello");
  });

  it("constructs with options", () => {
    const text = new Text("bold text", { bold: true, color: "red" });
    expect(text.content).toBe("bold text");
    expect(text.options.bold).toBe(true);
    expect(text.options.color).toBe("red");
  });

  it("set content updates value", () => {
    const text = new Text("old");
    text.content = "new";
    expect(text.content).toBe("new");
  });

  it("renderCommands creates Text node with SetText", () => {
    const text = new Text("hello");
    const cmds = text.renderCommands("t1");
    const createCmd = cmds.find((c) => c.type === "CreateNode");
    expect(createCmd).toBeDefined();
    if (createCmd?.type === "CreateNode") {
      expect(createCmd.kind).toBe("Text");
    }
    const setTextCmd = cmds.find((c) => c.type === "SetText");
    expect(setTextCmd).toBeDefined();
    if (setTextCmd?.type === "SetText") {
      expect(setTextCmd.text).toBe("hello");
    }
  });

  it("renderCommands returns only CreateNode when content is empty", () => {
    const text = new Text();
    const cmds = text.renderCommands("t1");
    expect(cmds.length).toBe(1);
    expect(cmds[0]?.type).toBe("CreateNode");
  });

  it("renderCommands includes style commands", () => {
    const text = new Text("styled", { bold: true, italic: true, color: "green" });
    const cmds = text.renderCommands("t1");
    expect(cmds.some((c) => c.type === "SetBold")).toBe(true);
    expect(cmds.some((c) => c.type === "SetItalic")).toBe(true);
    expect(cmds.some((c) => c.type === "SetForeground")).toBe(true);
  });

  it("update merges options and content", () => {
    const text = new Text("old");
    text.update({ content: "new", color: "blue" });
    expect(text.content).toBe("new");
    expect(text.options.color).toBe("blue");
  });
});
