#!/usr/bin/env bun

import {
  ASCIIFont,
  Box,
  type CliRenderer,
  Input,
  InputEvents,
  type KeyEvent,
  type LogLevel,
  RenderableEvents,
  Select,
  SelectEvents,
  type SelectOption,
  Text,
  TimeToFirstDraw,
  createCliRenderer,
} from "@bettertui/core";
import {
  CATEGORY_LABELS,
  DEFAULT_THEME_MODE,
  EXAMPLES_BOX_TITLE,
  EXAMPLES_INDENT,
  MENU_TERMINAL_TITLE,
  MENU_THEMES,
} from "./constants";
import * as asciiFontSelectionExample from "./examples/asciiFontSelection.example";
import * as audioStreamingDemo from "./examples/audioStreaming.example";
import * as clipboardPasteDemo from "./examples/clipboardPaste.example";
import * as codeDemo from "./examples/code.example";
import * as consoleExample from "./examples/console.example";
import * as corePluginSlotsDemo from "./examples/corePluginSlots.example";
import * as diffDemo from "./examples/diff.example";
import * as editorDemo from "./examples/editor.example";
import * as extmarksDemo from "./examples/extmarks.example";
import * as focusRestoreDemo from "./examples/focusRestore.example";
import * as boxExample from "./examples/fonts.example";
import * as framebufferExample from "./examples/framebuffer.example";
import * as fullUnicodeExample from "./examples/fullUnicode.example";
import * as grayscaleBufferDemo from "./examples/grayscaleBuffer.example";
import * as hastSyntaxHighlightingExample from "./examples/hastSyntaxHighlighting.example";
import * as inputExample from "./examples/input.example";
import * as inputSelectLayoutExample from "./examples/inputSelectLayout.example";
import * as keymapDemo from "./examples/keymap.example";
import * as keypressDebugDemo from "./examples/keypressDebug.example";
import * as linkDemo from "./examples/link.example";
import * as liveStateExample from "./examples/liveState.example";
import * as markdownDemo from "./examples/markdown.example";
import * as markdownCodeBlockRendererDemo from "./examples/markdownCodeBlockRenderer.example";
import * as mouseInteractionExample from "./examples/mouseInteraction.example";
import * as multitabDemo from "./examples/multitab.example";
import * as nativeAudioDemo from "./examples/nativeAudio.example";
import * as nestedZIndexDemo from "./examples/nestedZindex.example";
import * as notificationDemo from "./examples/notification.example";
import * as opacityExample from "./examples/opacityExample.example";
import * as qrcodeDemo from "./examples/qrcode.example";
import * as relativePositioningDemo from "./examples/relativePositioning.example";
import * as scrollExample from "./examples/scrollExample.example";
import * as scrollboxMouseTest from "./examples/scrollboxMouseTest.example";
import * as scrollboxOverlayHitTest from "./examples/scrollboxOverlayHitTest.example";
import * as selectExample from "./examples/select.example";
import * as layoutExample from "./examples/simpleLayoutExample.example";
import * as sliderDemo from "./examples/slider.example";
import * as splitFooterStreamingDemo from "./examples/splitFooterStreaming.example";
import * as splitModeExample from "./examples/splitMode.example";
import * as stickyScrollExample from "./examples/stickyScrollExample.example";
import * as styledTextExample from "./examples/styledText.example";
import * as tabSelectExample from "./examples/tabSelect.example";
import * as terminalDemo from "./examples/terminal.example";
import * as terminalTitleDemo from "./examples/terminalTitle.example";
import * as textNodeDemo from "./examples/textNode.example";
import * as textSelectionExample from "./examples/textSelection.example";
import * as textTableExample from "./examples/textTable.example";
import * as textTruncationDemo from "./examples/textTruncation.example";
import * as textWrapExample from "./examples/textWrap.example";
import * as timelineExample from "./examples/timelineExample.example";
import * as transparencyDemo from "./examples/transparency.example";
import * as vnodeCompositionDemo from "./examples/vnodeComposition.example";
import * as wideGraphemeOverlayDemo from "./examples/wideGraphemeOverlay.example";
import { setupCommonDemoKeys } from "./lib/standaloneKeys";
import type {
  Example,
  ExampleDefinition,
  ExampleMenuValue,
  ExampleSection,
  MenuFocusArea,
  MenuOption,
  MenuOptionValue,
  ThemeMode,
} from "./types/exampleList.types";

import type { ExampleCategory } from "./types/exampleList.types";

function sortExampleDefinitions(examples: readonly ExampleDefinition[]): ExampleDefinition[] {
  return [...examples].sort((left, right) => left.name.localeCompare(right.name));
}

