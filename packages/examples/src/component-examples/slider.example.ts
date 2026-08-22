import {
  Box,
  type CliRenderer,
  type RawMouseEvent,
  Slider,
  SliderEvents,
  Text,
  bold,
  createCliRenderer,
  fg,
  t,
} from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

// ── Types ──────────────────────────────────────────────────────────────────────

interface SliderRegion {
  slider: Slider;
  orientation: "horizontal" | "vertical";
  trackLength: number;
  /** Absolute X of the track start. */
  trackX: number;
  /** Absolute Y of the track start (top for vertical). */
  trackY: number;
  /** Width of the clickable region. */
  hitWidth: number;
  /** Height of the clickable region. */
  hitHeight: number;
}

interface DragState {
  region: SliderRegion;
  startX: number;
  startY: number;
  startValue: number;
}

// ── Module state ───────────────────────────────────────────────────────────────

let rootContainer: Box | null = null;
let renderer: CliRenderer | null = null;
const sliderRegions: SliderRegion[] = [];
const valueTexts: Text[] = [];
let activeSliderIndex = 0;
let dragState: DragState | null = null;
let mouseHandler: ((event: RawMouseEvent) => void) | null = null;
let lastCursorIsPointer = false;

const SLIDER_DEFS_CONFIG = [
  { label: "Volume", min: 0, max: 100, value: 60, step: 1, color: "#7aa2f7" },
  { label: "Brightness", min: 0, max: 100, value: 80, step: 5, color: "#9ece6a" },
  { label: "Contrast", min: -50, max: 50, value: 0, step: 1, color: "#e0af68" },
  { label: "Opacity", min: 0, max: 1, value: 0.8, step: 0.1, color: "#bb9af7" },
] as const;

// ── Layout constants (must match the layout built in run()) ─────────────────
const ROOT_PAD = 2;
const ROOT_GAP = 2;
const LABEL_WIDTH = 12;
const ROW_GAP = 2;
const H_SLIDER_WIDTH = 40;
const VERT_HEIGHT = 12;
const VERT_GAP = 4;

// ── Helpers ────────────────────────────────────────────────────────────────────

function formatValue(val: number, maxVal: number): string {
  return maxVal === 1 ? val.toFixed(1) : String(Math.round(val));
}

/** Write an OSC 22 cursor-shape sequence. */
function setCursorPointer(enabled: boolean): void {
  if (enabled === lastCursorIsPointer) return;
  lastCursorIsPointer = enabled;
  process.stdout.write(enabled ? "\x1b]22;pointer\x07" : "\x1b]22;default\x07");
}

/** Hit-test (x, y) against stored slider regions. */
function hitSlider(x: number, y: number): SliderRegion | null {
  for (const region of sliderRegions) {
    if (
      x >= region.trackX &&
      x < region.trackX + region.hitWidth &&
      y >= region.trackY &&
      y < region.trackY + region.hitHeight
    ) {
      return region;
    }
  }
  return null;
}

function focusSlider(index: number): void {
  sliderRegions[activeSliderIndex]?.slider.blur();
  activeSliderIndex = Math.max(0, Math.min(index, sliderRegions.length - 1));
  sliderRegions[activeSliderIndex]?.slider.focus();
}

// ── Mouse event handler ───────────────────────────────────────────────────────

