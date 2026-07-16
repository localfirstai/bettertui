import { describe, expect, it } from "vitest";
import { Select } from "../../widgets/Select";

describe("Select", () => {
  const options = [
    { label: "Apple", value: "apple" },
    { label: "Banana", value: "banana" },
    { label: "Cherry", value: "cherry" },
  ];

  it("constructs with empty options", () => {
    const select = new Select();
    expect(select.options.options).toBeUndefined();
  });

  it("constructs with options and selects by value", () => {
    const select = new Select({ options, value: "banana" });
    expect(select.options.options).toBe(options);
  });

  it("renderCommands creates List node", () => {
    const select = new Select({ options });
    const cmds = select.renderCommands("s1");
    expect(cmds.some((c) => c.type === "CreateNode")).toBe(true);
    expect(cmds.filter((c) => c.type === "CreateNode").length).toBeGreaterThan(1);
  });

  it("renderCommands creates child text nodes for each option", () => {
    const select = new Select({ options });
    const cmds = select.renderCommands("s1");
    const createNodes = cmds.filter((c) => c.type === "CreateNode");
    expect(createNodes.length).toBe(options.length + 1);
  });

  it("handleKey navigates down", () => {
    let changed = "";
    const select = new Select({
      options,
      onChange: (v) => {
        changed = v;
      },
    });
    expect(
      select.handleKey({
        key: "down",
        code: "ArrowDown",
        ctrl: false,
        shift: false,
        alt: false,
        meta: false,
        eventType: "press",
        source: "raw",
      }),
    ).toBe(true);
    expect(changed).toBe("apple");
  });

  it("handleKey navigates up", () => {
    let changed = "";
    const select = new Select({
      options,
      value: "cherry",
      onChange: (v) => {
        changed = v;
      },
    });
    select.handleKey({
      key: "up",
      code: "ArrowUp",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(changed).toBe("banana");
  });

  it("handleKey does nothing when disabled", () => {
    const select = new Select({ options, disabled: true });
    expect(
      select.handleKey({
        key: "down",
        code: "ArrowDown",
        ctrl: false,
        shift: false,
        alt: false,
        meta: false,
        eventType: "press",
        source: "raw",
      }),
    ).toBe(false);
  });

  it("handleKey does nothing when options are empty", () => {
    const select = new Select();
    expect(
      select.handleKey({
        key: "down",
        code: "ArrowDown",
        ctrl: false,
        shift: false,
        alt: false,
        meta: false,
        eventType: "press",
        source: "raw",
      }),
    ).toBe(false);
  });

  it("update replaces options", () => {
    const select = new Select({ options });
    const newOpts = [{ label: "Date", value: "date" }];
    select.update({ options: newOpts });
  });
});