function section(
  category: ExampleCategory,
  examples: readonly ExampleDefinition[],
): ExampleSection {
  return {
    category,
    examples: sortExampleDefinitions(examples),
  };
}

const EXAMPLE_SECTIONS: ExampleSection[] = [
  section("Layout & Composition", [
    {
      name: "Input & Select Layout Demo",
      description: "Interactive layout with input and select elements",
      run: inputSelectLayoutExample.run,
      destroy: inputSelectLayoutExample.destroy,
    },
    {
      name: "Layout System Demo",
      description: "Flex layout system with multiple configurations",
      run: layoutExample.run,
      destroy: layoutExample.destroy,
    },
    {
      name: "Nested Z-Index Demo",
      description: "Demonstrates z-index behavior with nested render objects",
      run: nestedZIndexDemo.run,
      destroy: nestedZIndexDemo.destroy,
    },
    {
      name: "BetterTUI Demo",
      description: "Multi-tab demo with various features",
      run: multitabDemo.run,
      destroy: multitabDemo.destroy,
    },
    {
      name: "Relative Positioning Demo",
      description: "Shows how child positions are relative to their parent containers",
      run: relativePositioningDemo.run,
      destroy: relativePositioningDemo.destroy,
    },
    {
      name: "Split Footer Streaming Demo",
      description:
        "Focused split-footer surface demo for progressive text, code, and markdown scrollback",
      run: splitFooterStreamingDemo.run,
      destroy: splitFooterStreamingDemo.destroy,
    },
    {
      name: "Split Mode Demo (Experimental)",
      description: "Renderer confined to bottom area with normal terminal output above",
      run: splitModeExample.run,
      destroy: splitModeExample.destroy,
    },
    {
      name: "VNode Composition Demo",
      description: "Declarative Box(Box(Box(children))) composition",
      run: vnodeCompositionDemo.run,
      destroy: vnodeCompositionDemo.destroy,
    },
  ]),
  section("Input & Editing", [
    {
      name: "ASCII Font Selection Demo",
      description:
        "Text selection with ASCII fonts - precise character-level selection across different font types",
      run: asciiFontSelectionExample.run,
      destroy: asciiFontSelectionExample.destroy,
    },
    {
      name: "Editor Demo",
      description: "Interactive text editor with Textarea - supports full editing capabilities",
      run: editorDemo.run,
      destroy: editorDemo.destroy,
    },
    {
      name: "Extmarks Demo",
      description:
        "Virtual extmarks - text ranges that the cursor jumps over, with deletion handling",
      run: extmarksDemo.run,
      destroy: extmarksDemo.destroy,
    },
    {
      name: "Input Demo",
      description: "Interactive InputElement demo with validation and multiple fields",
      run: inputExample.run,
      destroy: inputExample.destroy,
    },
    {
      name: "Keymap Demo",
      description:
        "Global and local bindings with counters, leader commands, a centered : prompt, and three switchable textareas",
      run: keymapDemo.run,
      destroy: keymapDemo.destroy,
    },
    {
      name: "Mouse Interaction Demo",
      description: "Interactive mouse trails and clickable cells demonstration",
      run: mouseInteractionExample.run,
      destroy: mouseInteractionExample.destroy,
    },
    {
      name: "Select Demo",
      description: "Interactive SelectElement demo with customizable options",
      run: selectExample.run,
      destroy: selectExample.destroy,
    },
    {
      name: "Slider Demo",
      description: "Interactive slider components with various orientations and configurations",
      run: sliderDemo.run,
      destroy: sliderDemo.destroy,
    },
    {
      name: "Tab Select",
      description: "Tab selection demo",
      run: tabSelectExample.run,
      destroy: tabSelectExample.destroy,
    },
    {
      name: "Text Selection Demo",
      description: "Text selection across multiple renderables with mouse drag",
      run: textSelectionExample.run,
      destroy: textSelectionExample.destroy,
    },
  ]),
  section("Scroll & Navigation", [
    {
      name: "ScrollBox Demo",
      description: "Scrollable container with customization",
      run: scrollExample.run,
      destroy: scrollExample.destroy,
    },
    {
      name: "Scrollbox Mouse Test",
      description: "Test scrollbox mouse hit detection with hover and click events",
      run: scrollboxMouseTest.run,
      destroy: scrollboxMouseTest.destroy,
    },
    {
      name: "Scrollbox Overlay Hit Test",
      description: "Test scrollbox hit detection with overlays and dialogs",
      run: scrollboxOverlayHitTest.run,
      destroy: scrollboxOverlayHitTest.destroy,
    },
    {
      name: "Sticky Scroll Demo",
      description:
        "ScrollBox with sticky scroll behavior - maintains position at borders when content changes",
      run: stickyScrollExample.run,
      destroy: stickyScrollExample.destroy,
    },
  ]),
  section("Text & Documents", [
    {
      name: "ASCII Font Demo",
      description: "ASCII font rendering with various colors and text",
      run: boxExample.run,
      destroy: boxExample.destroy,
    },
    {
      name: "Code Demo",
      description:
        "Code viewer with line numbers, diff highlights, and diagnostics using Code + LineNumber",
      run: codeDemo.run,
      destroy: codeDemo.destroy,
    },
    {
      name: "Diff Demo",
      description: "Unified and split diff views with syntax highlighting and multiple themes",
      run: diffDemo.run,
      destroy: diffDemo.destroy,
    },
    {
      name: "Full Unicode Demo",
      description: "Draggable boxes and background filled with complex graphemes",
      run: fullUnicodeExample.run,
      destroy: fullUnicodeExample.destroy,
    },
    {
      name: "HAST Syntax Highlighting Demo",
      description: "Convert HAST trees to syntax-highlighted text with efficient chunk generation",
      run: hastSyntaxHighlightingExample.run,
      destroy: hastSyntaxHighlightingExample.destroy,
    },
    {
      name: "Link Demo",
      description:
        "Hyperlink support with OSC 8 - clickable links and link inheritance in styled text",
      run: linkDemo.run,
      destroy: linkDemo.destroy,
    },
    {
      name: "Markdown Demo",
      description:
        "Markdown rendering with table alignment, syntax highlighting, and theme switching",
      run: markdownDemo.run,
      destroy: markdownDemo.destroy,
    },
    {
      name: "Markdown Code Block Renderer Demo",
      description: "Custom fenced-code rendering for a fake taskflow DSL inside markdown",
      run: markdownCodeBlockRendererDemo.run,
      destroy: markdownCodeBlockRendererDemo.destroy,
    },
    {
      name: "QR Code Demo",
      description:
        "Intrinsic QR code renderable with manual scaling and terminal-friendly half-block output",
      run: qrcodeDemo.run,
      destroy: qrcodeDemo.destroy,
    },
    {
      name: "Styled Text Demo",
      description: "Template literals with styled text, colors, and formatting",
      run: styledTextExample.run,
      destroy: styledTextExample.destroy,
    },
    {
      name: "Text Truncation Demo",
      description:
        "Middle truncation with ellipsis - toggle with 'T' key and resize to test responsive behavior",
      run: textTruncationDemo.run,
      destroy: textTruncationDemo.destroy,
    },
    {
      name: "Text Wrap Demo",
      description: "Text wrapping example",
      run: textWrapExample.run,
      destroy: textWrapExample.destroy,
    },
    {
      name: "TextNode Demo",
      description: "TextNode API for building complex styled text structures",
      run: textNodeDemo.run,
      destroy: textNodeDemo.destroy,
    },
    {
      name: "TextTable Demo",
      description:
        "TextTable renderable with styled chunks, Unicode content, and wrap/border toggles",
      run: textTableExample.run,
      destroy: textTableExample.destroy,
    },
    {
      name: "Wide Grapheme Overlay Demo",
      description: "Drag transparent boxes over CJK/emoji, toggle dimming scrim with D key",
      run: wideGraphemeOverlayDemo.run,
      destroy: wideGraphemeOverlayDemo.destroy,
    },
  ]),
  section("Rendering & Effects", [
    {
      name: "Framebuffer Demo",
      description: "Framebuffer rendering techniques",
      run: framebufferExample.run,
      destroy: framebufferExample.destroy,
    },
    {
      name: "Grayscale Buffer",
      description: "Grayscale buffer rendering with 1x vs 2x supersampled intensity",
      run: grayscaleBufferDemo.run,
      destroy: grayscaleBufferDemo.destroy,
    },
    {
      name: "Opacity Demo",
      description: "Box opacity and transparency effects with animated opacity transitions",
      run: opacityExample.run,
      destroy: opacityExample.destroy,
    },
    {
      name: "Timeline Example",
      description: "Animation timeline system",
      run: timelineExample.run,
      destroy: timelineExample.destroy,
    },
    {
      name: "Transparency Demo",
      description: "Alpha blending and transparency effects demonstration",
      run: transparencyDemo.run,
      destroy: transparencyDemo.destroy,
    },
  ]),
  section("Runtime & Tooling", [
    {
      name: "Console Demo",
      description: "Interactive console logging with clickable buttons for different log levels",
      run: consoleExample.run,
      destroy: consoleExample.destroy,
    },
    {
      name: "Core Plugin Slots Demo",
      description: "Framework-free plugin slots with cached renderables and deterministic ordering",
      run: corePluginSlotsDemo.run,
      destroy: corePluginSlotsDemo.destroy,
    },
    {
      name: "Live State Management Demo",
      description: "Test automatic renderer lifecycle management with live renderables",
      run: liveStateExample.run,
      destroy: liveStateExample.destroy,
    },
  ]),
  section("Terminal & Native", [
    {
      name: "Audio Streaming Demo",
      description:
        "Live MP3 URL streaming with reconnect controls, telemetry, and master-mix FFT visualization",
      run: audioStreamingDemo.run,
      destroy: audioStreamingDemo.destroy,
    },
    {
      name: "Audio Demo",
      description: "WAV-based native mixer with sound groups and live meter stats",
      run: nativeAudioDemo.run,
      destroy: nativeAudioDemo.destroy,
    },
    {
      name: "Clipboard & Paste Test Bed",
      description:
        "OSC 52 copy, paste transport, and editor semantics diagnostics with a selectable, copyable event log",
      run: clipboardPasteDemo.run,
      destroy: clipboardPasteDemo.destroy,
    },
    {
      name: "Focus Restore Demo",
      description: "Test focus restore - alt-tab away and back to verify mouse tracking resumes",
      run: focusRestoreDemo.run,
      destroy: focusRestoreDemo.destroy,
    },
    {
      name: "Keypress Debug Tool",
      description: "Debug tool to inspect keypress events, raw input, and terminal capabilities",
      run: keypressDebugDemo.run,
      destroy: keypressDebugDemo.destroy,
    },
    {
      name: "Notification Demo",
      description:
        "Standalone OSC terminal notification demo with capability detection and interactive triggers",
      run: notificationDemo.run,
      destroy: notificationDemo.destroy,
    },
    {
      name: "Terminal Palette Demo",
      description:
        "Terminal color palette detection and visualization - fetch and display all 256 terminal colors",
      run: terminalDemo.run,
      destroy: terminalDemo.destroy,
    },
    {
      name: "Terminal Title Demo",
      description: "Set and update the terminal window title with OSC title sequences",
      run: terminalTitleDemo.run,
      destroy: terminalTitleDemo.destroy,
    },
  ]),
];

