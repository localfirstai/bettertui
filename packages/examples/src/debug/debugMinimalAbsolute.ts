/**
 * Minimal diagnostic: does a single absolute-positioned Box render at all?
 * Run: pnpm exec tsx packages/examples/src/debug/debugMinimalAbsolute.ts
 */

import { Box, Text, createTestRenderer } from "@bettertui/core";

const log = (...a: unknown[]) => process.stderr.write(`${a.join(" ")}\n`);

const ESC = "\u001b";
const ANSI_RE = new RegExp(
  `${ESC}\\[[0-9;?=!><]*[A-Za-z~@^]|${ESC}][^\\u0007${ESC}]*(?:\\u0007|${ESC}\\\\)|${ESC}[^[]`,
  "g",
);
function stripAnsi(s: string): string {
  return s.replace(ANSI_RE, "").replace(new RegExp(ESC, "g"), "");
}

async function testCase(
  label: string,
  build: (renderer: import("@bettertui/core").CliRenderer) => void,
): Promise<void> {
  const { renderer, captureFrame, cleanup } = await createTestRenderer({
    width: 80,
    height: 24,
  });

  build(renderer);
  await new Promise<void>((r) => setTimeout(r, 20)); // 2 frame ticks at 60fps

  const raw = captureFrame();
  const plain = stripAnsi(raw);
  cleanup();

  // Find any non-space, non-control characters in the output
  const visible = plain.replace(/ /g, "").replace(/\n/g, "").trim();
  log(`\n[${label}]`);
  log(`  Frame bytes : ${raw.length}`);
  log(`  Plain chars : ${plain.length}`);
  log(`  Non-space   : ${visible.length > 0 ? JSON.stringify(visible.slice(0, 60)) : "(none)"}`);

  // Print row 5 of the grid (where our test box should appear)
  const lines = plain.split("\n");
  for (let i = 4; i < Math.min(12, lines.length); i++) {
    log(`  row ${String(i).padStart(2)} : |${(lines[i] ?? "").slice(0, 60)}|`);
  }
}

// ── TEST 1: box added directly to renderer.root (absolute, with background) ──
await testCase("1. abs box in root, no parent container", (renderer) => {
  const box = new Box(renderer, {
    id: "box1",
    position: "absolute",
    left: 5,
    top: 5,
    width: 20,
    height: 5,
    backgroundColor: "#FF0000",
    border: true,
    borderColor: "#FFFFFF",
  });
  renderer.root.add(box);
});

// ── TEST 2: box in a position:relative container (same as my demo) ────────────
await testCase("2. abs box in position:relative parent", (renderer) => {
  const parent = new Box(renderer, {
    id: "parent",
    position: "relative",
    zIndex: 1,
  });
  renderer.root.add(parent);

  const box = new Box(renderer, {
    id: "box2",
    position: "absolute",
    left: 5,
    top: 5,
    width: 20,
    height: 5,
    backgroundColor: "#00FF00",
    border: true,
    borderColor: "#FFFFFF",
  });
  parent.add(box);
});

// ── TEST 3: box in a container with width/height 100% (like Screen body) ─────
await testCase("3. abs box in 100% parent", (renderer) => {
  const parent = new Box(renderer, {
    id: "parent",
    width: "100%",
    height: "100%",
  });
  renderer.root.add(parent);

  const box = new Box(renderer, {
    id: "box3",
    position: "absolute",
    left: 5,
    top: 5,
    width: 20,
    height: 5,
    backgroundColor: "#0000FF",
    border: true,
    borderColor: "#FFFFFF",
  });
  parent.add(box);
});

// ── TEST 4: box with text (no absolute pos on parent) ─────────────────────────
await testCase("4. text node absolute in root", (renderer) => {
  const text = new Text(renderer, {
    id: "t1",
    content: "HELLO WORLD",
    position: "absolute",
    left: 5,
    top: 5,
    fg: "#FFFF00",
  });
  renderer.root.add(text);
});

// ── TEST 5: flow-layout box (should definitely show) ──────────────────────────
await testCase("5. flow layout box (control)", (renderer) => {
  const box = new Box(renderer, {
    id: "flow",
    width: 20,
    height: 5,
    backgroundColor: "#FF00FF",
    border: true,
    borderColor: "#FFFFFF",
  });
  renderer.root.add(box);
});

log("\n=== Done ===");
