import {
  Box,
  type CliRenderer,
  Input,
  InputEvents,
  type KeyEvent,
  type RawMouseEvent,
  RenderableEvents,
  Select,
  SelectEvents,
  type SelectOption,
  Text,
  bold,
  createCliRenderer,
  fg,
  t,
} from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys.js";

// ── Layout height constants ────────────────────────────────────────────────────
//
// Each constant is the TOTAL outer height of that panel (border included).
//
//   HEADER           3  = 1 border + 1 content + 1 border
//   PREVIEW          5  = 1 border + 3 content + 1 border  (title is in the border)
//   INPUT_CONTAINER  6  = 1 border + 1 label + 1 border + 1 input + 1 border + 1 border
//                      → inputLabel(1) + inputWrapper(h:3, border) = 4 inner rows
//   FOOTER           3  = 1 border + 1 content + 1 border
//
// selectArea fills all remaining rows via flexGrow:1 — no fixed height needed.
const HEADER_HEIGHT = 3;
const PREVIEW_HEIGHT = 5;
const INPUT_CONTAINER_HEIGHT = 6;
const FOOTER_HEIGHT = 3;

// ── Module-level state ────────────────────────────────────────────────────────
let renderer: CliRenderer | null = null;

// Boxes (refs needed for destroy)
let headerBox: Box | null = null;
let selectArea: Box | null = null; // flex-row, direct child of root, flexGrow:1
let colorSelectBox: Box | null = null;
let sizeSelectBox: Box | null = null;
let previewBox: Box | null = null;
let inputContainerBox: Box | null = null;
let footerBox: Box | null = null;

// Widgets
let colorSelect: Select | null = null;
let sizeSelect: Select | null = null;
let textInput: Input | null = null;

// Text nodes
let headerTitle: Text | null = null;
let previewText: Text | null = null;
let inputLabel: Text | null = null;
let footerText: Text | null = null;

// Focus management
let currentFocusIndex = 0;
const focusableWidgets: Array<Select | Input> = [];
const focusableBoxes: Array<Box | null> = [];

// Event handler refs for cleanup
let mouseHandler: ((event: RawMouseEvent) => void) | null = null;
let keyHandler: ((key: KeyEvent) => void) | null = null;
let resizeHandler: ((_w: number, _h: number) => void) | null = null;

// ── Options ───────────────────────────────────────────────────────────────────

const colorOptions: SelectOption[] = [
  {
    name: "Crimson",
    description: "Bold, passionate red — #dc2626",
    value: "#dc2626",
  },
  {
    name: "Sapphire",
    description: "Deep professional blue — #2563eb",
    value: "#2563eb",
  },
  {
    name: "Emerald",
    description: "Fresh natural green — #059669",
    value: "#059669",
  },
  {
    name: "Amber",
    description: "Warm energetic gold — #d97706",
    value: "#d97706",
  },
  {
    name: "Violet",
    description: "Creative regal purple — #7c3aed",
    value: "#7c3aed",
  },
  {
    name: "Coral",
    description: "Friendly vivid orange — #ea580c",
    value: "#ea580c",
  },
  {
    name: "Rose",
    description: "Soft romantic pink — #db2777",
    value: "#db2777",
  },
  {
    name: "Teal",
    description: "Calm oceanic cyan — #0891b2",
    value: "#0891b2",
  },
  {
    name: "Slate",
    description: "Neutral cool grey — #475569",
    value: "#475569",
  },
  {
    name: "Lime",
    description: "Vibrant electric green — #65a30d",
    value: "#65a30d",
  },
];

const sizeOptions: SelectOption[] = [
  { name: "Tiny", description: "Compact: 8px — dense dashboards", value: 8 },
  {
    name: "Small",
    description: "Readable: 10px — secondary content",
    value: 10,
  },
  { name: "Regular", description: "Standard: 12px — default body", value: 12 },
  { name: "Medium", description: "Comfortable: 14px — body copy", value: 14 },
  { name: "Large", description: "Prominent: 16px — headlines", value: 16 },
  { name: "XL", description: "Display: 20px — section headings", value: 20 },
  { name: "XXL", description: "Hero: 24px — major titles", value: 24 },
];

// ── Mouse helpers ─────────────────────────────────────────────────────────────

/** Enable SGR mouse button + scroll tracking in the terminal. */
function enableMouseTracking(): void {
  process.stdout.write("\x1b[?1000h\x1b[?1006h");
}

/** Disable mouse tracking on cleanup. */
function disableMouseTracking(): void {
  process.stdout.write("\x1b[?1006l\x1b[?1000l");
}