export const examples: Example[] = EXAMPLE_SECTIONS.flatMap(({ category, examples }) =>
  examples.map((example) => ({
    ...example,
    category,
  })),
);

function createMenuOptions(filteredExamples: readonly Example[]): MenuOption[] {
  if (filteredExamples.length === 0) {
    return [
      {
        name: "No matching examples",
        description: "Try a broader filter or press Escape to clear it.",
        value: { kind: "message" },
      },
    ];
  }

  const options: MenuOption[] = [];
  let shouldInsertSectionGap = false;

  for (const section of EXAMPLE_SECTIONS) {
    const sectionExamples = filteredExamples.filter(
      (example) => example.category === section.category,
    );
    if (sectionExamples.length === 0) {
      continue;
    }

    if (shouldInsertSectionGap) {
      options.push({
        name: "",
        description: "",
        value: { kind: "spacer" },
      });
    }

    shouldInsertSectionGap = true;

    options.push({
      name: CATEGORY_LABELS[section.category].toUpperCase(),
      description: "",
      value: { kind: "category", category: section.category },
    });

    options.push({
      name: "",
      description: "",
      value: { kind: "spacer" },
    });

    for (const example of sectionExamples) {
      options.push({
        name: example.name,
        description: `${EXAMPLES_INDENT}${example.description}`,
        value: { kind: "example", example },
      });
    }
  }

  return options;
}

