import { createTheme, defaultTheme } from "@bettertui/themes";
import { bench, describe } from "vitest";

describe("theme resolution", () => {
  bench("create default theme", () => {
    createTheme(defaultTheme);
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
    const theme = createTheme(defaultTheme);
    void theme.colors.background;
    void theme.colors.primary;
    void theme.colors.text;
  });

  bench("access theme spacing", () => {
    const theme = createTheme(defaultTheme);
    void theme.spacing.sm;
    void theme.spacing.md;
    void theme.spacing.lg;
  });
});

describe("theme creation overhead", () => {
  bench("10 theme creations", () => {
    for (let i = 0; i < 10; i++) {
      createTheme(defaultTheme);
    }
  });

  bench("100 theme creations", () => {
    for (let i = 0; i < 100; i++) {
      createTheme(defaultTheme);
    }
  });
});
