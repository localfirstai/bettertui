import { Box, Text, createTestRenderer } from "@bettertui/core";
import type { TestRendererSetup } from "@bettertui/core";
import { afterEach, describe, expect, it } from "vitest";

const ESC = "\u001b";
const ANSI_RE = new RegExp(
  `${ESC}\\[[0-9;?=!><]*[A-Za-z~@^]|${ESC}][^\\u0007${ESC}]*(?:\\u0007|${ESC}\\\\)|${ESC}[^[]`,
  "g",
);

function stripAnsi(s: string): string {
  return s.replace(ANSI_RE, "").replace(new RegExp(ESC, "g"), "");
}

interface Cell {
  ch: string;
  fg: string;
  bg: string;
}

function parseGrid(ansi: string, width: number, height: number): Cell[][] {
  const grid: Cell[][] = Array.from({ length: height }, () =>
    Array.from({ length: width }, () => ({
      ch: " ",
      fg: "default",
      bg: "default",
    })),
  );

  let cx = 0;
  let cy = 0;
  let fg = "default";
  let bg = "default";
  let i = 0;

  while (i < ansi.length) {
    if (ansi[i] !== ESC) {
      if (ansi[i] === "\n") {
        cy++;
        cx = 0;
      } else if (cy < height && cx < width) {
        const cell = grid[cy]?.[cx];
        if (cell) {
          cell.ch = ansi[i] ?? " ";
          cell.fg = fg;
          cell.bg = bg;
        }
        cx++;
      }
      i++;
      continue;
    }

    i++;
    if (i >= ansi.length) break;

    if (ansi[i] === "[") {
      i++;
      const paramStart = i;
      while (i < ansi.length && (ansi[i] ?? "") >= " " && (ansi[i] ?? "") < "@") i++;
      const finalByte = ansi[i];
      const params = ansi.slice(paramStart, i);
      i++;

      if (finalByte === "H" || finalByte === "f") {
        const parts = params.split(";");
        cy = Math.max(0, (Number.parseInt(parts[0] ?? "1", 10) || 1) - 1);
        cx = Math.max(0, (Number.parseInt(parts[1] ?? "1", 10) || 1) - 1);
      } else if (finalByte === "m") {
        const parts = params.split(";").map((p) => Number.parseInt(p, 10) || 0);
        let pi = 0;
        while (pi < parts.length) {
          const code = parts[pi] ?? 0;
          if (code === 0) {
            fg = "default";
            bg = "default";
          } else if (code === 38 && parts[pi + 1] === 2) {
            fg = `rgb(${parts[pi + 2]},${parts[pi + 3]},${parts[pi + 4]})`;
            pi += 4;
          } else if (code === 48 && parts[pi + 1] === 2) {
            bg = `rgb(${parts[pi + 2]},${parts[pi + 3]},${parts[pi + 4]})`;
            pi += 4;
          } else if (code === 39) {
            fg = "default";
          } else if (code === 49) {
            bg = "default";
          }
          pi++;
        }
      }
    } else if (ansi[i] === "]") {
      while (i < ansi.length && ansi[i] !== "\x07" && !(ansi[i] === ESC && ansi[i + 1] === "\\"))
        i++;
      if (ansi[i] === ESC) i += 2;
      else i++;
    } else {
      i++;
    }
  }

  return grid;
}

