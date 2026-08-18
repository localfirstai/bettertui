/**
 * Debug script: Headless capture of the nestedZindex example.
 *
 * Builds the same node tree as nestedZindex.example.ts using createTestRenderer
 * so no real terminal is needed. Strips ANSI and prints a readable character
 * grid so you can see exactly what is rendered.
 *
 * Run with:
 *   node --experimental-strip-types packages/examples/src/debug/debugNestedZindexV2.ts
 *   # or: pnpm exec tsx packages/examples/src/debug/debugNestedZindexV2.ts
 */

import { Box, Text, bold, createTestRenderer, t, underline } from "@bettertui/core";

// Write to the REAL stderr so output is visible even though createTestRenderer
// replaces process.stdout with the TestWriteStream.
const log = (...args: unknown[]) => process.stderr.write(`${args.join(" ")}\n`);

const ESC = "\u001b";
const ANSI_RE = new RegExp(
  `${ESC}\\[[0-9;?=!><]*[A-Za-z~@^]|${ESC}][^\\u0007${ESC}]*(?:\\u0007|${ESC}\\\\)|${ESC}[^[]`,
  "g",
);
function stripAnsi(str: string): string {
  return str.replace(ANSI_RE, "").replace(new RegExp(ESC, "g"), "");
}

// ── Set up headless renderer ────────────────────────────────────────────────

const setup = await createTestRenderer({ width: 120, height: 30 });
const { renderer, captureFrame, cleanup } = setup;

// ── Build the tree (mirrors nestedZindex.example.ts exactly) ───────────────

renderer.start();
renderer.setBackgroundColor("#001122");

const rootContainer = new Box(renderer, {
  id: "root-container",
  width: "100%",
  height: "100%",
  zIndex: 1,
});
renderer.root.add(rootContainer);

// Title
const title = new Text(renderer, {
  id: "main-title",
  content: t`${bold(underline("Nested Render Objects & Z-Index Demo"))}`,
  position: "absolute",
  left: 10,
  top: 1,
  fg: "#FFFF00",
  zIndex: 1000,
});
rootContainer.add(title);

// ── Parent Group A (top-left, z=100) ───────────────────────────────────────

const parentGroupA = new Box(renderer, {
  id: "parent-group-a",
  position: "absolute",
  left: 4,
  top: 4,
  width: 50,
  height: 16,
  zIndex: 100,
  border: true,
  borderStyle: "single",
  borderColor: "#9944FF",
  backgroundColor: "#1a0a2e",
});
rootContainer.add(parentGroupA);

const boxA1 = new Box(renderer, {
  id: "box-a1",
  position: "absolute",
  left: 2,
  top: 2,
  width: 22,
  height: 5,
  backgroundColor: "#441155",
  zIndex: 10,
  border: true,
  borderStyle: "single",
  borderColor: "#FF88FF",
  title: "A (z=100)",
  titleAlignment: "center",
});
parentGroupA.add(boxA1);

const textA1 = new Text(renderer, {
  id: "text-a1",
  content: t`${bold("Child A1")}`,
  position: "absolute",
  left: 4,
  top: 4,
  fg: "#FFFFFF",
  zIndex: 10,
});
parentGroupA.add(textA1);

const boxA2 = new Box(renderer, {
  id: "box-a2",
  position: "absolute",
  left: 2,
  top: 9,
  width: 14,
  height: 3,
  backgroundColor: "#552244",
  zIndex: 5,
  border: true,
  borderStyle: "single",
  borderColor: "#FFB8FF",
});
parentGroupA.add(boxA2);

const textA2 = new Text(renderer, {
  id: "text-a2",
  content: "A2",
  position: "absolute",
  left: 4,
  top: 10,
  fg: "#FFFFFF",
  zIndex: 5,
});
parentGroupA.add(textA2);

// ── Parent Group B (offset right+down to overlap A, z=50) ──────────────────

const parentGroupB = new Box(renderer, {
  id: "parent-group-b",
  position: "absolute",
  left: 16,
  top: 7,
  width: 50,
  height: 16,
  zIndex: 50,
  border: true,
  borderStyle: "single",
  borderColor: "#44FF44",
  backgroundColor: "#0a2e1a",
});
rootContainer.add(parentGroupB);

const boxB1 = new Box(renderer, {
  id: "box-b1",
  position: "absolute",
  left: 2,
  top: 2,
  width: 22,
  height: 5,
  backgroundColor: "#115522",
  zIndex: 20,
  border: true,
  borderStyle: "double",
  borderColor: "#88FF88",
  title: "B (z=50)",
  titleAlignment: "center",
});
parentGroupB.add(boxB1);

const textB1 = new Text(renderer, {
  id: "text-b1",
  content: t`${bold("Child B1")}`,
  position: "absolute",
  left: 4,
  top: 4,
  fg: "#FFFFFF",
  zIndex: 20,
});
parentGroupB.add(textB1);

const boxB2 = new Box(renderer, {
  id: "box-b2",
  position: "absolute",
  left: 2,
  top: 9,
  width: 14,
  height: 3,
  backgroundColor: "#226622",
  zIndex: 15,
  border: true,
  borderStyle: "single",
  borderColor: "#AAFFAA",
});
parentGroupB.add(boxB2);