function matchesExample(example: Example, filterText: string): boolean {
  const searchableText =
    `${example.category}\n${CATEGORY_LABELS[example.category]}\n${example.name}\n${example.description}`.toLowerCase();
  return searchableText.includes(filterText);
}

function isExampleMenuValue(value: MenuOptionValue | undefined): value is ExampleMenuValue {
  return value?.kind === "example";
}

function getExampleFromOption(option: SelectOption | null): Example | null {
  const menuOption = option as MenuOption | null;
  return isExampleMenuValue(menuOption?.value) ? menuOption.value.example : null;
}

function getFirstExampleOptionIndex(options: readonly MenuOption[]): number {
  for (let index = 0; index < options.length; index += 1) {
    if (isExampleMenuValue(options[index]?.value)) {
      return index;
    }
  }

  return -1;
}

function getExampleOptionIndexByName(options: readonly MenuOption[], name: string | null): number {
  if (!name) {
    return -1;
  }

  for (let index = 0; index < options.length; index += 1) {
    const optionValue = options[index]?.value;
    if (isExampleMenuValue(optionValue) && optionValue.example.name === name) {
      return index;
    }
  }

  return -1;
}

function getExamplesBoxTitle(filteredCount: number, isFiltered: boolean): string {
  if (!isFiltered || filteredCount > 0) {
    return EXAMPLES_BOX_TITLE;
  }

  return `${EXAMPLES_BOX_TITLE} (No Matches)`;
}

