import { describe, expect, it } from "vitest";
import { createTheme, defaultTheme } from "../index";

describe("createTheme edge cases", () => {
  it("returns default theme when given empty overrides", () => {
    const theme = createTheme({});
    expect(theme).toEqual(defaultTheme);
  });

  it("partial color override preserves all other colors", () => {
    const theme = createTheme({ colors: { ...defaultTheme.colors, primary: "#ff0000" } });
    expect(theme.colors.primary).toBe("#ff0000");
    expect(theme.colors.background).toBe(defaultTheme.colors.background);
    expect(theme.colors.text).toBe(defaultTheme.colors.text);
    expect(theme.colors.border).toBe(defaultTheme.colors.border);
  });

  it("partial spacing override preserves all other spacings", () => {
    const theme = createTheme({ spacing: { ...defaultTheme.spacing, md: 16 } });
    expect(theme.spacing.md).toBe(16);
    expect(theme.spacing.sm).toBe(defaultTheme.spacing.sm);
    expect(theme.spacing.lg).toBe(defaultTheme.spacing.lg);
  });

  it("overriding individual border properties", () => {
    const theme = createTheme({ borders: { style: "double" } });
    expect(theme.borders.style).toBe("double");
    expect(theme.borders.fg).toBe(defaultTheme.borders.fg);
  });

  it("overriding everything at once", () => {
    const newColors = Object.fromEntries(
      Object.entries(defaultTheme.colors).map(([k]) => [k, "#000000"]),
    ) as typeof defaultTheme.colors;
    const theme = createTheme({
      name: "dark-mode",
      colors: newColors,
      spacing: { none: 0, xxs: 2, xs: 4, sm: 8, md: 16, lg: 24, xl: 32, xxl: 48 },
      borders: { style: "rounded", fg: "#ffffff" },
    });
    expect(theme.name).toBe("dark-mode");
    for (const value of Object.values(theme.colors)) {
      expect(value).toBe("#000000");
    }
    expect(theme.spacing.md).toBe(16);
    expect(theme.borders.style).toBe("rounded");
  });
});

describe("defaultTheme immutability", () => {
  it("createTheme does not mutate defaultTheme", () => {
    const original = { ...defaultTheme };
    createTheme({ name: "mutated" });
    expect(defaultTheme.name).toBe(original.name);
  });

  it("multiple createTheme calls produce independent themes", () => {
    const theme1 = createTheme({ name: "first" });
    const theme2 = createTheme({ name: "second" });
    expect(theme1.name).toBe("first");
    expect(theme2.name).toBe("second");
  });
});

describe("defaultTheme structure", () => {
  it("all spacing values are numbers", () => {
    for (const value of Object.values(defaultTheme.spacing)) {
      expect(typeof value).toBe("number");
    }
  });

  it("all color values are strings", () => {
    for (const value of Object.values(defaultTheme.colors)) {
      expect(typeof value).toBe("string");
    }
  });

  it("spacing increases monotonically", () => {
    const values = Object.values(defaultTheme.spacing);
    for (let i = 1; i < values.length; i++) {
      expect(values[i]).toBeGreaterThanOrEqual(values[i - 1]);
    }
  });
});