function handleMouse(event: RawMouseEvent): void {
  if (!renderer) return;

  const { x, y, type, button } = event;
  const isLeftButton = button === 0;

  switch (type) {
    case "move": {
      setCursorPointer(hitSlider(x, y) !== null);
      break;
    }

    case "down": {
      if (!isLeftButton) break;
      const hit = hitSlider(x, y);
      if (!hit) break;

      const regionIndex = sliderRegions.indexOf(hit);
      if (regionIndex >= 0) focusSlider(regionIndex);

      // Click-to-jump: map click position onto value range
      let fraction: number;
      if (hit.orientation === "horizontal") {
        const clickOffset = x - hit.trackX;
        fraction = Math.max(0, Math.min(1, clickOffset / Math.max(1, hit.trackLength - 1)));
      } else {
        const clickOffset = y - hit.trackY;
        fraction = 1 - Math.max(0, Math.min(1, clickOffset / Math.max(1, hit.trackLength - 1)));
      }
      hit.slider.value = hit.slider.min + fraction * (hit.slider.max - hit.slider.min);

      dragState = { region: hit, startX: x, startY: y, startValue: hit.slider.value };
      setCursorPointer(true);
      break;
    }

    case "drag": {
      if (!isLeftButton || !dragState) break;
      const { region, startY, startValue } = dragState;
      const sl = region.slider;

      if (region.orientation === "horizontal") {
        const clickOffset = x - region.trackX;
        const fraction = Math.max(
          0,
          Math.min(1, clickOffset / Math.max(1, region.trackLength - 1)),
        );
        sl.value = sl.min + fraction * (sl.max - sl.min);
      } else {
        const delta = startY - y;
        const perRow = (sl.max - sl.min) / Math.max(1, region.trackLength - 1);
        sl.value = startValue + delta * perRow;
      }

      setCursorPointer(true);
      break;
    }

    case "up": {
      dragState = null;
      setCursorPointer(hitSlider(x, y) !== null);
      break;
    }

    case "drag-end": {
      dragState = null;
      setCursorPointer(false);
      break;
    }

    case "scroll": {
      const hit = hitSlider(x, y);
      if (!hit) break;
      const dir = event.scroll?.direction;
      const sl = hit.slider;
      if (hit.orientation === "horizontal") {
        if (dir === "right") sl.value += sl.step;
        else if (dir === "left") sl.value -= sl.step;
      } else {
        if (dir === "up") sl.value += sl.step;
        else if (dir === "down") sl.value -= sl.step;
      }
      break;
    }

    default:
      break;
  }
}

// ── run / destroy ─────────────────────────────────────────────────────────────