// ── Hit-zone mapping ──────────────────────────────────────────────────────────

/**
 * Map a click position to a focusable-element index:
 *   0  → colorSelect (left half of selectArea)
 *   1  → sizeSelect  (right half of selectArea)
 *   2  → textInput
 *  -1  → non-focusable zone (header / preview / footer)
 *
 * Uses the fixed-height constants to derive zone boundaries.
 */
function zoneForPosition(x: number, y: number): number {
  if (!renderer) return -1;
  const h = renderer.terminalHeight;
  const w = renderer.terminalWidth;

  const selectTop = HEADER_HEIGHT;
  const selectBottom = h - PREVIEW_HEIGHT - INPUT_CONTAINER_HEIGHT - FOOTER_HEIGHT;
  const inputTop = selectBottom + PREVIEW_HEIGHT;
  const inputBottom = inputTop + INPUT_CONTAINER_HEIGHT;

  if (y >= selectTop && y < selectBottom) {
    return x < Math.floor(w / 2) ? 0 : 1;
  }
  if (y >= inputTop && y < inputBottom) {
    return 2;
  }
  return -1;
}

// ── Focus helpers ─────────────────────────────────────────────────────────────

function blurAll(): void {
  for (const w of focusableWidgets) w.blur();
  for (const b of focusableBoxes) b?.blur();
}

function focusIndex(idx: number): void {
  blurAll();
  const len = focusableWidgets.length;
  currentFocusIndex = ((idx % len) + len) % len;
  focusableWidgets[currentFocusIndex]?.focus();
  focusableBoxes[currentFocusIndex]?.focus();
  updateHeader();
}

// ── Select height calculation ─────────────────────────────────────────────────

/**
 * Compute the explicit height the Select widgets need.
 *
 * `Select._getViewHeight()` falls back to a viewport-walking heuristic when
 * `height` is "auto"; that heuristic under-counts on short terminals because it
 * can't see the Taffy-computed flex height.  Passing an explicit number
 * short-circuits the heuristic.
 *
 * Overhead:
 *   HEADER + PREVIEW + INPUT_CONTAINER + FOOTER  = fixed panel rows
 *   + 2  (colorSelectBox / sizeSelectBox border)
 *
 * No outer selectContainerBox border is added here because we removed that
 * wrapper — selectArea is a borderless flex-row directly in root.
 */
function calcSelectHeight(r: CliRenderer): number {
  const fixed = HEADER_HEIGHT + PREVIEW_HEIGHT + INPUT_CONTAINER_HEIGHT + FOOTER_HEIGHT;
  return Math.max(4, r.terminalHeight - fixed - 2);
}

/** Re-apply explicit heights after a terminal resize and force re-render. */
function refreshSelectHeights(r: CliRenderer): void {
  if (!colorSelect || !sizeSelect) return;
  const h = calcSelectHeight(r);
  colorSelect.height = h;
  sizeSelect.height = h;
  // moveUp(0) calls _render() internally without changing the selected index.
  colorSelect.moveUp(0);
  sizeSelect.moveUp(0);
}

// ── Live display helpers ──────────────────────────────────────────────────────

function getFocusName(): string {
  switch (currentFocusIndex) {
    case 0:
      return "Color Select";
    case 1:
      return "Size Select";
    case 2:
      return "Text Input";
    default:
      return "—";
  }
}

function updateHeader(): void {
  if (!headerTitle) return;
  headerTitle.content = t`${bold(fg("#ffffff")(" INPUT & SELECT LAYOUT"))}   ${fg("#94a3b8")("focus:")} ${fg("#38bdf8")(getFocusName())}`;
}

function updatePreview(): void {
  if (!previewText || !colorSelect || !sizeSelect || !textInput) return;

  const color = colorSelect.getSelectedOption();
  const size = sizeSelect.getSelectedOption();
  const text = textInput.value.trim();

  const colorHex = (color?.value as string) ?? "#e2e8f0";
  const colorName = color?.name ?? "—";
  const sizeName = size?.name ?? "—";
  const sizePx = (size?.value as number) ?? 12;
  const sampleText = text || "Type something below to see it styled here…";

  // All styled values are interpolated directly inside t`` — using a plain
  // backtick template would call .toString() on StyledText → "[object Object]".
  previewText.content = t` ${fg("#475569")("Preview |")} ${fg("#64748b")("color:")} ${fg(colorHex)(colorName)} ${fg("#475569")(`(${colorHex})`)}   ${fg("#64748b")("size:")} ${fg("#facc15")(sizeName)} ${fg("#475569")(`(${sizePx}px)`)}
 ${fg(colorHex)(bold(sampleText))}`;
}

