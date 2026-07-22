import { describe, expect, it } from "vitest";
import {
  ROUNDED_BOX,
  bar,
  displayWidth,
  drawBox,
  moveTo,
  padEnd,
  padStart,
  sgr,
  sparkline,
  stripAnsi,
  truncate,
} from "../overlay/ansiUtils";

describe("moveTo", () => {
  it("builds a 1-based absolute cursor move", () => {
    expect(moveTo(3, 5)).toBe("\x1b[3;5H");
  });

  it("clamps to a minimum of 1,1", () => {
    expect(moveTo(0, -4)).toBe("\x1b[1;1H");
  });

  it("floors fractional coordinates", () => {
    expect(moveTo(2.9, 4.2)).toBe("\x1b[2;4H");
  });
});

describe("sgr", () => {
  it("wraps text with codes and a reset", () => {
    expect(sgr("hi", 1, 31)).toBe("\x1b[1;31mhi\x1b[0m");
  });

  it("returns text unchanged with no codes", () => {
    expect(sgr("hi")).toBe("hi");
  });
});

describe("stripAnsi / displayWidth", () => {
  it("strips SGR sequences", () => {
    expect(stripAnsi("\x1b[31mred\x1b[0m")).toBe("red");
  });

  it("strips save/restore cursor", () => {
    expect(stripAnsi("\x1b7abc\x1b8")).toBe("abc");
  });

  it("computes visible width ignoring escapes", () => {
    expect(displayWidth("\x1b[1mhello\x1b[0m")).toBe(5);
  });
});

describe("truncate", () => {
  it("returns text unchanged when within width", () => {
    expect(truncate("abc", 5)).toBe("abc");
  });

  it("appends an ellipsis when clipped", () => {
    expect(truncate("abcdef", 4)).toBe("abc…");
  });

  it("returns empty for non-positive width", () => {
    expect(truncate("abc", 0)).toBe("");
  });

  it("hard-clips when width is smaller than the ellipsis", () => {
    expect(truncate("abcdef", 1, "…")).toBe("a");
  });
});

describe("padEnd / padStart", () => {
  it("right-pads to width", () => {
    expect(padEnd("ab", 5)).toBe("ab   ");
  });

  it("left-pads to width", () => {
    expect(padStart("ab", 5)).toBe("   ab");
  });

  it("does not shrink longer strings", () => {
    expect(padEnd("abcdef", 3)).toBe("abcdef");
  });
});

describe("drawBox", () => {
  it("frames content with a border and title", () => {
    const box = drawBox(["hello"], { title: "T", width: 7 });
    expect(box).toHaveLength(3); // top + 1 content + bottom
    expect(box[0]?.startsWith(ROUNDED_BOX.topLeft)).toBe(true);
    expect(box[2]?.startsWith(ROUNDED_BOX.bottomLeft)).toBe(true);
  });

  it("pads content rows to a uniform width", () => {
    const box = drawBox(["a", "bb"], { width: 6 });
    const widths = box.map((l) => displayWidth(l));
    expect(new Set(widths).size).toBe(1);
    expect(widths[0]).toBe(8); // width + 2 borders
  });

  it("truncates over-long content", () => {
    const box = drawBox(["abcdefghij"], { width: 5 });
    // content row = border + 5 cols + border
    expect(displayWidth(box[1] ?? "")).toBe(7);
  });
});

describe("sparkline", () => {
  it("returns empty for empty input", () => {
    expect(sparkline([], 10)).toBe("");
  });

  it("produces one glyph per sampled value", () => {
    expect(sparkline([1, 2, 3], 10)).toHaveLength(3);
  });

  it("samples only the last `width` values", () => {
    expect(sparkline([1, 2, 3, 4, 5], 2)).toHaveLength(2);
  });
});

describe("bar", () => {
  it("fills proportionally", () => {
    expect(bar(0.5, 10)).toBe("█████░░░░░");
  });

  it("clamps above 1 and below 0", () => {
    expect(bar(2, 4)).toBe("████");
    expect(bar(-1, 4)).toBe("░░░░");
  });
});
