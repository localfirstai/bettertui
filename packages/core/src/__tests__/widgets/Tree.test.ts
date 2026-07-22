import type { KeyEvent } from "@bettertui/shared";
import type { TreeNode } from "../../widgets/widget.types";
import { describe, expect, it } from "vitest";
import { Tree } from "../../widgets/Tree";

function key(name: string): KeyEvent {
  return {
    key: name,
    code: name,
    ctrl: false,
    shift: false,
    alt: false,
    meta: false,
    eventType: "press",
    source: "raw",
  };
}

const nodes: TreeNode[] = [
  {
    id: "root1",
    label: "Root 1",
    expanded: true,
    children: [
      { id: "child1a", label: "Child 1a" },
      { id: "child1b", label: "Child 1b" },
    ],
  },
  {
    id: "root2",
    label: "Root 2",
    children: [{ id: "child2a", label: "Child 2a" }],
  },
];

describe("Tree", () => {
  it("constructs with nodes", () => {
    const t = new Tree({ nodes });
    const cmds = t.renderCommands("t1");
    expect(cmds[0]?.type).toBe("CreateNode");
  });

  it("renders expanded children", () => {
    const t = new Tree({ nodes });
    const cmds = t.renderCommands("t1");
    // root1 is expanded, so root1 + child1a + child1b + root2 = 4 rows
    const rows = cmds.filter(
      (c) => c.type === "CreateNode" && "id" in c && c.id.startsWith("t1-row-"),
    );
    expect(rows.length).toBe(4);
  });

  it("does not render collapsed children", () => {
    const collapsed: TreeNode[] = [
      { id: "r1", label: "Root", expanded: false, children: [{ id: "c1", label: "Child" }] },
    ];
    const t = new Tree({ nodes: collapsed });
    const cmds = t.renderCommands("t1");
    const rows = cmds.filter(
      (c) => c.type === "CreateNode" && "id" in c && c.id.startsWith("t1-row-"),
    );
    expect(rows.length).toBe(1); // Only root, no child
  });

  it("handleKey ArrowDown moves selection", () => {
    const t = new Tree({ nodes, selectedId: "root1" });
    t.handleKey(key("down"));
    // root1 is expanded, next is child1a
    const cmds = t.renderCommands("t1");
    // row 0 = root1, row 1 = child1a (selected)
    const row1Inverse = cmds.find(
      (c) => c.type === "SetInverse" && "id" in c && c.id === "t1-row-1",
    );
    expect(row1Inverse).toBeDefined();
  });

  it("handleKey ArrowRight expands collapsed node", () => {
    const collapsed: TreeNode[] = [
      { id: "r1", label: "Root", expanded: false, children: [{ id: "c1", label: "Child" }] },
    ];
    const t = new Tree({ nodes: collapsed, selectedId: "r1" });
    t.handleKey(key("right"));
    const cmds = t.renderCommands("t1");
    const rows = cmds.filter(
      (c) => c.type === "CreateNode" && "id" in c && c.id.startsWith("t1-row-"),
    );
    expect(rows.length).toBe(2); // root + child now visible
  });

  it("handleKey ArrowLeft collapses expanded node", () => {
    const expanded: TreeNode[] = [
      { id: "r1", label: "Root", expanded: true, children: [{ id: "c1", label: "Child" }] },
    ];
    const t = new Tree({ nodes: expanded, selectedId: "r1" });
    t.handleKey(key("left"));
    const cmds = t.renderCommands("t1");
    const rows = cmds.filter(
      (c) => c.type === "CreateNode" && "id" in c && c.id.startsWith("t1-row-"),
    );
    expect(rows.length).toBe(1); // Only root
  });

  it("handleKey Home jumps to first item", () => {
    const t = new Tree({ nodes, selectedId: "child1a" });
    t.handleKey(key("home"));
    const cmds = t.renderCommands("t1");
    const row0Inverse = cmds.find(
      (c) => c.type === "SetInverse" && "id" in c && c.id === "t1-row-0",
    );
    expect(row0Inverse).toBeDefined();
  });
});
