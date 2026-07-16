import { describe, expect, it } from "vitest";
import { TabSelect } from "../../widgets/TabSelect";

describe("TabSelect", () => {
  const tabs = [
    { label: "Tab 1", value: "t1" },
    { label: "Tab 2", value: "t2" },
    { label: "Tab 3", value: "t3" },
  ];

  it("constructs with default options", () => {
    const ts = new TabSelect();
    expect(ts.options.tabs).toBeUndefined();
  });

  it("constructs with tabs and activeIndex", () => {
    const ts = new TabSelect({ tabs, activeIndex: 1 });
    expect(ts.options.tabs).toBe(tabs);
    expect(ts.options.activeIndex).toBe(1);
  });

  it("renderCommands creates Box node", () => {
    const ts = new TabSelect({ tabs });
    const cmds = ts.renderCommands("ts1");
    expect(cmds.some((c) => c.type === "CreateNode")).toBe(true);
    expect(cmds.some((c) => c.type === "SetFlexDirection")).toBe(true);
  });

  it("renderCommands creates text nodes for each tab", () => {
    const ts = new TabSelect({ tabs });
    const cmds = ts.renderCommands("ts1");
    const textNodes = cmds.filter((c) => c.type === "CreateNode");
    expect(textNodes.length).toBe(tabs.length + 1);
  });

  it("handleKey navigates right", () => {
    let val = "";
    const ts = new TabSelect({
      tabs,
      onChange: (v) => {
        val = v;
      },
    });
    ts.handleKey({
      key: "right",
      code: "ArrowRight",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
    });
    expect(val).toBe("t2");
  });

  it("handleKey navigates left", () => {
    let val = "";
    const ts = new TabSelect({
      tabs,
      activeIndex: 1,
      onChange: (v) => {
        val = v;
      },
    });
    ts.handleKey({
      key: "left",
      code: "ArrowLeft",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
    });
    expect(val).toBe("t1");
  });

  it("handleKey wraps at boundaries", () => {
    const ts = new TabSelect({ tabs, activeIndex: 0 });
    ts.handleKey({
      key: "left",
      code: "ArrowLeft",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
    });
    expect(ts.options.activeIndex).toBe(0);
  });

  it("handleKey does nothing with empty tabs", () => {
    const ts = new TabSelect();
    expect(
      ts.handleKey({
        key: "right",
        code: "ArrowRight",
        ctrl: false,
        shift: false,
        alt: false,
        meta: false,
      }),
    ).toBe(false);
  });

  it("update updates tabs and activeIndex", () => {
    const ts = new TabSelect({ tabs });
    ts.update({ activeIndex: 2 });
    expect(ts.options.activeIndex).toBe(2);
  });
});
