import { describe, expect, it, vi } from "vitest";
import { KeyEvent } from "../../lib/keyHandler";
import type { ParsedKey } from "../../lib/parseKeypress";
import { SelectEvents } from "../../lib/renderableEvents";
import { CliRenderer } from "../../platform/cliRenderer";
import { Select, type SelectOption } from "../../renderables/Select";
import { createMockKeys } from "../../testing/mockKeys";
import { createTestStdin, createTestStdout } from "../../testing/testStreams";

function muteStdout() {
  return vi.spyOn(process.stdout, "write").mockImplementation(() => true);
}

function setup(width = 80, height = 24) {
  const spy = muteStdout();
  const stdin = createTestStdin();
  const stdout = createTestStdout(width, height);
  const origStdin = process.stdin;
  const origStdout = process.stdout;
  Object.defineProperty(process, "stdin", {
    value: stdin,
    writable: true,
    configurable: true,
  });
  Object.defineProperty(process, "stdout", {
    value: stdout,
    writable: true,
    configurable: true,
  });

  const renderer = new CliRenderer({ width, height, autoStart: false });
  renderer.start();
  const mockInput = createMockKeys(renderer);

  const cleanup = () => {
    renderer.stop();
    renderer.destroy();
    spy.mockRestore();
    Object.defineProperty(process, "stdin", {
      value: origStdin,
      writable: true,
      configurable: true,
    });
    Object.defineProperty(process, "stdout", {
      value: origStdout,
      writable: true,
      configurable: true,
    });
  };

  return { renderer, mockInput, cleanup, stdin };
}

function createKeyEvent(
  name: string,
  modifiers: Partial<
    Pick<ParsedKey, "ctrl" | "shift" | "meta" | "option" | "super" | "baseCode">
  > = {},
): KeyEvent {
  return new KeyEvent({
    name,
    sequence: name,
    ctrl: modifiers.ctrl ?? false,
    meta: modifiers.meta ?? false,
    shift: modifiers.shift ?? false,
    option: modifiers.option ?? false,
    number: false,
    raw: name,
    eventType: "press",
    source: "raw",
    ...(modifiers.baseCode !== undefined ? { baseCode: modifiers.baseCode } : {}),
    ...(modifiers.super !== undefined ? { super: modifiers.super } : {}),
  });
}

const sampleOptions: SelectOption[] = [
  { name: "Option 1", description: "First option" },
  { name: "Option 2", description: "Second option" },
  { name: "Option 3", description: "Third option" },
  { name: "Option 4", description: "Fourth option" },
  { name: "Option 5", description: "Fifth option" },
];

function createSelect(
  renderer: CliRenderer,
  options: ConstructorParameters<typeof Select>[1] = {},
) {
  const select = new Select(renderer, options);
  renderer.appendChild(renderer.rootNodeId, select.nodeId);
  return select;
}

