import type { Theme } from "@bettertui/shared";

export const defaultTheme: Theme = {
  name: "default",
  colors: {
    background: "#1e1e1e",
    surface: "#252526",
    surfaceHigh: "#2d2d2d",
    surfaceLow: "#1a1a1a",
    primary: "#007acc",
    primaryForeground: "#ffffff",
    secondary: "#6c757d",
    secondaryForeground: "#ffffff",
    text: "#d4d4d4",
    textMuted: "#808080",
    textDim: "#5a5a5a",
    border: "#3c3c3c",
    borderFocused: "#007acc",
    accent: "#007acc",
    accentForeground: "#ffffff",
    error: "#dc3545",
    warning: "#ffc107",
    success: "#28a745",
    info: "#17a2b8",
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
    style: "single",
    fg: "#666666",
  },
};

export function createTheme(overrides: Partial<Theme>): Theme {
  return {
    ...defaultTheme,
    ...overrides,
    colors: { ...defaultTheme.colors, ...overrides.colors },
    spacing: { ...defaultTheme.spacing, ...overrides.spacing },
    borders: { ...defaultTheme.borders, ...overrides.borders },
  };
}

export type { Theme, ThemeColors, ThemeSpacing } from "@bettertui/shared";
