import { RGBA } from "@bettertui/core";
import { brightness } from "./colorUtils";

/** Whether an OSC sequence is a color reply (palette fg/bg). */
export function isColorOscResponse(sequence: string): boolean {
  // biome-ignore lint/suspicious/noControlCharactersInRegex: OSC response detection requires ESC
  return /^\x1b\](?:4;0|10|11);/.test(sequence);
}

/** Uppercase a hex value or render a fallback. */
export function formatHex(value: string | null): string {
  return value?.toUpperCase() ?? "N/A";
}

/** Infer terminal light/dark mode from a hex background color. */
export function inferModeFromHex(value: string | null): "dark" | "light" | "unknown" {
  if (!value) return "unknown";
  const rgba = RGBA.fromHex(value);
  const r = Math.round(rgba.r * 255);
  const g = Math.round(rgba.g * 255);
  const b = Math.round(rgba.b * 255);
  return brightness(r, g, b) > 128 ? "light" : "dark";
}

/** Render an OSC sequence with control characters made visible for display. */
export function visibleOsc(sequence: string): string {
  let result = sequence;
  // biome-ignore lint/suspicious/noControlCharactersInRegex: OSC visible representation requires ESC/BEL chars
  result = result.replace(/\x1b\\/g, " ST");
  // biome-ignore lint/suspicious/noControlCharactersInRegex: OSC visible representation requires ESC chars
  result = result.replace(/\x1b/g, "ESC");
  // biome-ignore lint/suspicious/noControlCharactersInRegex: OSC visible representation requires BEL chars
  result = result.replace(/\x07/g, " BEL");
  return result;
}