export function run(rendererInstance: CliRenderer): void {
  renderer = rendererInstance;
  renderer.setBackgroundColor("#1a1b26");

  // Enable terminal mouse reporting: all-motion (1003) + SGR encoding (1006)
  process.stdout.write("\x1b[?1003h\x1b[?1006h");

  rootContainer = new Box(renderer, {
    id: "slider-example-root",
    flexDirection: "column",
    width: "100%",
    height: "100%",
    padding: ROOT_PAD,
    gap: ROOT_GAP,
  });
  renderer.root.add(rootContainer);

  // Header
  rootContainer.add(
    new Text(renderer, {
      content: t`${bold(fg("#7aa2f7")("Slider Component Example"))}`,
      height: 1,
      flexShrink: 0,
    }),
  );
  rootContainer.add(
    new Text(renderer, {
      content: "─".repeat(60),
      fg: "#414868",
      height: 1,
      flexShrink: 0,
    }),
  );

  // ── Horizontal sliders ──────────────────────────────────────────────────────
  // Y position tracking: pad(2) + header(1) + gap(2) + separator(1) + gap(2) = 8
  let currentY = ROOT_PAD + 1 + ROOT_GAP + 1 + ROOT_GAP;

  for (let i = 0; i < SLIDER_DEFS_CONFIG.length; i++) {
    const def = SLIDER_DEFS_CONFIG[i];
    const row = new Box(renderer, {
      flexDirection: "row",
      gap: ROW_GAP,
      alignItems: "center",
      flexShrink: 0,
      marginBottom: 1,
    });

    row.add(new Text(renderer, { content: `${def.label}:`, width: LABEL_WIDTH, fg: "#a9b1d6" }));

    const slider = new Slider(renderer, {
      orientation: "horizontal",
      width: H_SLIDER_WIDTH,
      height: 1,
      min: def.min,
      max: def.max,
      value: def.value,
      step: def.step,
      thumbColor: def.color,
      activeTrackColor: def.color,
      trackColor: "#333333",
    });

    const valText = new Text(renderer, {
      content: formatValue(def.value, def.max),
      width: 8,
      fg: def.color,
    });

    const capturedI = i;
    slider.on(SliderEvents.CHANGE, (val: number) => {
      valText.content = formatValue(val, SLIDER_DEFS_CONFIG[capturedI].max);
    });

    row.add(slider);
    row.add(valText);
    rootContainer.add(row);

    const trackX = ROOT_PAD + LABEL_WIDTH + ROW_GAP;
    sliderRegions.push({
      slider,
      orientation: "horizontal",
      trackLength: H_SLIDER_WIDTH - 2,
      trackX,
      trackY: currentY,
      hitWidth: H_SLIDER_WIDTH,
      hitHeight: 1,
    });
    valueTexts.push(valText);

    // Advance Y: row height(1) + marginBottom(1) + gap(2) = 4
    currentY += 1 + 1 + ROOT_GAP;
  }

  // ── Vertical sliders ────────────────────────────────────────────────────────
  rootContainer.add(
    new Text(renderer, {
      content: t`${fg("#565f89")("Vertical sliders")}`,
      height: 1,
      flexShrink: 0,
    }),
  );
  // After last horizontal row we added gap already; then vertLabel: currentY is correct
  // Subtract the extra marginBottom+gap from the last row since the next item is the label
  // Actually: last row added (1 + 1 + 2), so currentY already points to this text.
  // Then vertLabel height=1 + gap=2 → vertRow starts at currentY + 1 + 2
  const vertRowY = currentY + 1 + ROOT_GAP;

  const vertRow = new Box(renderer, {
    flexDirection: "row",
    gap: VERT_GAP,
    alignItems: "flex-end",
    height: 15,
    flexShrink: 0,
  });

  const VERT_COLORS = ["#f7768e", "#e0af68", "#9ece6a"] as const;
  const VERT_LABELS = ["R", "G", "B"] as const;
  const VERT_INIT = [200, 150, 100] as const;

  for (let i = 0; i < 3; i++) {
    const col = new Box(renderer, { flexDirection: "column", alignItems: "center", gap: 1 });

    const vSlider = new Slider(renderer, {
      orientation: "vertical",
      width: 1,
      height: VERT_HEIGHT,
      min: 0,
      max: 255,
      value: VERT_INIT[i],
      step: 5,
      thumbColor: VERT_COLORS[i],
      activeTrackColor: VERT_COLORS[i],
      trackColor: "#333333",
    });

    const vLabel = new Text(renderer, {
      content: VERT_LABELS[i],
      fg: VERT_COLORS[i],
      textAlign: "center",
    });

    col.add(vSlider);
    col.add(vLabel);
    vertRow.add(col);

    // Vertical slider position: vertRow is height 15, alignItems flex-end.
    // The column contains vSlider(h=12) + gap(1) + label(h=1) = 14 total.
    // With flex-end alignment in a h=15 row, the column starts at y_offset=1 from vertRow top.
    // The vSlider is the first child of the column, so its top = vertRowY + 1.
    const vSliderY = vertRowY + 1;
    // X position: rootPad + i*(1 + VERT_GAP) for each vertical slider column
    const vSliderX = ROOT_PAD + i * (1 + VERT_GAP);

    sliderRegions.push({
      slider: vSlider,
      orientation: "vertical",
      trackLength: VERT_HEIGHT - 2,
      trackX: vSliderX,
      trackY: vSliderY,
      hitWidth: 1,
      hitHeight: VERT_HEIGHT,
    });
  }

  rootContainer.add(vertRow);

  // ── Navigation hint ─────────────────────────────────────────────────────────
  rootContainer.add(
    new Text(renderer, {
      content: t`${fg("#414868")(
        "Tab/Shift+Tab switch focus  ·  ←→ / ↑↓ adjust  ·  Click or drag slider  ·  Scroll wheel  ·  Ctrl+C quit",
      )}`,
      height: 1,
      flexShrink: 0,
    }),
  );

  // ── Keyboard tab navigation ─────────────────────────────────────────────────
  rendererInstance.keyInput.on("keypress", (key) => {
    if (key.name === "tab") {
      key.preventDefault?.();
      focusSlider(key.shift ? activeSliderIndex - 1 : activeSliderIndex + 1);
    }
  });

  // ── Mouse handler ───────────────────────────────────────────────────────────
  mouseHandler = (event: RawMouseEvent) => handleMouse(event);
  rendererInstance.keyInput.on("mouse", mouseHandler);

  focusSlider(0);
}

export function destroy(rendererInstance: CliRenderer): void {
  // Disable mouse reporting and reset cursor
  process.stdout.write("\x1b[?1003l\x1b[?1006l");
  setCursorPointer(false);
  lastCursorIsPointer = false;

  if (mouseHandler) {
    rendererInstance.keyInput.off("mouse", mouseHandler);
    mouseHandler = null;
  }

  for (const d of sliderRegions) d.slider.destroy();
  sliderRegions.length = 0;
  valueTexts.length = 0;
  dragState = null;

  if (rootContainer) {
    rendererInstance.root.remove(rootContainer);
    rootContainer.destroy();
    rootContainer = null;
  }
  renderer = null;
  activeSliderIndex = 0;
}

if (import.meta.main) {
  const r = await createCliRenderer({ exitOnCtrlC: true });
  run(r);
  setupCommonDemoKeys(r);
  r.start();
}
