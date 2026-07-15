import { describe, expect, it } from "vitest";
import { Diff } from "../../widgets/Diff";

describe("Diff", () => {
  it("constructs with default options", () => {
    const diff = new Diff();
    expect(diff.content).toBe("");
  });

  it("constructs with content", () => {
    const diff = new Diff({ content: "diff content" });
    expect(diff.content).toBe("diff content");
  });

  it("constructs with old/new content", () => {
    const diff = new Diff({ oldContent: "old", newContent: "new" });
    expect(diff.options.oldContent).toBe("old");
    expect(diff.options.newContent).toBe("new");
  });

  it("renderCommands creates Diff node", () => {
    const diff = new Diff({ content: "change" });
    const cmds = diff.renderCommands("d1");
    expect(cmds[0]?.type).toBe("CreateNode");
    if (cmds[0]?.type === "CreateNode") {
      expect(cmds[0].kind).toBe("Diff");
    }
  });

  it("renderCommands includes SetText when content exists", () => {
    const diff = new Diff({ content: "-old\n+new" });
    const cmds = diff.renderCommands("d1");
    expect(cmds.some((c) => c.type === "SetText")).toBe(true);
  });

  it("renderCommands returns only CreateNode without content", () => {
    const diff = new Diff();
    const cmds = diff.renderCommands("d1");
    expect(cmds.length).toBe(1);
  });

  it("content setter works", () => {
    const diff = new Diff({ content: "old" });
    diff.content = "new";
    expect(diff.content).toBe("new");
  });

  it("update changes content", () => {
    const diff = new Diff({ content: "old" });
    diff.update({ content: "new" });
    expect(diff.content).toBe("new");
  });
});
