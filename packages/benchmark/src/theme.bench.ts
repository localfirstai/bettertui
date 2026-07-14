import type { Theme } from "@bettertui/shared";
import { bench, describe } from "vitest";

function createTheme(overrides: Partial<Theme>): Theme {
  return {
    name: "default",
    ...overrides,
    colors: { ...defaultColors, ...overrides.colors },
    spacing: { ...defaultSpacing, ...overrides.spacing },
    borders: { ...defaultBorders, ...overrides.borders },
  };
}

const defaultColors: Theme["colors"] = {
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
  scrollbar: "#3c3c3c",
  scrollbarThumb: "#666666",
};

const defaultSpacing: Theme["spacing"] = {
  none: 0,
  xxs: 1,
  xs: 2,
  sm: 4,
  md: 8,
  lg: 12,
  xl: 16,
  xxl: 24,
};

const defaultBorders: Theme["borders"] = {
  style: "solid",
  fg: "#666666",
};

const defaultTheme: Theme = createTheme({});

describe("theme resolution", () => {
  bench("create default theme", () => {
    createTheme({});
  });

  bench("create custom theme", () => {
    createTheme({
      name: "custom",
      colors: {
        background: "#1a1a2e",
        surface: "#16213e",
        surfaceHigh: "#1f2b47",
        surfaceLow: "#111827",
        primary: "#0f3460",
        primaryForeground: "#ffffff",
        secondary: "#533483",
        secondaryForeground: "#ffffff",
        text: "#e6e6e6",
        textMuted: "#a0a0a0",
        textDim: "#707070",
        border: "#333333",
        borderFocused: "#0f3460",
        accent: "#533483",
        accentForeground: "#ffffff",
        error: "#ff4444",
        warning: "#ffaa00",
        success: "#44ff44",
        info: "#4488ff",
        scrollbar: "#333333",
        scrollbarThumb: "#707070",
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
    });
  });

  bench("access theme colors", () => {
    const theme = defaultTheme;
    void theme.colors.background;
    void theme.colors.primary;
    void theme.colors.text;
  });

  bench("access theme spacing", () => {
    const theme = defaultTheme;
    void theme.spacing.sm;
    void theme.spacing.md;
    void theme.spacing.lg;
  });
});

describe("theme creation overhead", () => {
  bench("10 theme creations", () => {
    for (let i = 0; i < 10; i++) {
      createTheme({});
    }
  });

  bench("100 theme creations", () => {
    for (let i = 0; i < 100; i++) {
      createTheme({});
    }
  });
});
