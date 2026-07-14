import { describe, expect, it } from "vitest";
import type {
  AlignItems,
  AlignSelf,
  BorderStyle,
  FlexDirection,
  Gap,
  Inset,
  JustifyContent,
  KeyEvent,
  LayoutConstraints,
  Margin,
  MouseButton,
  MouseEvent,
  Overflow,
  Padding,
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

describe("Point", () => {
  it("validates point geometry", () => {
    const p: Point = { x: 10, y: 20 };
    expect(p.x).toBe(10);
    expect(p.y).toBe(20);
  });

  it("supports negative coordinates", () => {
    const p: Point = { x: -5, y: -10 };
    expect(p.x).toBe(-5);
    expect(p.y).toBe(-10);
  });

  it("supports zero coordinates", () => {
    const p: Point = { x: 0, y: 0 };
    expect(p.x).toBe(0);
  });

  it("supports fractional coordinates", () => {
    const p: Point = { x: 1.5, y: 2.75 };
    expect(p.x).toBe(1.5);
    expect(p.y).toBe(2.75);
  });
});

describe("Size", () => {
  it("validates size dimensions", () => {
    const s: Size = { width: 100, height: 50 };
    expect(s.width).toBe(100);
    expect(s.height).toBe(50);
  });

  it("supports zero size", () => {
    const s: Size = { width: 0, height: 0 };
    expect(s.width).toBe(0);
  });
});

describe("Rect", () => {
  it("validates rect with all properties", () => {
    const r: Rect = { x: 0, y: 0, width: 80, height: 24 };
    expect(r.x).toBe(0);
    expect(r.y).toBe(0);
    expect(r.width).toBe(80);
    expect(r.height).toBe(24);
  });

  it("supports rect at arbitrary positions", () => {
    const r: Rect = { x: 10, y: 5, width: 40, height: 12 };
    expect(r.width).toBe(40);
    expect(r.height).toBe(12);
  });
});

describe("KeyEvent", () => {
  it("validates a basic key event", () => {
    const k: KeyEvent = {
      key: "a",
      code: "KeyA",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
    };
    expect(k.key).toBe("a");
    expect(k.code).toBe("KeyA");
  });

  it("supports modifier keys", () => {
    const k: KeyEvent = {
      key: "c",
      code: "KeyC",
      ctrl: true,
      shift: false,
      alt: false,
      meta: false,
    };
    expect(k.ctrl).toBe(true);
  });

  it("supports all modifiers simultaneously", () => {
    const k: KeyEvent = {
      key: "z",
      code: "KeyZ",
      ctrl: true,
      shift: true,
      alt: true,
      meta: true,
    };
    expect(k.ctrl && k.shift && k.alt && k.meta).toBe(true);
  });

  it("supports special keys", () => {
    const k: KeyEvent = {
      key: "Escape",
      code: "Escape",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
    };
    expect(k.key).toBe("Escape");
  });

  it("supports function keys", () => {
    const k: KeyEvent = {
      key: "F1",
      code: "F1",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
    };
    expect(k.key).toBe("F1");
  });
});

describe("MouseEvent", () => {
  it("validates a basic mouse event", () => {
    const m: MouseEvent = {
      button: "left",
      position: { x: 5, y: 10 },
      ctrl: false,
      shift: false,
      alt: false,
    };
    expect(m.button).toBe("left");
    expect(m.position.x).toBe(5);
  });

  it("supports all mouse buttons", () => {
    const buttons: MouseButton[] = ["left", "right", "middle", "none"];
    for (const button of buttons) {
      const m: MouseEvent = {
        button,
        position: { x: 0, y: 0 },
        ctrl: false,
        shift: false,
        alt: false,
      };
      expect(m.button).toBe(button);
    }
  });

  it("supports mouse event with modifiers", () => {
    const m: MouseEvent = {
      button: "right",
      position: { x: 100, y: 200 },
      ctrl: true,
      shift: true,
      alt: false,
    };
    expect(m.button).toBe("right");
    expect(m.ctrl).toBe(true);
  });
});

describe("Style", () => {
  it("supports empty style", () => {
    const s: Style = {};
    expect(Object.keys(s)).toHaveLength(0);
  });

  it("supports foreground and background colors", () => {
    const s: Style = { fg: "red", bg: "black" };
    expect(s.fg).toBe("red");
    expect(s.bg).toBe("black");
  });

  it("supports text attributes", () => {
    const s: Style = {
      bold: true,
      italic: true,
      underline: true,
      dim: true,
      strikethrough: true,
      inverse: true,
    };
    expect(s.bold).toBe(true);
    expect(s.italic).toBe(true);
    expect(s.underline).toBe(true);
    expect(s.dim).toBe(true);
    expect(s.strikethrough).toBe(true);
    expect(s.inverse).toBe(true);
  });

  it("supports partial text attributes", () => {
    const s: Style = { bold: true };
    expect(s.bold).toBe(true);
    expect(s.italic).toBeUndefined();
  });
});

describe("BorderStyle", () => {
  it("supports all border styles", () => {
    const styles = ["none", "solid", "dashed", "dotted", "double"] as const;
    for (const style of styles) {
      const b: BorderStyle = { style, fg: "#fff" };
      expect(b.style).toBe(style);
    }
  });

  it("supports border without color", () => {
    const b: BorderStyle = { style: "solid" };
    expect(b.style).toBe("solid");
    expect(b.fg).toBeUndefined();
  });
});

describe("FlexDirection", () => {
  it("supports all flex directions", () => {
    const directions: FlexDirection[] = ["row", "column", "row-reverse", "column-reverse"];
    expect(directions).toHaveLength(4);
  });
});

describe("JustifyContent", () => {
  it("supports all justify content values", () => {
    const values: JustifyContent[] = [
      "flex-start",
      "center",
      "flex-end",
      "space-between",
      "space-around",
      "space-evenly",
    ];
    expect(values).toHaveLength(6);
  });
});

describe("AlignItems and AlignSelf", () => {
  it("supports all align items values", () => {
    const values: AlignItems[] = ["flex-start", "center", "flex-end", "stretch", "baseline"];
    expect(values).toHaveLength(5);
  });

  it("supports all align self values", () => {
    const values: AlignSelf[] = ["flex-start", "center", "flex-end", "stretch", "baseline"];
    expect(values).toHaveLength(5);
  });
});

describe("Position and Overflow", () => {
  it("supports all position values", () => {
    const positions: Position[] = ["relative", "absolute"];
    expect(positions).toHaveLength(2);
  });

  it("supports all overflow values", () => {
    const overflows: Overflow[] = ["visible", "hidden", "scroll"];
    expect(overflows).toHaveLength(3);
  });
});

describe("Sizing", () => {
  it("supports number sizing", () => {
    const s: Sizing = 100;
    expect(typeof s).toBe("number");
  });

  it("supports percentage string sizing", () => {
    const s: Sizing = "50%";
    expect(typeof s).toBe("string");
  });

  it("supports calc string sizing", () => {
    const s: Sizing = "calc(100% - 20px)";
    expect(s).toBe("calc(100% - 20px)");
  });

  it("supports auto sizing", () => {
    const s: Sizing = "auto";
    expect(s).toBe("auto");
  });
});

describe("Padding", () => {
  it("supports partial padding", () => {
    const p: Padding = { top: 1, bottom: 2 };
    expect(p.top).toBe(1);
    expect(p.bottom).toBe(2);
    expect(p.left).toBeUndefined();
    expect(p.right).toBeUndefined();
  });

  it("supports full padding", () => {
    const p: Padding = { top: 1, right: 2, bottom: 3, left: 4 };
    expect(p.top).toBe(1);
    expect(p.right).toBe(2);
    expect(p.bottom).toBe(3);
    expect(p.left).toBe(4);
  });

  it("supports zero padding", () => {
    const p: Padding = { top: 0 };
    expect(p.top).toBe(0);
  });
});

describe("Margin", () => {
  it("supports partial margin", () => {
    const m: Margin = { top: 1, bottom: 2 };
    expect(m.top).toBe(1);
    expect(m.bottom).toBe(2);
    expect(m.left).toBeUndefined();
    expect(m.right).toBeUndefined();
  });
});

describe("Inset", () => {
  it("supports partial inset", () => {
    const i: Inset = { top: 1, left: 2 };
    expect(i.top).toBe(1);
    expect(i.left).toBe(2);
    expect(i.right).toBeUndefined();
    expect(i.bottom).toBeUndefined();
  });

  it("supports full inset", () => {
    const i: Inset = { top: 1, right: 2, bottom: 3, left: 4 };
    expect(i.top).toBe(1);
    expect(i.right).toBe(2);
    expect(i.bottom).toBe(3);
    expect(i.left).toBe(4);
  });
});

describe("Gap", () => {
  it("supports row and column gap", () => {
    const g: Gap = { row: 4, column: 8 };
    expect(g.row).toBe(4);
    expect(g.column).toBe(8);
  });

  it("supports partial gap", () => {
    const g: Gap = { row: 4 };
    expect(g.row).toBe(4);
    expect(g.column).toBeUndefined();
  });
});

describe("LayoutConstraints", () => {
  it("supports empty constraints", () => {
    const l: LayoutConstraints = {};
    expect(Object.keys(l)).toHaveLength(0);
  });

  it("supports display values", () => {
    const l1: LayoutConstraints = { display: "flex" };
    const l2: LayoutConstraints = { display: "none" };
    expect(l1.display).toBe("flex");
    expect(l2.display).toBe("none");
  });

  it("supports flex properties", () => {
    const l: LayoutConstraints = {
      display: "flex",
      flexDirection: "column",
      justifyContent: "center",
      alignItems: "stretch",
      alignSelf: "flex-start",
      flexGrow: 1,
      flexShrink: 0,
      flexBasis: "auto",
      flexWrap: "wrap",
    };
    expect(l.flexDirection).toBe("column");
    expect(l.flexGrow).toBe(1);
    expect(l.flexShrink).toBe(0);
    expect(l.flexBasis).toBe("auto");
    expect(l.flexWrap).toBe("wrap");
  });

  it("supports width and height as numbers", () => {
    const l: LayoutConstraints = { width: 800, height: 600 };
    expect(l.width).toBe(800);
    expect(l.height).toBe(600);
  });

  it("supports width and height as percentages", () => {
    const l: LayoutConstraints = { width: "100%", height: "50%" };
    expect(l.width).toBe("100%");
    expect(l.height).toBe("50%");
  });

  it("supports min/max constraints", () => {
    const l: LayoutConstraints = {
      minWidth: 100,
      maxWidth: 800,
      minHeight: 50,
      maxHeight: 600,
    };
    expect(l.minWidth).toBe(100);
    expect(l.maxWidth).toBe(800);
    expect(l.minHeight).toBe(50);
    expect(l.maxHeight).toBe(600);
  });

  it("supports position and inset", () => {
    const l: LayoutConstraints = {
      position: "absolute",
      top: 10,
      right: 20,
      bottom: 30,
      left: 40,
    };
    expect(l.position).toBe("absolute");
    expect(l.top).toBe(10);
    expect(l.left).toBe(40);
  });

  it("supports zIndex", () => {
    const l: LayoutConstraints = { zIndex: 100 };
    expect(l.zIndex).toBe(100);
  });

  it("supports visibility", () => {
    const l: LayoutConstraints = { visible: false };
    expect(l.visible).toBe(false);
  });

  it("supports overflow", () => {
    const l: LayoutConstraints = { overflow: "hidden" };
    expect(l.overflow).toBe("hidden");
  });

  it("supports padding/margin as numbers", () => {
    const l: LayoutConstraints = { padding: 8, margin: 16 };
    expect(l.padding).toBe(8);
    expect(l.margin).toBe(16);
  });

  it("supports individual padding/margin props", () => {
    const l: LayoutConstraints = {
      paddingTop: 1,
      paddingRight: 2,
      paddingBottom: 3,
      paddingLeft: 4,
      marginTop: 5,
      marginRight: 6,
      marginBottom: 7,
      marginLeft: 8,
    };
    expect(l.paddingTop).toBe(1);
    expect(l.marginBottom).toBe(7);
  });

  it("supports gap as number", () => {
    const l: LayoutConstraints = { gap: 8 };
    expect(l.gap).toBe(8);
  });
});

describe("ThemeColors", () => {
  it("supports full theme colors", () => {
    const c: ThemeColors = {
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
    expect(c.background).toBe("#000");
    expect(c.text).toBe("#fff");
    expect(c.error).toBe("#f00");
  });
});

describe("ThemeSpacing", () => {
  it("supports full spacing scale", () => {
    const s: ThemeSpacing = { none: 0, xxs: 1, xs: 2, sm: 4, md: 8, lg: 16, xl: 24, xxl: 32 };
    expect(s.none).toBe(0);
    expect(s.md).toBe(8);
    expect(s.xxl).toBe(32);
  });
});

describe("Theme", () => {
  it("supports full theme structure", () => {
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
      scrollbar: "#333",
      scrollbarThumb: "#666",
    };
    const spacing: ThemeSpacing = { none: 0, xxs: 1, xs: 2, sm: 4, md: 8, lg: 16, xl: 24, xxl: 32 };
    const t: Theme = {
      name: "test-theme",
      colors,
      spacing,
      borders: { style: "solid", fg: "#fff" },
    };
    expect(t.name).toBe("test-theme");
    expect(t.borders.style).toBe("solid");
  });
});