describe("absolute-positioned button: Box with Text child", () => {
  const W = 80;
  const H = 24;
  let setup: TestRendererSetup | undefined;

  afterEach(() => {
    setup?.cleanup();
    setup = undefined;
  });

  it("mounts without errors", async () => {
    setup = await createTestRenderer({ width: W, height: H });
    const { renderer } = setup;
    expect(() => {
      const box = new Box(renderer, {
        id: "btn",
        zIndex: 150,
        width: 16,
        height: 3,
        backgroundColor: "#F85149",
        borderStyle: "single",
        borderColor: "#FFFFFF",
        position: "absolute",
        bottom: 1,
        right: 1,
        border: true,
      });
      const text = new Text(renderer, {
        id: "btn-text",
        content: "BOTTOM RIGHT",
        fg: "#FFFFFF",
        bg: "transparent",
        flexGrow: 1,
        flexShrink: 1,
        textAlign: "center",
      });
      box.add(text);
      renderer.root.add(box);
    }).not.toThrow();
  });

  it("produces a non-empty frame", async () => {
    setup = await createTestRenderer({ width: W, height: H });
    const box = new Box(setup.renderer, {
      id: "btn",
      zIndex: 150,
      width: 16,
      height: 3,
      backgroundColor: "#F85149",
      borderStyle: "single",
      borderColor: "#FFFFFF",
      position: "absolute",
      bottom: 1,
      right: 1,
      border: true,
    });
    const text = new Text(setup.renderer, {
      id: "btn-text",
      content: "BOTTOM RIGHT",
      fg: "#FFFFFF",
      bg: "transparent",
      flexGrow: 1,
      flexShrink: 1,
      textAlign: "center",
    });
    box.add(text);
    setup.renderer.root.add(box);
    await new Promise<void>((r) => setTimeout(r, 20));
    setup.renderOnce();
    const frame = setup.captureFrame();
    expect(frame.length).toBeGreaterThan(50);
  });

  it("renders 'BOTTOM RIGHT' text in the frame", async () => {
    setup = await createTestRenderer({ width: W, height: H });
    setup.renderer.setBackgroundColor("#0D1117");

    const box = new Box(setup.renderer, {
      id: "btn",
      zIndex: 150,
      width: 16,
      height: 3,
      backgroundColor: "#F85149",
      borderStyle: "single",
      borderColor: "#FFFFFF",
      position: "absolute",
      bottom: 1,
      right: 1,
      flexDirection: "row",
      alignItems: "center",
      justifyContent: "center",
      border: true,
    });
    const text = new Text(setup.renderer, {
      id: "btn-text",
      content: "BOTTOM RIGHT",
      fg: "#FFFFFF",
      bg: "transparent",
      flexGrow: 1,
      flexShrink: 1,
      textAlign: "center",
    });
    box.add(text);
    setup.renderer.root.add(box);

    await new Promise<void>((r) => setTimeout(r, 20));
    setup.renderOnce();
    const frame = setup.captureFrame();
    const plain = stripAnsi(frame);

    expect(plain).toContain("BOTTOM RIGHT");
  });

  it("text cells have red background and white foreground", async () => {
    setup = await createTestRenderer({ width: W, height: H });
    setup.renderer.setBackgroundColor("#0D1117");

    const box = new Box(setup.renderer, {
      id: "btn",
      zIndex: 150,
      width: 16,
      height: 3,
      backgroundColor: "#F85149",
      borderStyle: "single",
      borderColor: "#FFFFFF",
      position: "absolute",
      bottom: 1,
      right: 1,
      flexDirection: "row",
      alignItems: "center",
      justifyContent: "center",
      border: true,
    });
    const text = new Text(setup.renderer, {
      id: "btn-text",
      content: "BOTTOM RIGHT",
      fg: "#FFFFFF",
      bg: "transparent",
      flexGrow: 1,
      flexShrink: 1,
      textAlign: "center",
    });
    box.add(text);
    setup.renderer.root.add(box);

    await new Promise<void>((r) => setTimeout(r, 20));
    setup.renderOnce();
    const frame = setup.captureFrame();
    const grid = parseGrid(frame, W, H);

    // Box: bottom:1, right:1 in 80x24 → top-left corner at col=63, row=20
    // Inner content row: row=21 (top border at row=20)
    const innerRow = 21;

    const textCells = grid[innerRow]?.filter((c) => "BOTTOMRIGH".includes(c.ch) && c.ch !== " ");
    expect(textCells?.length).toBeGreaterThan(0);

    const redBg = textCells?.every((c) => c.bg.includes("248"));
    expect(redBg).toBe(true);

    const whiteFg = textCells?.every((c) => c.fg.includes("255"));
    expect(whiteFg).toBe(true);
  });
});
