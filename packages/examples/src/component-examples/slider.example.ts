import {
  Box,
  type CliRenderer,
  Slider,
  SliderEvents,
  Text,
  bold,
  createCliRenderer,
  fg,
  t,
} from "@bettertui/core";
import { setupCommonDemoKeys } from "../lib/standaloneKeys";

let rootContainer: Box | null = null;
let renderer: CliRenderer | null = null;
const sliders: Slider[] = [];
const valueTexts: Text[] = [];
let activeSliderIndex = 0;

const SLIDER_DEFS = [
  {
    label: "Volume",
    min: 0,
    max: 100,
    value: 60,
    step: 1,
    color: "#7aa2f7",
    orientation: "horizontal",
  },
  {
    label: "Brightness",
    min: 0,
    max: 100,
    value: 80,
    step: 5,
    color: "#9ece6a",
    orientation: "horizontal",
  },
  {
    label: "Contrast",
    min: -50,
    max: 50,
    value: 0,
    step: 1,
    color: "#e0af68",
    orientation: "horizontal",
  },
  {
    label: "Opacity",
    min: 0,
    max: 1,
    value: 0.8,
    step: 0.1,
    color: "#bb9af7",
    orientation: "horizontal",
  },
] as const;

function formatValue(val: number, def: (typeof SLIDER_DEFS)[number]): string {
  if (def.max === 1) return val.toFixed(1);
  return String(Math.round(val));
}

function focusSlider(index: number): void {
  sliders[activeSliderIndex]?.blur();
  activeSliderIndex = Math.max(0, Math.min(index, sliders.length - 1));
  sliders[activeSliderIndex]?.focus();
}

export function run(rendererInstance: CliRenderer): void {
  renderer = rendererInstance;
  renderer.setBackgroundColor("#1a1b26");

  rootContainer = new Box(renderer, {
    id: "slider-example-root",
    flexDirection: "column",
    width: "100%",
    height: "100%",
    padding: 2,
    gap: 2,
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

  // Slider rows
  for (let i = 0; i < SLIDER_DEFS.length; i++) {
    const def = SLIDER_DEFS[i];
    const row = new Box(renderer, {
      flexDirection: "row",
      gap: 2,
      alignItems: "center",
      flexShrink: 0,
      marginBottom: 1,
    });

    // Label
    row.add(new Text(renderer, { content: `${def.label}:`, width: 12, fg: "#a9b1d6" }));

    // Slider
    const slider = new Slider(renderer, {
      orientation: "horizontal",
      width: 40,
      height: 1,
      min: def.min,
      max: def.max,
      value: def.value,
      step: def.step,
      thumbColor: def.color,
      activeTrackColor: def.color,
      trackColor: "#333333",
      focusedBorderColor: def.color,
    });

    // Value display
    const valText = new Text(renderer, {
      content: formatValue(def.value, def),
      width: 8,
      fg: def.color,
    });

    const capturedI = i;
    slider.on(SliderEvents.CHANGE, (val: number) => {
      valText.content = formatValue(val, SLIDER_DEFS[capturedI]);
    });

    row.add(slider);
    row.add(valText);
    rootContainer.add(row);

    sliders.push(slider);
    valueTexts.push(valText);
  }

  // Vertical slider section
  rootContainer.add(
    new Text(renderer, {
      content: t`${fg("#565f89")("Vertical sliders")}`,
      height: 1,
      flexShrink: 0,
    }),
  );

  const vertRow = new Box(renderer, {
    flexDirection: "row",
    gap: 4,
    alignItems: "flex-end",
    height: 15,
    flexShrink: 0,
  });

  const vertColors = ["#f7768e", "#e0af68", "#9ece6a"];
  const vertLabels = ["R", "G", "B"];

  for (let i = 0; i < 3; i++) {
    const col = new Box(renderer, { flexDirection: "column", alignItems: "center", gap: 1 });

    const vSlider = new Slider(renderer, {
      orientation: "vertical",
      width: 1,
      height: 12,
      min: 0,
      max: 255,
      value: [200, 150, 100][i],
      step: 5,
      thumbColor: vertColors[i],
      activeTrackColor: vertColors[i],
      trackColor: "#333333",
    });

    const vLabel = new Text(renderer, {
      content: vertLabels[i] ?? "",
      fg: vertColors[i] ?? "#ffffff",
      textAlign: "center",
    });

    col.add(vSlider);
    col.add(vLabel);
    vertRow.add(col);

    sliders.push(vSlider);
  }

  rootContainer.add(vertRow);

  // Navigation hint
  rootContainer.add(
    new Text(renderer, {
      content: t`${fg("#414868")("Tab / Shift+Tab to switch sliders  ·  ← → to adjust  ·  Ctrl+C to quit")}`,
      height: 1,
      flexShrink: 0,
    }),
  );

  // Tab navigation between sliders
  rendererInstance.keyInput.on("keypress", (key) => {
    if (key.name === "tab") {
      key.preventDefault?.();
      if (key.shift) {
        focusSlider(activeSliderIndex - 1);
      } else {
        focusSlider(activeSliderIndex + 1);
      }
    }
  });

  focusSlider(0);
}

export function destroy(rendererInstance: CliRenderer): void {
  for (const s of sliders) s.destroy();
  sliders.length = 0;
  valueTexts.length = 0;

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