function updateInputLabel(): void {
  if (!inputLabel || !textInput) return;
  const val = textInput.value;
  if (val.length === 0) {
    inputLabel.content = "  Enter text:";
    return;
  }
  const remaining = 80 - val.length;
  const filled = Math.round((val.length / 80) * 20);
  const bar = "█".repeat(filled);
  const empty = "░".repeat(20 - filled);
  inputLabel.content = t`  ${fg("#94a3b8")("Enter text:")}  ${fg("#22d3ee")(`${val.length}/80`)}  ${fg("#3b82f6")(bar)}${fg("#1e293b")(empty)}  ${fg(remaining > 0 ? "#64748b" : "#ef4444")(`${remaining} remaining`)}`;
}

// ── Layout factory ────────────────────────────────────────────────────────────

function buildLayout(r: CliRenderer): void {
  renderer = r;
  r.setBackgroundColor("#0a0f1e");

  // ── Header ──────────────────────────────────────────────────────────────────
  // height: HEADER_HEIGHT, flexShrink: 0 so it never yields rows to flex layout
  headerBox = new Box(r, {
    id: "isl-header",
    width: "auto",
    height: HEADER_HEIGHT,
    backgroundColor: "#1e3a5f",
    borderStyle: "single",
    borderColor: "#2563eb",
    border: true,
    overflow: "hidden",
    flexGrow: 0,
    flexShrink: 0,
    paddingLeft: 1,
    zIndex: 0,
  });

  headerTitle = new Text(r, {
    id: "isl-header-title",
    content: "",
    fg: "#ffffff",
    bg: "transparent",
    flexGrow: 1,
    flexShrink: 1,
    zIndex: 1,
  });
  headerBox.add(headerTitle);

  // ── Select area ─────────────────────────────────────────────────────────────
  // Borderless flex-row that grows to fill all space not taken by fixed panels.
  // Each half (colorSelectBox / sizeSelectBox) has flexGrow:1 so they share
  // available width equally and stretch to fill the row's height automatically.
  selectArea = new Box(r, {
    id: "isl-select-area",
    width: "auto",
    height: "auto",
    flexDirection: "row",
    flexGrow: 1,
    flexShrink: 1,
    minHeight: 8,
    overflow: "hidden",
    zIndex: 0,
  });

  // Left panel — Color
  colorSelectBox = new Box(r, {
    id: "isl-color-select-box",
    width: "auto",
    height: "auto",
    borderStyle: "round",
    borderColor: "#334155",
    focusedBorderColor: "#3b82f6",
    title: " Color ",
    titleAlignment: "center",
    flexGrow: 1,
    flexShrink: 1,
    overflow: "hidden",
    backgroundColor: "#0a0f1e",
    border: true,
    zIndex: 0,
  });

  colorSelect = new Select(r, {
    id: "isl-color-select",
    width: "auto",
    height: calcSelectHeight(r),
    flexGrow: 1,
    flexShrink: 1,
    options: colorOptions,
    backgroundColor: "#0d1525",
    focusedBackgroundColor: "#111827",
    textColor: "#94a3b8",
    focusedTextColor: "#e2e8f0",
    selectedBackgroundColor: "#1d4ed8",
    selectedTextColor: "#ffffff",
    descriptionColor: "#475569",
    selectedDescriptionColor: "#93c5fd",
    showScrollIndicator: true,
    wrapSelection: true,
    showDescription: true,
    showSelectionIndicator: false,
    fastScrollStep: 3,
    zIndex: 1,
  });
  colorSelectBox.add(colorSelect);

  // Right panel — Size
  sizeSelectBox = new Box(r, {
    id: "isl-size-select-box",
    width: "auto",
    height: "auto",
    borderStyle: "round",
    borderColor: "#334155",
    focusedBorderColor: "#059669",
    title: " Size ",
    titleAlignment: "center",
    flexGrow: 1,
    flexShrink: 1,
    overflow: "hidden",
    backgroundColor: "#0a0f1e",
    border: true,
    zIndex: 0,
  });

  sizeSelect = new Select(r, {
    id: "isl-size-select",
    width: "auto",
    height: calcSelectHeight(r),
    flexGrow: 1,
    flexShrink: 1,
    options: sizeOptions,
    backgroundColor: "#0d1525",
    focusedBackgroundColor: "#111827",
    textColor: "#94a3b8",
    focusedTextColor: "#e2e8f0",
    selectedBackgroundColor: "#065f46",
    selectedTextColor: "#ffffff",
    descriptionColor: "#475569",
    selectedDescriptionColor: "#6ee7b7",
    showScrollIndicator: true,
    wrapSelection: false,
    showDescription: true,
    showSelectionIndicator: false,
    fastScrollStep: 3,
    zIndex: 1,
  });
  sizeSelectBox.add(sizeSelect);

  selectArea.add(colorSelectBox);
  selectArea.add(sizeSelectBox);

  // ── Preview ─────────────────────────────────────────────────────────────────
  // Fixed-height panel; title is rendered in the border so the 3 inner rows are
  // fully available for content (sep + meta + sample).  overflow:"hidden" clips
  // any text that overruns the box.
  previewBox = new Box(r, {
    id: "isl-preview-box",
    width: "auto",
    height: PREVIEW_HEIGHT,
    backgroundColor: "#0d1525",
    borderStyle: "single",
    borderColor: "#1e293b",
    title: " Preview ",
    titleAlignment: "left",
    border: true,
    overflow: "hidden",
    flexGrow: 0,
    flexShrink: 0,
    zIndex: 0,
  });

  previewText = new Text(r, {
    id: "isl-preview-text",
    content: "",
    fg: "#e2e8f0",
    bg: "transparent",
    flexGrow: 1,
    flexShrink: 1,
    zIndex: 1,
  });
  previewBox.add(previewText);

  // ── Input container ─────────────────────────────────────────────────────────
  // Height: 6 = 1(border) + 1(label) + 1(inner-border) + 1(input) +
  //              1(inner-border) + 1(border)
  // overflow:"hidden" prevents the inputWrapper from visually spilling out.
  inputContainerBox = new Box(r, {
    id: "isl-input-container",
    width: "auto",
    height: INPUT_CONTAINER_HEIGHT,
    backgroundColor: "#060d1a",
    borderStyle: "single",
    borderColor: "#1e293b",
    title: " Text Input ",
    titleAlignment: "left",
    flexDirection: "column",
    flexGrow: 0,
    flexShrink: 0,
    border: true,
    overflow: "hidden",
    zIndex: 0,
  });

  inputLabel = new Text(r, {
    id: "isl-input-label",
    content: "  Enter text:",
    fg: "#94a3b8",
    bg: "transparent",
    flexGrow: 0,
    flexShrink: 0,
    zIndex: 0,
  });

  // inputWrapper: height:3 = border(1) + input(1) + border(1).
  // marginLeft/Right give breathing room inside the container.
  const inputWrapper = new Box(r, {
    id: "isl-input-wrapper",
    width: "auto",
    height: 3,
    borderStyle: "round",
    borderColor: "#334155",
    focusedBorderColor: "#facc15",
    flexGrow: 0,
    flexShrink: 0,
    marginLeft: 1,
    marginRight: 1,
    overflow: "hidden",
    backgroundColor: "transparent",
    border: true,
    zIndex: 0,
  });

  textInput = new Input(r, {
    id: "isl-text-input",
    width: "auto",
    height: 1,
    placeholder: "Type something and watch the preview update…",
    backgroundColor: "#060d1a",
    focusedBackgroundColor: "#0f1f35",
    textColor: "#e2e8f0",
    focusedTextColor: "#ffffff",
    placeholderColor: "#334155",
    cursorColor: "#facc15",
    maxLength: 80,
    flexGrow: 1,
    flexShrink: 1,
    zIndex: 1,
  });

  inputWrapper.add(textInput);
  inputContainerBox.add(inputLabel);
  inputContainerBox.add(inputWrapper);

  // ── Footer ──────────────────────────────────────────────────────────────────
  footerBox = new Box(r, {
    id: "isl-footer",
    width: "auto",
    height: FOOTER_HEIGHT,
    backgroundColor: "#0f172a",
    borderStyle: "single",
    borderColor: "#1e293b",
    flexGrow: 0,
    flexShrink: 0,
    overflow: "hidden",
    border: true,
    paddingLeft: 1,
    zIndex: 0,
  });

  footerText = new Text(r, {
    id: "isl-footer-text",
    content: t`${fg("#64748b")("TAB")} next  ${fg("#64748b")("SHIFT+TAB")} prev  ${fg("#64748b")("↑↓/jk")} navigate  ${fg("#64748b")("ENTER")} select  ${fg("#64748b")("MOUSE")} click·scroll  ${fg("#64748b")("ESC")} quit`,
    fg: "#64748b",
    bg: "transparent",
    flexGrow: 1,
    flexShrink: 1,
    zIndex: 1,
  });
  footerBox.add(footerText);

  // ── Wire into root ────────────────────────────────────────────────────────
  r.root.add(headerBox);
  r.root.add(selectArea);
  r.root.add(previewBox);
  r.root.add(inputContainerBox);
  r.root.add(footerBox);

  // ── Focusable registry ────────────────────────────────────────────────────
  focusableWidgets.push(colorSelect, sizeSelect, textInput);
  focusableBoxes.push(colorSelectBox, sizeSelectBox, inputWrapper);
}