function getPrintableKeyText(key: KeyEvent): string | null {
  if (key.ctrl || key.meta) {
    return null;
  }

  if (key.name === "space") {
    return " ";
  }

  if (!key.sequence || Array.from(key.sequence).length !== 1) {
    return null;
  }

  const firstCharCode = key.sequence.charCodeAt(0);
  if (firstCharCode < 32 || firstCharCode === 127) {
    return null;
  }

  return key.sequence;
}

function findNearestExampleOptionIndex(
  options: readonly MenuOption[],
  startIndex: number,
  direction: -1 | 1,
  wrap: boolean,
): number {
  if (options.length === 0) {
    return -1;
  }

  let index = startIndex;

  for (let attempts = 0; attempts < options.length; attempts += 1) {
    if (index < 0 || index >= options.length) {
      if (!wrap) {
        return -1;
      }

      index = index < 0 ? options.length - 1 : 0;
    }

    if (isExampleMenuValue(options[index]?.value)) {
      return index;
    }

    index += direction;
  }

  return -1;
}

export class ExampleSelector {
  private renderer: CliRenderer;
  private currentExample: Example | null = null;
  private inMenu = true;
  private themeMode: ThemeMode = DEFAULT_THEME_MODE;

  private menuContainer: Box | null = null;
  private title: ASCIIFont | null = null;
  private titleWidth = 0;
  private titleFont = "tiny";
  private titleText = "BETTERTUI EXAMPLES";
  private filterBox: Box | null = null;
  private filterInput: Input | null = null;
  private instructions: Text | null = null;
  private timeToFirstDrawText: TimeToFirstDraw | null = null;
  private selectElement: Select | null = null;
  private selectBox: Box | null = null;
  private notImplementedText: Text | null = null;
  private readonly allExamples: Example[] = examples;
  private selectedExampleName: string | null = examples[0]?.name ?? null;
  private menuFocusArea: MenuFocusArea = "filter";
  private filterText = "";

  constructor(renderer: CliRenderer) {
    this.renderer = renderer;
    this.themeMode = this.renderer.themeMode ?? DEFAULT_THEME_MODE;
    this.renderer.setTerminalTitle(MENU_TERMINAL_TITLE);
    this.createLayout();
    this.setupKeyboardHandling();

    this.renderer.on("theme_mode", (mode: ThemeMode) => {
      this.applyTheme(mode);
      console.log(`Theme mode changed to ${mode}, applied new theme to menu`);
    });

    this.applyTheme(this.renderer.themeMode);

    this.renderer.on("resize", (width: number, height: number) => {
      this.handleResize(width, height);
    });
  }

