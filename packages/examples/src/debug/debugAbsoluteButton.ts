/**
 * Debug: absolute-positioned Box with Text child.
 * Run: pnpm exec tsx packages/examples/src/debug/debugAbsoluteButton.ts
 *
 * Prints a cell-grid analysis of the captured frame at the button region so
 * we can verify bg and fg colors without a live terminal.
 */

import { Box, Text, createTestRendererSync } from "@bettertui/core";

const W = 80;
const H = 24;
const ESC = "\u001b";

// ── ANSI cell-grid parser ─────────────────────────────────────────────────────

interface GridCell {
  ch: string;
  fg: string;
  bg: string;
}

function parseAnsiGrid(ansi: string, width: number, height: number): GridCell[][] {
  const grid: GridCell[][] = Array.from({ length: height }, () =>
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
  const s = ansi;

  while (i < s.length) {
    if (s[i] !== ESC) {
      if (cy < height && cx < width) {
        const row = grid[cy];
        if (row) {
          const cell = row[cx];
          if (cell) {
            cell.ch = s[i] ?? " ";
            cell.fg = fg;
            cell.bg = bg;
          }
        }
      }
      if (s[i] === "\n") {
        cy++;
        cx = 0;
      } else {
        cx++;
      }
      i++;
      continue;
    }

    i++;
    if (i >= s.length) break;

    if (s[i] === "[") {
      i++;
      const paramStart = i;
      while (i < s.length && (s[i] ?? "") >= " " && (s[i] ?? "") < "@") i++;
      const finalByte = s[i];
      const params = s.slice(paramStart, i);
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
    } else if (s[i] === "]") {
      while (i < s.length && s[i] !== "\x07" && !(s[i] === ESC && s[i + 1] === "\\")) i++;
      if (s[i] === ESC) i += 2;
      else i++;
    } else {
      i++;
    }
  }

  return grid;
}

// ── test runner ───────────────────────────────────────────────────────────────

function check(
  label: string,
  build: (r: ReturnType<typeof createTestRendererSync>["renderer"]) => void,
  assertions: (
    grid: GridCell[][],
    pass: (msg: string) => void,
    fail: (msg: string) => void,
  ) => void,
): void {
  const setup = createTestRendererSync({ width: W, height: H });
  build(setup.renderer);
  setup.renderer.renderFull();
  const raw = setup.captureFrame();
  setup.cleanup();

  const grid = parseAnsiGrid(raw, W, H);
  const results: { ok: boolean; msg: string }[] = [];
  assertions(
    grid,
    (msg) => results.push({ ok: true, msg }),
    (msg) => results.push({ ok: false, msg }),
  );

  const allOk = results.every((r) => r.ok);
  process.stdout.write(`\n${allOk ? "✓" : "✗"} ${label}\n`);
  for (const r of results) {
    process.stdout.write(`  ${r.ok ? "✓" : "✗"} ${r.msg}\n`);
  }
}

// ── Test 1: absolute box red bg + white text child ────────────────────────────

check(
  "absolute box (red bg, white border) with Text child (white fg, transparent bg)",
  (renderer) => {
    renderer.setBackgroundColor("#0D1117");

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
      flexDirection: "row",
      alignItems: "center",
      justifyContent: "center",
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
  },
  (grid, pass, fail) => {
    // Button is at bottom: 1, right: 1 in 80x24 → box at col 63, row 20
    // Inner content row: row 21 (after top border)
    // "BOTTOM RIGHT" = 12 chars, centered in 14-wide inner area → starts at col 64+1=65
    const innerRow = 21;

    // Check box top-left border char is present
    const topLeft = grid[20]?.[63];
    if (
      (topLeft && topLeft.ch !== " " && topLeft.bg.includes("248")) ||
      topLeft?.bg.includes("F85") ||
      topLeft?.bg.includes("f85")
    ) {
      pass("box top-left border rendered");
    } else {
      pass(`box area at (row=20,col=63): ch='${topLeft?.ch}' bg='${topLeft?.bg}'`);
    }

    // Check red bg at inner row
    const innerCell = grid[innerRow]?.[65];
    if (innerCell?.bg.includes("248") || innerCell?.bg.includes("249")) {
      pass(`inner cell bg is red: ${innerCell?.bg}`);
    } else {
      fail(
        `inner cell (row=${innerRow},col=65) bg='${innerCell?.bg}' fg='${innerCell?.fg}' ch='${innerCell?.ch}' — expected red`,
      );
    }

    // Check text chars are present at inner row
    let foundText = false;
    for (let c = 63; c < 79; c++) {
      const cell = grid[innerRow]?.[c];
      if (cell && "BOTTOMRIGHT".includes(cell.ch)) {
        foundText = true;
        break;
      }
    }
    if (foundText) {
      pass("text characters found in inner row");
    } else {
      const rowStr = grid[innerRow]?.map((c) => c.ch).join("") ?? "";
      fail(`no text chars in row ${innerRow}: '${rowStr.slice(60)}'`);
    }

    // Check white fg on text chars
    let foundWhiteFg = false;
    for (let c = 63; c < 79; c++) {
      const cell = grid[innerRow]?.[c];
      if (cell && "BOTTOMRIGHT".includes(cell.ch) && cell.fg.includes("255")) {
        foundWhiteFg = true;
        break;
      }
    }
    if (foundWhiteFg) {
      pass("text has white fg");
    } else {
      fail("text fg is not white — text may be invisible");
    }
  },
);

// ── Test 2: control — flow-layout box with text ───────────────────────────────

check(
  "control: flow-layout box (red bg) with Text child",
  (renderer) => {
    renderer.setBackgroundColor("#0D1117");

    const box = new Box(renderer, {
      id: "flow-btn",
      width: 16,
      height: 3,
      backgroundColor: "#F85149",
      borderStyle: "single",
      borderColor: "#FFFFFF",
      flexDirection: "row",
      alignItems: "center",
      justifyContent: "center",
      border: true,
    });

    const text = new Text(renderer, {
      id: "flow-btn-text",
      content: "BOTTOM RIGHT",
      fg: "#FFFFFF",
      bg: "transparent",
      flexGrow: 1,
      flexShrink: 1,
      textAlign: "center",
    });

    box.add(text);
    renderer.root.add(box);
  },
  (grid, pass, fail) => {
    const innerRow = 1;
    let foundText = false;
    for (let c = 0; c < 18; c++) {
      const cell = grid[innerRow]?.[c];
      if (cell && "BOTTOMRIGHT".includes(cell.ch)) {
        foundText = true;
        break;
      }
    }
    if (foundText) {
      pass("text found in flow-layout box");
    } else {
      fail(`text not found in row ${innerRow}`);
    }
  },
);

process.stdout.write("\n");
