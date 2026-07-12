import { describe, expect, it } from "vitest";
import type {
  AlignItems,
  BorderStyle,
  FlexDirection,
  JustifyContent,
  KeyEvent,
  LayoutConstraints,
  MouseEvent,
  Overflow,
  Point,
  Position,
  Rect,
  Size,
  Sizing,
  Style,
  Theme,
  ThemeColors,
  ThemeSpacing,
} from "../index";

describe("shared types", () => {
  it("exports type aliases that compile", () => {
    const point: Point = { x: 1, y: 2 };
    expect(point.x).toBe(1);

    const size: Size = { width: 100, height: 50 };
    expect(size.width).toBe(100);

    const rect: Rect = { x: 0, y: 0, width: 80, height: 24 };
    expect(rect.width).toBe(80);

    const keyEvent: KeyEvent = {
      key: "a",
      code: "KeyA",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
    };
    expect(keyEvent.key).toBe("a");

    const mouseEvent: MouseEvent = {
      button: "left",
      position: { x: 5, y: 10 },
      ctrl: false,
      shift: false,
      alt: false,
    };
    expect(mouseEvent.button).toBe("left");

    const style: Style = { fg: "red", bg: "black", bold: true };
    expect(style.bold).toBe(true);

    const border: BorderStyle = { style: "rounded", fg: "blue" };
    expect(border.style).toBe("rounded");
  });

  it("supports all string literal types", () => {
    const directions: FlexDirection[] = ["row", "column", "row-reverse", "column-reverse"];
    expect(directions).toHaveLength(4);

    const justify: JustifyContent[] = [
      "flex-start",
      "center",
      "flex-end",
      "space-between",
      "space-around",
      "space-evenly",
    ];
    expect(justify).toHaveLength(6);

    const align: AlignItems[] = ["flex-start", "center", "flex-end", "stretch", "baseline"];
    expect(align).toHaveLength(5);

    const positions: Position[] = ["relative", "absolute"];
    expect(positions).toHaveLength(2);

    const overflows: Overflow[] = ["visible", "hidden", "scroll"];
    expect(overflows).toHaveLength(3);
  });

  it("supports Sizing as number or string", () => {
    const numSize: Sizing = 100;
    const strSize: Sizing = "50%";
    expect(typeof numSize).toBe("number");
    expect(typeof strSize).toBe("string");
  });

  it("supports LayoutConstraints with valid values", () => {
    const layout: LayoutConstraints = {
      display: "flex",
      flexDirection: "column",
      justifyContent: "center",
      alignItems: "stretch",
      width: 800,
      height: 600,
    };
    expect(layout.display).toBe("flex");
    expect(layout.flexDirection).toBe("column");
  });

  it("supports Theme structure", () => {
    const colors: ThemeColors = {
      background: "#000",
      surface: "#111",
      surfaceHigh: "#222",
      surfaceLow: "#0a0a0a",
      primary: "#fff",
      primaryForeground: "#000",
      secondary: "#555",
      secondaryForeground: "#fff",
      text: "#fff",
      textMuted: "#888",
      textDim: "#666",
      border: "#333",
      borderFocused: "#fff",
      accent: "#0ff",
      accentForeground: "#000",
      error: "#f00",
      warning: "#ff0",
      success: "#0f0",
      info: "#0ff",
    };
    expect(colors.background).toBe("#000");

    const spacing: ThemeSpacing = { none: 0, xxs: 1, xs: 2, sm: 4, md: 8, lg: 16, xl: 24, xxl: 32 };
    expect(spacing.md).toBe(8);

    const theme: Theme = {
      name: "test",
      colors,
      spacing,
      borders: { style: "single", fg: "#fff" },
    };
    expect(theme.name).toBe("test");
  });
});