  private createLayout(): void {
    const theme = MENU_THEMES[this.themeMode].components;

    // Menu container with column layout
    this.menuContainer = new Box(this.renderer, {
      id: "example-menu-container",
      flexDirection: "column",
      width: "100%",
      height: "100%",
      backgroundColor: theme.appBackground,
    });
    this.renderer.root.add(this.menuContainer);

    // Title
    const titleText = this.titleText;
    const titleFont = this.titleFont;

    this.title = new ASCIIFont(this.renderer, {
      id: "example-index-title",
      alignSelf: "center",
      marginTop: 1,
      marginBottom: 1,
      text: titleText,
      font: titleFont,
      color: theme.title,
      backgroundColor: "transparent",
    });
    this.menuContainer.add(this.title);

    // Filter box with border (grows with content)
    this.filterBox = new Box(this.renderer, {
      id: "example-index-filter-box",
      marginLeft: 1,
      marginRight: 1,
      flexShrink: 0,
      backgroundColor: "transparent",
      border: true,
      borderStyle: "single",
      borderColor: theme.border,
    });
    this.menuContainer.add(this.filterBox);

    // Filter input inside the box (transparent bg so box bg shows through)
    this.filterInput = new Input(this.renderer, {
      id: "example-index-filter-input",
      width: "100%",
      placeholder: "Filter examples...",
      placeholderColor: theme.inputPlaceholder,
      backgroundColor: "transparent",
      focusedBackgroundColor: "transparent",
      textColor: theme.inputText,
      focusedTextColor: theme.inputFocusedText,
      showCursor: true,
      cursorColor: theme.inputCursor,
    });
    this.filterBox.add(this.filterInput);

    this.filterInput.on(InputEvents.INPUT, (value: string) => {
      this.filterText = value;
      this.filterExamples();
    });

    // Select box (grows to fill remaining space)
    this.selectBox = new Box(this.renderer, {
      id: "example-selector-box",
      marginLeft: 1,
      marginRight: 1,
      marginBottom: 1,
      flexGrow: 1,
      borderStyle: "single",
      borderColor: theme.border,
      focusedBorderColor: theme.focusedBorder,
      title: EXAMPLES_BOX_TITLE,
      titleAlignment: "center",
      backgroundColor: "transparent",
      border: true,
    });
    this.menuContainer.add(this.selectBox);

    // Select element
    const selectOptions = createMenuOptions(this.allExamples);
    const initialSelectedIndex = Math.max(0, getFirstExampleOptionIndex(selectOptions));

    this.selectElement = new Select(this.renderer, {
      id: "example-selector",
      height: "100%",
      options: selectOptions,
      selectedIndex: initialSelectedIndex,
      backgroundColor: "transparent",
      focusedBackgroundColor: "transparent",
      focusedTextColor: theme.selectText,
      selectedBackgroundColor: theme.selectSelectedBackground,
      textColor: theme.selectText,
      selectedTextColor: theme.selectSelectedText,
      descriptionColor: theme.selectDescription,
      selectedDescriptionColor: theme.selectSelectedDescription,
      showScrollIndicator: true,
      wrapSelection: false,
      showDescription: true,
      fastScrollStep: 5,
    });
    this.selectBox.add(this.selectElement);

    this.filterInput.on(RenderableEvents.FOCUSED, () => {
      this.menuFocusArea = "filter";
      this.syncFilterInputText();
      this.updateMenuFocusStyles();
    });

    this.selectElement.on(RenderableEvents.FOCUSED, () => {
      this.menuFocusArea = "list";
      this.updateMenuFocusStyles();
    });

    this.selectElement.on(SelectEvents.SELECTION_CHANGED, (index: number, option: SelectOption) => {
      const selectedExample = getExampleFromOption(option);
      if (!selectedExample) {
        this.focusNearestExampleOption(index, 1);
        return;
      }

      this.selectedExampleName = selectedExample.name;
    });

    this.selectElement.on(SelectEvents.ITEM_SELECTED, (index: number, option: SelectOption) => {
      const selectedExample = getExampleFromOption(option);
      if (!selectedExample) {
        this.focusNearestExampleOption(index, 1);
        return;
      }

      void this.runSelected(selectedExample);
    });

    this.setMenuFocus("filter");

    this.timeToFirstDrawText = new TimeToFirstDraw(this.renderer, {
      id: "example-index-time-to-first-draw",
      alignSelf: "center",
      fg: theme.instructions,
    });
    this.menuContainer.add(this.timeToFirstDrawText);

    // Instructions at the bottom
    this.instructions = new Text(this.renderer, {
      id: "example-index-instructions",
      height: 1,
      flexShrink: 0,
      alignSelf: "center",
      content:
        "Tab/Esc switch focus | Type in filter | ↑↓/j/k list | Enter run | / filter | ctrl+c quit",
      fg: theme.instructions,
    });
    this.menuContainer.add(this.instructions);
  }

  private applyTheme(mode: ThemeMode | null): void {
    this.themeMode = mode ?? DEFAULT_THEME_MODE;
    const theme = MENU_THEMES[this.themeMode].components;

    this.renderer.setBackgroundColor(theme.appBackground);
    if (this.menuContainer) {
      this.menuContainer.backgroundColor = theme.appBackground;
    }

    if (this.title) {
      this.title.color = theme.title;
    }

    if (this.filterInput) {
      this.filterInput.textColor = theme.inputText;
      this.filterInput.focusedTextColor = theme.inputFocusedText;
      this.filterInput.placeholderColor = theme.inputPlaceholder;
      this.filterInput.cursorColor = theme.inputCursor;
    }

    if (this.filterBox) {
      this.filterBox.borderColor = theme.border;
    }

    if (this.selectBox) {
      this.selectBox.focusedBorderColor = theme.focusedBorder;
    }

    if (this.selectElement) {
      this.selectElement.selectedBackgroundColor = theme.selectSelectedBackground;
      this.selectElement.textColor = theme.selectText;
      this.selectElement.focusedTextColor = theme.selectText;
      this.selectElement.selectedTextColor = theme.selectSelectedText;
      this.selectElement.descriptionColor = theme.selectDescription;
      this.selectElement.selectedDescriptionColor = theme.selectSelectedDescription;
    }

    if (this.instructions) {
      this.instructions.fg = theme.instructions;
    }

    if (this.timeToFirstDrawText) {
      this.timeToFirstDrawText.fg = theme.instructions;
    }

    if (this.notImplementedText) {
      this.notImplementedText.fg = theme.notImplemented;
    }

    this.updateMenuFocusStyles();
    this.renderer.requestRender();
  }

