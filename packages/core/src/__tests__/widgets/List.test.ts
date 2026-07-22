import type { KeyEvent } from "@bettertui/shared";
import { describe, expect, it } from "vitest";
import { List } from "../../widgets/List";

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

describe("List", () => {
  const items = [
    { id: "a", label: "Alpha" },
    { id: "b", label: "Beta" },
    { id: "c", label: "Gamma" },
  ];

  it("constructs with default options", () => {
    const l = new List();
    expect(l.selectedItem).toBeUndefined();
  });

  it("constructs with items", () => {
    const l = new List({ items });
    expect(l.selectedIndex).toBe(0);
  });

  it("renderCommands creates List node", () => {
    const l = new List({ items });
    const cmds = l.renderCommands("l1");
    expect(cmds[0]?.type).toBe("CreateNode");
    if (cmds[0]?.type === "CreateNode") {
      expect(cmds[0].kind).toBe("List");
    }
  });

  it("renders visible items", () => {
    const l = new List({ items });
    const cmds = l.renderCommands("l1");
    const itemNodes = cmds.filter(
      (c) => c.type === "CreateNode" && "id" in c && c.id.startsWith("l1-item-"),
    );
    expect(itemNodes.length).toBe(3);
  });

  it("handleKey ArrowDown moves selection", () => {
    const l = new List({ items });
    l.handleKey(key("down"));
    expect(l.selectedIndex).toBe(1);
  });

  it("handleKey ArrowUp moves selection up", () => {
    const l = new List({ items });
    l.handleKey(key("down"));
    l.handleKey(key("up"));
    expect(l.selectedIndex).toBe(0);
  });

  it("handleKey does not go below 0", () => {
    const l = new List({ items });
    l.handleKey(key("up"));
    expect(l.selectedIndex).toBe(0);
  });

  it("handleKey Enter calls onSelect", () => {
    let selected = "";
    const l = new List({
      items,
      onSelect: (item) => {
        selected = item.id;
      },
    });
    l.handleKey(key("return"));
    expect(selected).toBe("a");
  });

  it("handleKey ArrowDown calls onChange", () => {
    let changed = "";
    const l = new List({
      items,
      onChange: (item) => {
        changed = item.id;
      },
    });
    l.handleKey(key("down"));
    expect(changed).toBe("b");
  });

  it("handleKey Home jumps to first", () => {
    const l = new List({ items });
    l.handleKey(key("down"));
    l.handleKey(key("down"));
    l.handleKey(key("home"));
    expect(l.selectedIndex).toBe(0);
  });

  it("handleKey End jumps to last", () => {
    const l = new List({ items });
    l.handleKey(key("end"));
    expect(l.selectedIndex).toBe(2);
  });

  it("update replaces items and resets selection", () => {
    const l = new List({ items });
    l.handleKey(key("down"));
    l.update({ items: [{ id: "x", label: "X" }] });
    expect(l.selectedIndex).toBe(0);
  });
});
