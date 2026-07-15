// Example-facing theme definitions. Internal to the examples package. OpenTUI's
// launcher toggles a dark/light palette; we mirror that with BetterTUI's own
// SharedTheme objects passed to the <Provider theme> prop.

import type { Theme } from "@bettertui/shared";

export interface ExampleThemeName {
  dark: "dark";
  light: "light";
}

export type ExampleThemeNameLiteral = "dark" | "light";

function makeTheme(name: "dark" | "light", colors: Theme["colors"]): Theme {
  return {
    name,
    colors,
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
    borders: { style: "solid", fg: colors.border },
  };
}

const darkColors: Theme["colors"] = {
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
  scrollbar: "#282837",
  scrollbarThumb: "#5a5a69",
};

const lightColors: Theme["colors"] = {
  background: "#f4f4f8",
  surface: "#ffffff",
  surfaceHigh: "#eef0f6",
  surfaceLow: "#e2e4ec",
  primary: "#2f6bdc",
  primaryForeground: "#ffffff",
  secondary: "#7a3fc0",
  secondaryForeground: "#ffffff",
  text: "#1b1b24",
  textMuted: "#5a5a69",
  textDim: "#8c8ca0",
  border: "#c4c8d4",
  borderFocused: "#2f6bdc",
  accent: "#1f9e7a",
  accentForeground: "#ffffff",
  error: "#c0392b",
  warning: "#b8860b",
  success: "#2e8b57",
  info: "#2f6bdc",
  scrollbar: "#eef0f6",
  scrollbarThumb: "#8c8ca0",
};

export const exampleThemes: Record<ExampleThemeNameLiteral, Theme> = {
  dark: makeTheme("dark", darkColors),
  light: makeTheme("light", lightColors),
};
