/**
 * useTheme — reactive NapiTheme accessor.
 *
 * Returns the dark or light theme object from the native engine.
 * The theme is memoised per `mode` using a Solid memo so repeated calls
 * within the same component share the same instance.
 */

import { createDarkTheme, createLightTheme } from "@bettertui/core";
import type { NapiTheme } from "@bettertui/core";
import { createMemo } from "solid-js";

export type ThemeMode = "dark" | "light";

export function useTheme(mode: ThemeMode = "dark"): () => NapiTheme {
  return createMemo(() => (mode === "dark" ? createDarkTheme() : createLightTheme()));
}