// ── Event wiring ──────────────────────────────────────────────────────────────

function wireEvents(r: CliRenderer): void {
  if (!colorSelect || !sizeSelect || !textInput) return;

  // Select changes → live preview
  colorSelect.on(SelectEvents.SELECTION_CHANGED, () => updatePreview());
  colorSelect.on(SelectEvents.ITEM_SELECTED, () => updatePreview());
  sizeSelect.on(SelectEvents.SELECTION_CHANGED, () => updatePreview());
  sizeSelect.on(SelectEvents.ITEM_SELECTED, () => updatePreview());

  // Input changes → label + preview
  textInput.on(InputEvents.INPUT, () => {
    updateInputLabel();
    updatePreview();
  });
  textInput.on(InputEvents.CHANGE, () => {
    updateInputLabel();
    updatePreview();
  });

  // Focus changes → header badge
  for (const w of focusableWidgets) {
    w.on(RenderableEvents.FOCUSED, () => updateHeader());
    w.on(RenderableEvents.BLURRED, () => updateHeader());
  }

  // ── Resize: update explicit Select heights ─────────────────────────────────
  resizeHandler = (_w: number, _h: number) => {
    refreshSelectHeights(r);
    updatePreview();
  };
  r.on("resize", resizeHandler);

  // ── Keyboard ───────────────────────────────────────────────────────────────
  keyHandler = (key: KeyEvent) => {
    if (key.name === "tab") {
      focusIndex(key.shift ? currentFocusIndex - 1 : currentFocusIndex + 1);
      return;
    }
    if (key.name === "escape" || (key.ctrl && key.name === "q")) {
      r.destroy();
      process.exit(0);
    }
  };
  r.keyInput.on("keypress", keyHandler);

  // ── Mouse: click → focus, scroll → navigate ────────────────────────────────
  mouseHandler = (event: RawMouseEvent) => {
    if (event.type === "down") {
      const zone = zoneForPosition(event.x, event.y);
      if (zone >= 0) focusIndex(zone);
      return;
    }
    if (event.type === "scroll" && event.scroll) {
      const focused = focusableWidgets[currentFocusIndex];
      if (focused instanceof Select) {
        if (event.scroll.direction === "up") focused.moveUp(event.scroll.delta);
        else if (event.scroll.direction === "down") focused.moveDown(event.scroll.delta);
      }
    }
  };
  r.keyInput.on("mouse", mouseHandler);
}