describe("Select", () => {
  describe("Initialization", () => {
    it("initializes with default options", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, { options: sampleOptions });

      expect(select.options).toEqual(sampleOptions);
      expect(select.getSelectedIndex()).toBe(0);
      expect(select.getSelectedOption()).toEqual(sampleOptions[0]);
      expect(select.showScrollIndicator).toBe(false);
      expect(select.showDescription).toBe(true);
      expect(select.showSelectionIndicator).toBe(true);
      expect(select.wrapSelection).toBe(false);
      expect(select.fastScrollStep).toBe(5);

      cleanup();
    });

    it("initializes with a custom selected index", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, {
        options: sampleOptions,
        selectedIndex: 2,
      });

      expect(select.getSelectedIndex()).toBe(2);
      expect(select.getSelectedOption()).toEqual(sampleOptions[2]);

      cleanup();
    });

    it("clamps an out-of-range selected index", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, {
        options: sampleOptions,
        selectedIndex: 10,
      });

      expect(select.getSelectedIndex()).toBe(sampleOptions.length - 1);

      cleanup();
    });

    it("handles an empty options array", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, { options: [] });

      expect(select.options).toEqual([]);
      expect(select.getSelectedIndex()).toBe(0);
      expect(select.getSelectedOption()).toBeUndefined();
      expect(select.selectedIndex).toBe(0);

      cleanup();
    });

    it("skips non-selectable spacer/category options", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, {
        options: [
          { name: "Categories", description: "", value: { kind: "category" } },
          { name: "Option A", description: "" },
          { name: "Option B", description: "" },
        ],
      });

      expect(select.getSelectedIndex()).toBe(1);

      cleanup();
    });
  });

  describe("Options management", () => {
    it("updates options dynamically and clamps the selection", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, {
        options: sampleOptions,
        selectedIndex: 2,
      });

      const newOptions: SelectOption[] = [
        { name: "New 1", description: "First" },
        { name: "New 2", description: "Second" },
      ];
      select.options = newOptions;

      expect(select.options).toEqual(newOptions);
      expect(select.getSelectedIndex()).toBe(1);
      expect(select.getSelectedOption()).toEqual(newOptions[1]);

      cleanup();
    });

    it("handles setting empty options", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, {
        options: sampleOptions,
        selectedIndex: 2,
      });

      select.options = [];

      expect(select.options).toEqual([]);
      expect(select.getSelectedIndex()).toBe(0);
      expect(select.getSelectedOption()).toBeUndefined();

      cleanup();
    });
  });

  describe("Selection management", () => {
    it("moves up and down", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, {
        options: sampleOptions,
        selectedIndex: 2,
      });

      select.moveUp();
      expect(select.getSelectedIndex()).toBe(1);
      select.moveUp();
      expect(select.getSelectedIndex()).toBe(0);
      select.moveUp();
      expect(select.getSelectedIndex()).toBe(0);

      select.moveDown();
      expect(select.getSelectedIndex()).toBe(1);
      select.moveDown();
      expect(select.getSelectedIndex()).toBe(2);
      select.moveDown();
      select.moveDown();
      select.moveDown();
      expect(select.getSelectedIndex()).toBe(sampleOptions.length - 1);

      cleanup();
    });

    it("wraps selection when enabled", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, {
        options: sampleOptions,
        wrapSelection: true,
      });

      select.moveUp();
      expect(select.getSelectedIndex()).toBe(sampleOptions.length - 1);
      select.moveDown();
      expect(select.getSelectedIndex()).toBe(0);

      cleanup();
    });

    it("moves multiple steps", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, { options: sampleOptions });

      select.moveDown(3);
      expect(select.getSelectedIndex()).toBe(3);
      select.moveUp(2);
      expect(select.getSelectedIndex()).toBe(1);

      cleanup();
    });

    it("does not crash with empty options and wrap enabled", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, {
        options: [],
        wrapSelection: true,
      });

      expect(() => select.moveUp()).not.toThrow();
      expect(() => select.moveDown()).not.toThrow();
      expect(select.selectedIndex).toBe(0);

      cleanup();
    });

    it("sets the selected index programmatically and emits", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, { options: sampleOptions });

      let emittedIndex = -1;
      let emittedOption: SelectOption | undefined;
      select.on(SelectEvents.SELECTION_CHANGED, (index: number, opt: SelectOption) => {
        emittedIndex = index;
        emittedOption = opt;
      });

      select.setSelectedIndex(3);

      expect(select.getSelectedIndex()).toBe(3);
      expect(emittedIndex).toBe(3);
      expect(emittedOption).toEqual(sampleOptions[3]);

      cleanup();
    });

    it("selects the current item", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, {
        options: sampleOptions,
        selectedIndex: 2,
      });

      let selectedIndex = -1;
      let selectedOption: SelectOption | undefined;
      select.on(SelectEvents.ITEM_SELECTED, (index: number, opt: SelectOption) => {
        selectedIndex = index;
        selectedOption = opt;
      });

      select.selectCurrent();

      expect(selectedIndex).toBe(2);
      expect(selectedOption).toEqual(sampleOptions[2]);

      cleanup();
    });

    it("does not select when there are no options", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, { options: [] });

      let fired = false;
      select.on(SelectEvents.ITEM_SELECTED, () => {
        fired = true;
      });

      select.selectCurrent();
      expect(fired).toBe(false);

      cleanup();
    });
  });

  describe("Keyboard interaction", () => {
    it("handles arrow keys via handleKeyPress", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, {
        options: sampleOptions,
        selectedIndex: 1,
      });

      expect(select.handleKeyPress(createKeyEvent("down"))).toBe(true);
      expect(select.getSelectedIndex()).toBe(2);
      expect(select.handleKeyPress(createKeyEvent("up"))).toBe(true);
      expect(select.getSelectedIndex()).toBe(1);

      cleanup();
    });

    it("handles vim-style j/k keys", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, {
        options: sampleOptions,
        selectedIndex: 1,
      });

      expect(select.handleKeyPress(createKeyEvent("j"))).toBe(true);
      expect(select.getSelectedIndex()).toBe(2);
      expect(select.handleKeyPress(createKeyEvent("k"))).toBe(true);
      expect(select.getSelectedIndex()).toBe(1);

      cleanup();
    });

    it("handles enter/return/linefeed to select", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, {
        options: sampleOptions,
        selectedIndex: 2,
      });

      let itemSelected = false;
      select.on(SelectEvents.ITEM_SELECTED, () => {
        itemSelected = true;
      });

      expect(select.handleKeyPress(createKeyEvent("return"))).toBe(true);
      expect(itemSelected).toBe(true);

      itemSelected = false;
      expect(select.handleKeyPress(createKeyEvent("linefeed"))).toBe(true);
      expect(itemSelected).toBe(true);

      itemSelected = false;
      expect(select.handleKeyPress(createKeyEvent("enter"))).toBe(true);
      expect(itemSelected).toBe(true);

      cleanup();
    });

    it("handles fast scroll with shift modifier", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, {
        options: sampleOptions,
        fastScrollStep: 3,
      });

      expect(select.handleKeyPress(createKeyEvent("down", { shift: true }))).toBe(true);
      expect(select.getSelectedIndex()).toBe(3);
      expect(select.handleKeyPress(createKeyEvent("up", { shift: true }))).toBe(true);
      expect(select.getSelectedIndex()).toBe(0);

      cleanup();
    });

    it("handles pageup/pagedown and home/end", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, {
        options: sampleOptions,
        fastScrollStep: 2,
      });

      select.setSelectedIndex(2);
      select.handleKeyPress(createKeyEvent("pagedown"));
      expect(select.getSelectedIndex()).toBe(sampleOptions.length - 1);

      select.handleKeyPress(createKeyEvent("pageup"));
      expect(select.getSelectedIndex()).toBe(0);

      select.handleKeyPress(createKeyEvent("end"));
      expect(select.getSelectedIndex()).toBe(sampleOptions.length - 1);

      select.handleKeyPress(createKeyEvent("home"));
      expect(select.getSelectedIndex()).toBe(0);

      cleanup();
    });

    it("ignores unhandled keys", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, {
        options: sampleOptions,
        selectedIndex: 1,
      });

      expect(select.handleKeyPress(createKeyEvent("a"))).toBe(false);
      expect(select.getSelectedIndex()).toBe(1);

      cleanup();
    });

    it("matches bindings via Kitty baseCode for alternate layouts", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, {
        options: sampleOptions,
        selectedIndex: 1,
        keyBindings: [{ name: "j", action: "move-down" }],
      });

      // "ㅓ" with baseCode 106 (physical J key) should resolve to the "j" binding.
      const handled = select.handleKeyPress(createKeyEvent("ㅓ", { baseCode: 106 }));

      expect(handled).toBe(true);
      expect(select.getSelectedIndex()).toBe(2);

      cleanup();
    });
  });

  describe("Key bindings and aliases", () => {
    it("supports custom key bindings", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, {
        options: sampleOptions,
        keyBindings: [
          { name: "h", action: "move-up" },
          { name: "l", action: "move-down" },
        ],
      });

      expect(select.handleKeyPress(createKeyEvent("l"))).toBe(true);
      expect(select.getSelectedIndex()).toBe(1);
      expect(select.handleKeyPress(createKeyEvent("h"))).toBe(true);
      expect(select.getSelectedIndex()).toBe(0);

      cleanup();
    });

    it("merges custom bindings with defaults", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, {
        options: sampleOptions,
        keyBindings: [{ name: "n", action: "move-down" }],
      });

      // Default binding still works.
      expect(select.handleKeyPress(createKeyEvent("down"))).toBe(true);
      expect(select.getSelectedIndex()).toBe(1);
      // Custom binding works too.
      expect(select.handleKeyPress(createKeyEvent("n"))).toBe(true);
      expect(select.getSelectedIndex()).toBe(2);

      cleanup();
    });

    it("overrides default bindings with custom ones", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, {
        options: sampleOptions,
        keyBindings: [{ name: "k", action: "move-down" }],
      });

      // "k" now moves down instead of up.
      expect(select.handleKeyPress(createKeyEvent("k"))).toBe(true);
      expect(select.getSelectedIndex()).toBe(1);

      cleanup();
    });

    it("supports key aliases", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, {
        options: sampleOptions,
        keyBindings: [{ name: "q", action: "move-down" }],
        keyAliasMap: { q: "z" },
      });

      // Alias "z" resolves to the "q" binding.
      expect(select.handleKeyPress(createKeyEvent("z"))).toBe(true);
      expect(select.getSelectedIndex()).toBe(1);

      cleanup();
    });

    it("updates key bindings dynamically", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, { options: sampleOptions });

      select.keyBindings = [{ name: "x", action: "move-down" }];

      expect(select.handleKeyPress(createKeyEvent("x"))).toBe(true);
      expect(select.getSelectedIndex()).toBe(1);
      // Defaults remain merged.
      expect(select.handleKeyPress(createKeyEvent("down"))).toBe(true);
      expect(select.getSelectedIndex()).toBe(2);

      cleanup();
    });

    it("supports modifiers in custom bindings", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, {
        options: sampleOptions,
        selectedIndex: 2,
        keyBindings: [
          { name: "n", ctrl: true, action: "move-down" },
          { name: "p", ctrl: true, action: "move-up" },
        ],
      });

      expect(select.handleKeyPress(createKeyEvent("p", { ctrl: true }))).toBe(true);
      expect(select.getSelectedIndex()).toBe(1);
      expect(select.handleKeyPress(createKeyEvent("n", { ctrl: true }))).toBe(true);
      expect(select.getSelectedIndex()).toBe(2);

      cleanup();
    });
  });

  describe("Focus and dispatch", () => {
    it("focus/blur toggles focus state", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, { options: sampleOptions });

      expect(select.focused).toBe(false);
      select.focus();
      expect(select.focused).toBe(true);
      select.blur();
      expect(select.focused).toBe(false);

      cleanup();
    });

    it("navigates through the renderer key handler when focused", () => {
      const { renderer, mockInput, cleanup } = setup();
      const select = createSelect(renderer, { options: sampleOptions });
      select.focus();

      mockInput.pressArrow("down");
      expect(select.selectedIndex).toBe(1);
      mockInput.pressArrow("down");
      expect(select.selectedIndex).toBe(2);
      mockInput.pressArrow("up");
      expect(select.selectedIndex).toBe(1);

      cleanup();
    });

    it("does not react to keys when blurred", () => {
      const { renderer, mockInput, cleanup } = setup();
      const select = createSelect(renderer, { options: sampleOptions });

      mockInput.pressArrow("down");
      expect(select.selectedIndex).toBe(0);

      cleanup();
    });

    it("does not reuse the same keypress after focusing another select", () => {
      const { renderer, mockInput, cleanup } = setup();
      const first = createSelect(renderer, {
        options: sampleOptions,
        selectedIndex: 1,
      });
      const second = createSelect(renderer, {
        options: [
          { name: "A", description: "A" },
          { name: "B", description: "B" },
        ],
      });

      let firstSelections = 0;
      let secondSelections = 0;

      first.on(SelectEvents.ITEM_SELECTED, () => {
        firstSelections++;
        second.focus();
      });
      second.on(SelectEvents.ITEM_SELECTED, () => {
        secondSelections++;
      });

      first.focus();
      mockInput.pressKey("RETURN");

      expect(firstSelections).toBe(1);
      expect(secondSelections).toBe(0);
      expect(second.focused).toBe(true);

      cleanup();
    });
  });

  describe("Property changes", () => {
    it("updates toggles and colors without throwing", () => {
      const { renderer, cleanup } = setup();
      const select = createSelect(renderer, { options: sampleOptions });

      select.showScrollIndicator = true;
      expect(select.showScrollIndicator).toBe(true);
      select.showDescription = false;
      expect(select.showDescription).toBe(false);
      select.wrapSelection = true;
      expect(select.wrapSelection).toBe(true);
      select.fastScrollStep = 10;
      expect(select.fastScrollStep).toBe(10);
      select.selectionIndicator = "> ";
      expect(select.selectionIndicator).toBe("> ");
      select.unselectedIndicator = "  ";
      expect(select.unselectedIndicator).toBe("  ");

      select.textColor = "#ff0000";
      select.focusedTextColor = "#00ff00";
      select.selectedTextColor = "#0000ff";
      select.selectedBackgroundColor = "#ffff00";
      select.descriptionColor = "#888888";
      select.selectedDescriptionColor = "#cccccc";
      select.focusedBackgroundColor = "#333333";

      expect(select.focusedBackgroundColor).toBeDefined();

      cleanup();
    });
  });
});
