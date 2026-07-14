import type { Theme } from "./types";

/**
 * The default dark theme.
 * Values match the Rust engine's `Theme::dark()` output exactly.
 * Used as the base when no user theme is provided.
 */
export const DEFAULT_THEME: Theme = {
  name: "dark",
  colors: {
    background: "#1e1e28",
    surface: "#1e1e28",
    surfaceHigh: "#282837",
    surfaceLow: "#14141c",
    primary: "#648cdc",
    primaryForeground: "#ffffff",
    secondary: "#8c64c8",
    secondaryForeground: "#ffffff",
    text: "#dcdce6",
    textMuted: "#8c8ca0",
    textDim: "#5a5a69",
    border: "#3c3c50",
    borderFocused: "#648cdc",
    accent: "#50c8a0",
    accentForeground: "#ffffff",
    error: "#dc5050",
    warning: "#dcb43c",
    success: "#50c878",
    info: "#50a0dc",
    scrollbar: "#323241",
    scrollbarThumb: "#646482",
  },
  spacing: {
    none: 0,
    xxs: 1,
    xs: 2,
    sm: 4,
    md: 8,
    lg: 12,
    xl: 16,
    xxl: 24,
  },
  borders: {
    style: "solid",
    fg: "#3c3c50",
  },
};

/** Matches hex color strings: #RGB, #RRGGBB, or #RRGGBBAA. */
export const COLOR_REGEX = /^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$/;

/** Matches rgb() CSS color strings. */
export const RGB_REGEX = /^rgb\(\s*\d+\s*,\s*\d+\s*,\s*\d+\s*\)$/;

/** Matches rgba() CSS color strings. */
export const RGBA_REGEX = /^rgba\(\s*\d+\s*,\s*\d+\s*,\s*\d+\s*,\s*[\d.]+\s*\)$/;

/** Set of CSS named colors supported by the terminal renderer. */
export const NAMED_COLORS = new Set([
  "black",
  "white",
  "red",
  "green",
  "blue",
  "yellow",
  "cyan",
  "magenta",
  "gray",
  "grey",
  "transparent",
]);

/** All valid values for FlexDirection. */
export const VALID_FLEX_DIRECTIONS = ["row", "column", "row-reverse", "column-reverse"] as const;

/** All valid values for JustifyContent. */
export const VALID_JUSTIFY_CONTENTS = [
  "flex-start",
  "center",
  "flex-end",
  "space-between",
  "space-around",
  "space-evenly",
] as const;

/** All valid values for AlignItems. */
export const VALID_ALIGN_ITEMS = [
  "flex-start",
  "center",
  "flex-end",
  "stretch",
  "baseline",
] as const;

/** All valid values for AlignSelf. */
export const VALID_ALIGN_SELVES = [
  "flex-start",
  "center",
  "flex-end",
  "stretch",
  "baseline",
] as const;

/** All valid values for Position. */
export const VALID_POSITIONS = ["relative", "absolute"] as const;

/** All valid values for Overflow. */
export const VALID_OVERFLOWS = ["visible", "hidden", "scroll"] as const;

/** All valid values for flex-wrap. */
export const VALID_FLEX_WRAPS = ["nowrap", "wrap"] as const;