const textB2 = new Text(renderer, {
  id: "text-b2",
  content: "B2",
  position: "absolute",
  left: 4,
  top: 10,
  fg: "#FFFFFF",
  zIndex: 15,
});
parentGroupB.add(textB2);

// ── Parent Group C (offset further to overlap A+B, z=20) ───────────────────

const parentGroupC = new Box(renderer, {
  id: "parent-group-c",
  position: "absolute",
  left: 28,
  top: 10,
  width: 50,
  height: 16,
  zIndex: 20,
  border: true,
  borderStyle: "single",
  borderColor: "#FFFF44",
  backgroundColor: "#2e2a0a",
});
rootContainer.add(parentGroupC);

const boxC1 = new Box(renderer, {
  id: "box-c1",
  position: "absolute",
  left: 2,
  top: 2,
  width: 22,
  height: 5,
  backgroundColor: "#554411",
  zIndex: 30,
  border: true,
  borderStyle: "round",
  borderColor: "#FFFF88",
  title: "C (z=20)",
  titleAlignment: "center",
});
parentGroupC.add(boxC1);

const textC1 = new Text(renderer, {
  id: "text-c1",
  content: t`${bold("Child C1")}`,
  position: "absolute",
  left: 4,
  top: 4,
  fg: "#FFFFFF",
  zIndex: 30,
});
parentGroupC.add(textC1);

const boxC2 = new Box(renderer, {
  id: "box-c2",
  position: "absolute",
  left: 2,
  top: 9,
  width: 14,
  height: 3,
  backgroundColor: "#444422",
  zIndex: 25,
  border: true,
  borderStyle: "single",
  borderColor: "#FFFFAA",
});
parentGroupC.add(boxC2);

const textC2 = new Text(renderer, {
  id: "text-c2",
  content: "C2",
  position: "absolute",
  left: 4,
  top: 10,
  fg: "#FFFFFF",
  zIndex: 25,
});
parentGroupC.add(textC2);

// ── Footer / status text ───────────────────────────────────────────────────

const termH = renderer.terminalHeight;

const explanation1 = new Text(renderer, {
  id: "explanation1",
  content:
    "Key Concept: Parent z-index determines group layering, child z-index determines order within group",
  position: "absolute",
  left: 2,
  top: termH - 5,
  fg: "#AAAAAA",
  zIndex: 1000,
});
rootContainer.add(explanation1);

const explanation2 = new Text(renderer, {
  id: "explanation2",
  content: "Even if Child C1 has z=30, it renders behind Parent A & B because Parent C has z=20",
  position: "absolute",
  left: 2,
  top: termH - 4,
  fg: "#AAAAAA",
  zIndex: 1000,
});
rootContainer.add(explanation2);

const phaseIndicator = new Text(renderer, {
  id: "phase-indicator",
  content: t`${bold("Animation Phase: 1/4")}`,
  position: "absolute",
  left: 2,
  top: termH - 2,
  fg: "#FFFFFF",
  zIndex: 1000,
});
rootContainer.add(phaseIndicator);

const zIndexDisplay = new Text(renderer, {
  id: "zindex-display",
  content: "Current Z-Indices - A:100, B:50, C:20",
  position: "absolute",
  left: 40,
  top: termH - 2,
  fg: "#FFFFFF",
  zIndex: 1000,
});
rootContainer.add(zIndexDisplay);

// ── Render and capture one frame ─────────────────────────────────────────
// Await one event-loop tick so the frame loop's lifecycle passes run and
// sync Text content to the Rust engine before render() reads it.

await new Promise<void>((resolve) => setTimeout(resolve, 0));

const raw = captureFrame();

// Cleanup restores process.stdout; all further output goes to real stdout/stderr
cleanup();

const plain = stripAnsi(raw);
const lines = plain.split("\n");

log("\n=== Raw output (first 200 chars, hex) ===");
log(Buffer.from(raw.slice(0, 200)).toString("hex"));

log("\n=== Plain text after ANSI strip (first 200 chars) ===");
log(JSON.stringify(plain.slice(0, 200)));

log("\n=== Visible character grid (rows 0-29, cols 0-119) ===");
log(`${"─".repeat(5)}┬${"─".repeat(121)}`);
for (let row = 0; row < Math.min(30, lines.length); row++) {
  const line = (lines[row] ?? "").padEnd(120, " ");
  const rowLabel = String(row).padStart(3);
  log(`${rowLabel}  │ ${line}`);
}
log(`${"─".repeat(5)}┴${"─".repeat(121)}`);

log(`\nRaw output size : ${raw.length} bytes`);
log(`Plain text size : ${plain.length} chars`);
log(`Visible lines   : ${lines.length}`);
log(`Contains 'Nested': ${plain.includes("Nested")}`);
log(`Contains 'Child A1': ${plain.includes("Child A1")}`);
log(`Contains 'Animation Phase': ${plain.includes("Animation Phase")}`);
log("\nDone.");