// ── Public API ────────────────────────────────────────────────────────────────

export function run(rendererInstance: CliRenderer): void {
  buildLayout(rendererInstance);
  wireEvents(rendererInstance);
  enableMouseTracking();
  focusIndex(0);
  updatePreview();
  updateInputLabel();
}

export function destroy(rendererInstance: CliRenderer): void {
  disableMouseTracking();

  if (keyHandler) {
    rendererInstance.keyInput.off("keypress", keyHandler);
    keyHandler = null;
  }
  if (mouseHandler) {
    rendererInstance.keyInput.off("mouse", mouseHandler);
    mouseHandler = null;
  }
  if (renderer && resizeHandler) {
    renderer.off("resize", resizeHandler);
  }
  resizeHandler = null;

  colorSelect?.destroy();
  sizeSelect?.destroy();
  textInput?.destroy();

  if (headerBox) rendererInstance.root.remove(headerBox);
  if (selectArea) rendererInstance.root.remove(selectArea);
  if (previewBox) rendererInstance.root.remove(previewBox);
  if (inputContainerBox) rendererInstance.root.remove(inputContainerBox);
  if (footerBox) rendererInstance.root.remove(footerBox);

  renderer = null;
  headerBox = null;
  headerTitle = null;
  selectArea = null;
  colorSelectBox = null;
  sizeSelectBox = null;
  colorSelect = null;
  sizeSelect = null;
  previewBox = null;
  previewText = null;
  inputContainerBox = null;
  inputLabel = null;
  textInput = null;
  footerBox = null;
  footerText = null;

  focusableWidgets.length = 0;
  focusableBoxes.length = 0;
  currentFocusIndex = 0;
}

if (import.meta.main) {
  const r = await createCliRenderer({
    exitOnCtrlC: true,
    targetFps: 30,
  });
  run(r);
  setupCommonDemoKeys(r);
  r.start();
}
