import { createDarkTheme, createLightTheme } from "@bettertui/core";
import type { NapiTheme } from "@bettertui/core";
import { useMemo } from "react";

export type ThemeMode = "dark" | "light";

/**
 * Returns a theme token object for the given mode (default: dark).
 * The theme is memoised and only recreated when `mode` changes.
 */
export function useTheme(mode: ThemeMode = "dark"): NapiTheme {
  return useMemo(() => (mode === "light" ? createLightTheme() : createDarkTheme()), [mode]);
}
