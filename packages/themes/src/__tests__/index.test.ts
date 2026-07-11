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
    expect(colors.primary).toBeDefined();
    expect(colors.text).toBeDefined();
    expect(colors.border).toBeDefined();
    expect(colors.error).toBeDefined();
    expect(colors.warning).toBeDefined();
    expect(colors.success).toBeDefined();
    expect(colors.info).toBeDefined();
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

  it("has border style", () => {
    expect(defaultTheme.borders.style).toBeDefined();
    expect(defaultTheme.borders.fg).toBeDefined();
  });

  it("colors are hex strings", () => {
    for (const value of Object.values(defaultTheme.colors)) {
      expect(value).toMatch(/^#[0-9a-fA-F]{6}$/);
    }
  });
});

describe("createTheme", () => {
  it("returns default theme when no overrides", () => {
    const theme = createTheme({});
    expect(theme.name).toBe(defaultTheme.name);
    expect(theme.colors).toEqual(defaultTheme.colors);
  });

  it("overrides name", () => {
    const theme = createTheme({ name: "custom" });
    expect(theme.name).toBe("custom");
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
});
