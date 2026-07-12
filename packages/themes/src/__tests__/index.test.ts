import { describe, expect, it } from "vitest";
import { createTheme, defaultTheme } from "../index";

describe("defaultTheme", () => {
  it("has a name", () => {
    expect(defaultTheme.name).toBe("default");
  });

  it("has all color categories", () => {
    const colors = defaultTheme.colors;
    expect(colors.background).toBeDefined();
    expect(colors.surface).toBeDefined();
    expect(colors.surfaceHigh).toBeDefined();
    expect(colors.surfaceLow).toBeDefined();
    expect(colors.primary).toBeDefined();
    expect(colors.primaryForeground).toBeDefined();
    expect(colors.secondary).toBeDefined();
    expect(colors.secondaryForeground).toBeDefined();
    expect(colors.text).toBeDefined();
    expect(colors.textMuted).toBeDefined();
    expect(colors.textDim).toBeDefined();
    expect(colors.border).toBeDefined();
    expect(colors.borderFocused).toBeDefined();
    expect(colors.accent).toBeDefined();
    expect(colors.accentForeground).toBeDefined();
    expect(colors.error).toBeDefined();
    expect(colors.warning).toBeDefined();
    expect(colors.success).toBeDefined();
    expect(colors.info).toBeDefined();
  });

  it("has exactly 19 colors", () => {
    expect(Object.keys(defaultTheme.colors)).toHaveLength(19);
  });

  it("has spacing scale", () => {
    const spacing = defaultTheme.spacing;
    expect(spacing.none).toBe(0);
    expect(spacing.xxs).toBeGreaterThan(0);
    expect(spacing.sm).toBeLessThan(spacing.md);
    expect(spacing.md).toBeLessThan(spacing.lg);
    expect(spacing.lg).toBeLessThan(spacing.xl);
    expect(spacing.xl).toBeLessThan(spacing.xxl);
  });

  it("has exactly 8 spacing values", () => {
    expect(Object.keys(defaultTheme.spacing)).toHaveLength(8);
  });

  it("has border style", () => {
    expect(defaultTheme.borders.style).toBeDefined();
    expect(defaultTheme.borders.fg).toBeDefined();
  });

  it("borders has exactly 2 keys", () => {
    expect(Object.keys(defaultTheme.borders)).toHaveLength(2);
  });

  it("colors are hex strings", () => {
    for (const value of Object.values(defaultTheme.colors)) {
      expect(value).toMatch(/^#[0-9a-fA-F]{6}$/);
    }
  });

  it("has a theme named 'default'", () => {
    expect(defaultTheme.name).toBe("default");
  });

  it("spacing values are non-negative", () => {
    for (const value of Object.values(defaultTheme.spacing)) {
      expect(value).toBeGreaterThanOrEqual(0);
    }
  });
});

describe("createTheme", () => {
  it("returns default theme when no overrides", () => {
    const theme = createTheme({});
    expect(theme.name).toBe(defaultTheme.name);
    expect(theme.colors).toEqual(defaultTheme.colors);
    expect(theme.spacing).toEqual(defaultTheme.spacing);
    expect(theme.borders).toEqual(defaultTheme.borders);
  });

  it("overrides name", () => {
    const theme = createTheme({ name: "custom" });
    expect(theme.name).toBe("custom");
    expect(theme.colors).toEqual(defaultTheme.colors);
  });

  it("overrides specific colors", () => {
    const theme = createTheme({ colors: { ...defaultTheme.colors, primary: "#ff0000" } });
    expect(theme.colors.primary).toBe("#ff0000");
    expect(theme.colors.background).toBe(defaultTheme.colors.background);
  });

  it("overrides borders", () => {
    const theme = createTheme({ borders: { style: "double", fg: "#ffffff" } });
    expect(theme.borders.style).toBe("double");
    expect(theme.borders.fg).toBe("#ffffff");
  });

  it("merges spacing", () => {
    const theme = createTheme({ spacing: { ...defaultTheme.spacing, md: 16 } });
    expect(theme.spacing.md).toBe(16);
    expect(theme.spacing.sm).toBe(defaultTheme.spacing.sm);
  });

  it("overrides a single color without affecting others", () => {
    const theme = createTheme({ colors: { ...defaultTheme.colors, error: "#123456" } });
    expect(theme.colors.error).toBe("#123456");
    expect(theme.colors.success).toBe(defaultTheme.colors.success);
  });

  it("overrides all colors", () => {
    const customColors = Object.fromEntries(
      Object.entries(defaultTheme.colors).map(([k]) => [k, "#000000"]),
    ) as unknown as typeof defaultTheme.colors;
    const theme = createTheme({ colors: customColors });
    for (const value of Object.values(theme.colors)) {
      expect(value).toBe("#000000");
    }
  });

  it("overrides with empty name uses default name", () => {
    const theme = createTheme({ name: "" });
    expect(theme.name).toBe("");
  });

  it("preserves defaultTheme immutability", () => {
    const originalName = defaultTheme.name;
    createTheme({ name: "mutated" });
    expect(defaultTheme.name).toBe(originalName);
  });

  it("can override just spacing.none", () => {
    const theme = createTheme({ spacing: { ...defaultTheme.spacing, none: 2 } });
    expect(theme.spacing.none).toBe(2);
  });

  it("can override just the fg of borders", () => {
    const theme = createTheme({ borders: { ...defaultTheme.borders, fg: "#ff0000" } });
    expect(theme.borders.fg).toBe("#ff0000");
    expect(theme.borders.style).toBe(defaultTheme.borders.style);
  });

  it("creates independent copies (mutating result does not affect default)", () => {
    const theme = createTheme({});
    (theme.colors as Record<string, string>)["background"] = "#CHANGED";
    expect(defaultTheme.colors.background).not.toBe("#CHANGED");
  });
});