  private setMenuFocus(focusArea: MenuFocusArea): void {
    this.menuFocusArea = focusArea;

    if (focusArea === "filter") {
      this.selectElement?.blur();
      this.syncFilterInputText();
      this.filterInput?.focus();
    } else {
      this.filterInput?.blur();
      this.selectElement?.focus();
    }

    this.updateMenuFocusStyles();
  }

  private updateMenuFocusStyles(): void {
    const theme = MENU_THEMES[this.themeMode].components;

    if (this.filterBox) {
      this.filterBox.borderColor =
        this.menuFocusArea === "filter" ? theme.focusedBorder : theme.border;
    }

    if (this.selectBox) {
      this.selectBox.focusedBorderColor =
        this.menuFocusArea === "list" ? theme.focusedBorder : theme.border;
    }
  }

  private clearFilter(): void {
    if (!this.filterInput || this.filterText.length === 0) {
      return;
    }

    this.filterText = "";
    this.filterInput.value = "";
    this.filterInput.cursorOffset = 0;
  }

  private syncFilterInputText(): void {
    if (!this.filterInput || this.filterInput.plainText === this.filterText) {
      return;
    }

    this.filterInput.value = this.filterText;
    this.filterInput.cursorOffset = this.filterInput.plainText.length;
  }

  private updateSelectOptions(filteredExamples: readonly Example[]): void {
    if (!this.selectElement) {
      return;
    }

    if (this.selectBox) {
      this.selectBox.title = getExamplesBoxTitle(
        filteredExamples.length,
        this.filterText.trim().length > 0,
      );
    }

    const options = createMenuOptions(filteredExamples);
    this.selectElement.options = options;

    if (options.length === 0) {
      return;
    }

    const selectedIndex = getExampleOptionIndexByName(options, this.selectedExampleName);
    const nextIndex = selectedIndex >= 0 ? selectedIndex : getFirstExampleOptionIndex(options);

    if (nextIndex < 0) {
      return;
    }

    this.setSelectedOptionIndex(nextIndex);
  }

  private setSelectedOptionIndex(index: number): void {
    if (!this.selectElement) {
      return;
    }

    this.selectElement.selectedIndex = index;
    const option = (this.selectElement.options as MenuOption[])[index] ?? null;
    this.selectedExampleName = getExampleFromOption(option)?.name ?? this.selectedExampleName;
  }

  private focusNearestExampleOption(startIndex: number, direction: -1 | 1): void {
    if (!this.selectElement) {
      return;
    }

    const options = this.selectElement.options as MenuOption[];
    const nextIndex = findNearestExampleOptionIndex(
      options,
      startIndex + direction,
      direction,
      this.selectElement.wrapSelection,
    );

    if (nextIndex >= 0) {
      this.setSelectedOptionIndex(nextIndex);
      return;
    }

    const fallbackIndex = findNearestExampleOptionIndex(
      options,
      startIndex - direction,
      direction === 1 ? -1 : 1,
      this.selectElement.wrapSelection,
    );

    if (fallbackIndex >= 0) {
      this.setSelectedOptionIndex(fallbackIndex);
    }
  }

  private moveSelection(direction: -1 | 1, steps: number): void {
    if (!this.selectElement) {
      return;
    }

    const options = this.selectElement.options as MenuOption[];
    if (options.length === 0) {
      return;
    }

    let currentIndex = this.selectElement.getSelectedIndex();

    for (let step = 0; step < steps; step += 1) {
      const nextIndex = findNearestExampleOptionIndex(
        options,
        currentIndex + direction,
        direction,
        this.selectElement.wrapSelection,
      );

      if (nextIndex < 0) {
        break;
      }

      currentIndex = nextIndex;
    }

    this.setSelectedOptionIndex(currentIndex);
  }

  private filterExamples(): void {
    if (!this.filterInput || !this.selectElement) return;

    const filterText = this.filterText.toLowerCase().trim();

    if (filterText === "") {
      this.updateSelectOptions(this.allExamples);
    } else {
      const filtered = this.allExamples.filter((example) => matchesExample(example, filterText));
      this.updateSelectOptions(filtered);
    }
  }

  private handleResize(_width: number, _height: number): void {
    this.renderer.requestRender();
  }

  private setupKeyboardHandling(): void {
    this.renderer.keyInput.on("keypress", (key: KeyEvent) => {
      if (key.name === "c" && key.ctrl) {
        this.cleanup();
        return;
      }

      if (!this.inMenu) {
        switch (key.name) {
          case "escape":
            this.returnToMenu();
            break;
        }
        return;
      }

      if (key.name === "tab" || key.name === "escape") {
        key.preventDefault();
        this.setMenuFocus(this.menuFocusArea === "filter" ? "list" : "filter");
        return;
      }

      const printableText = getPrintableKeyText(key);

      if (this.menuFocusArea === "list") {
        if (printableText === "/") {
          key.preventDefault();
          this.setMenuFocus("filter");
          return;
        }
      }

      if (this.menuFocusArea === "filter" && this.selectElement) {
        if (key.name === "up") {
          key.preventDefault();
          this.moveSelection(-1, key.shift ? 5 : 1);
          return;
        }

        if (key.name === "down") {
          key.preventDefault();
          this.moveSelection(1, key.shift ? 5 : 1);
          return;
        }

        if (key.name === "return" || key.name === "linefeed" || key.name === "enter") {
          key.preventDefault();
          this.selectElement.selectCurrent();
          return;
        }
      }

      if (key.name === "c" && key.ctrl) {
        this.cleanup();
        return;
      }
      switch (key.name) {
        case "c":
          console.log("Capabilities:", this.renderer.capabilities);
          break;
        case "z":
          if (key.ctrl) {
            console.log("Suspending renderer... (will auto-resume in 5 seconds)");
            this.renderer.suspend();
            setTimeout(() => {
              console.log("Resuming renderer...");
              this.renderer.resume();
            }, 5000);
          }
          break;
      }
    });
    setupCommonDemoKeys(this.renderer);
  }

  private async runSelected(selected: Example): Promise<void> {
    this.inMenu = false;
    this.hideMenuElements();

    if (selected.run) {
      this.currentExample = selected;
      await selected.run(this.renderer);
    } else {
      if (!this.notImplementedText) {
        const theme = MENU_THEMES[this.themeMode].components;
        const unavailableMessage =
          selected.unavailableMessage ?? `${selected.name} is not implemented yet.`;
        this.notImplementedText = new Text(this.renderer, {
          id: "not-implemented",
          position: "absolute",
          left: 10,
          top: 10,
          content: `${unavailableMessage} Press Escape to return.`,
          fg: theme.notImplemented,
          zIndex: 10,
        });
        this.renderer.root.add(this.notImplementedText);
      }
      this.renderer.requestRender();
    }
  }

  private hideMenuElements(): void {
    if (this.menuContainer) {
      this.menuContainer.visible = false;
    }
    if (this.title) {
      this.title.visible = false;
    }
    if (this.filterBox) {
      this.filterBox.visible = false;
    }
    if (this.selectBox) {
      this.selectBox.visible = false;
    }
    if (this.instructions) {
      this.instructions.visible = false;
    }
    if (this.timeToFirstDrawText) {
      this.timeToFirstDrawText.visible = false;
    }
    if (this.filterInput) {
      this.filterInput.blur();
    }
    if (this.selectElement) {
      this.selectElement.blur();
    }
  }

  private showMenuElements(): void {
    this.renderer.setTerminalTitle(MENU_TERMINAL_TITLE);

    if (this.menuContainer) {
      this.menuContainer.visible = true;
    }
    if (this.title) {
      this.title.visible = true;
    }
    if (this.filterBox) {
      this.filterBox.visible = true;
    }
    if (this.selectBox) {
      this.selectBox.visible = true;
    }
    if (this.instructions) {
      this.instructions.visible = true;
    }
    if (this.timeToFirstDrawText) {
      this.timeToFirstDrawText.visible = true;
    }

    this.clearFilter();
    this.setMenuFocus("filter");
  }

  private returnToMenu(): void {
    if (this.currentExample) {
      this.currentExample.destroy?.(this.renderer);
      this.currentExample = null;
    }

    if (this.notImplementedText) {
      this.renderer.root.remove(this.notImplementedText);
      this.notImplementedText = null;
    }

    this.inMenu = true;
    this.restart();
  }

  private restart(): void {
    this.renderer.pause();
    this.renderer.auto();
    this.showMenuElements();
    this.renderer.setBackgroundColor(MENU_THEMES[this.themeMode].components.appBackground);
    this.renderer.requestRender();
  }

  private cleanup(): void {
    if (this.currentExample) {
      this.currentExample.destroy?.(this.renderer);
    }
    if (this.filterInput) {
      this.filterInput.blur();
    }
    if (this.selectElement) {
      this.selectElement.blur();
    }
    if (this.menuContainer) {
      this.menuContainer.destroy();
    }
    this.renderer.destroy();
  }
}

const logLevel = (process.env.BTUI_LOG_LEVEL as LogLevel) ?? "info";
const logFile = process.env.BTUI_LOG_FILE ?? "logs/examples.log";

const renderer = await createCliRenderer({
  exitOnCtrlC: false,
  targetFps: 60,
  logger: {
    dev: true,
    level: logLevel,
    file: logFile,
  },
});

renderer.setBackgroundColor(MENU_THEMES[DEFAULT_THEME_MODE].components.appBackground);
new ExampleSelector(renderer);
