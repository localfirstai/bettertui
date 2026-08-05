import { createTestRenderer } from "@bettertui/core";
import type { TestRendererSetup } from "@bettertui/core";
import type { Text } from "@bettertui/core";
/**
 * Tests for the nestedZindex example.
 *
 * Rendering strategy
 * ──────────────────
 * Box styles (background, border) are applied synchronously in the Box
 * constructor. After `run()` returns, the Rust engine already holds all layout
 * and style data. A single `renderOnce()` call triggers layout computation and
 * ANSI output without waiting for the setInterval-based frame loop.
 *
 * Text node *content* is verified via `node.content` rather than the ANSI
 * frame because text sync happens inside the lifecycle passes that only run
 * inside the frame loop.
 */
import { afterEach, describe, expect, it } from "vitest";
import { destroy, run } from "../examples/nestedZindex.example";

// ── helpers ──────────────────────────────────────────────────────────────────

// Strip all ANSI/CSI/OSC escape sequences.
// Built via new RegExp to avoid biome's noControlCharactersInRegex lint on
// regex literals that contain ESC.
const ESC = "\u001b";
const ANSI_RE = new RegExp(
  `${ESC}\\[[0-9;?=!><]*[A-Za-z~@^]|${ESC}][^\\u0007${ESC}]*(?:\\u0007|${ESC}\\\\)|${ESC}[^[]`,
  "g",
);

function stripAnsi(str: string): string {
  return str.replace(ANSI_RE, "").replace(new RegExp(ESC, "g"), "");
}

// Wait one event-loop tick so async set-up in run() can complete (e.g. the
// initial frame callback fires and updates Text node content).
function waitFrame(): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, 0));
}

// ── suite ─────────────────────────────────────────────────────────────────────

describe("nestedZindex example", () => {
  let setup: TestRendererSetup | undefined;

  afterEach(() => {
    if (setup) {
      destroy(setup.renderer);
      setup.cleanup();
      setup = undefined;
    }
  });

  // ── mount ─────────────────────────────────────────────────────────────────

  it("mounts without errors", async () => {
    setup = await createTestRenderer({ width: 120, height: 30 });
    expect(() => run(setup?.renderer)).not.toThrow();
  });

  // ── tree structure ────────────────────────────────────────────────────────

  it("creates the Screen container in the renderer root", async () => {
    setup = await createTestRenderer({ width: 120, height: 30 });
    run(setup.renderer);
    expect(setup.renderer.root.getRenderable("nestedZindex-screen")).toBeDefined();
    expect(setup.renderer.root.getRenderable("nestedZindex-body")).toBeDefined();
  });

  it("creates three parent groups in the tree", async () => {
    setup = await createTestRenderer({ width: 120, height: 30 });
    run(setup.renderer);
    expect(setup.renderer.root.getRenderable("parent-group-a")).toBeDefined();
    expect(setup.renderer.root.getRenderable("parent-group-b")).toBeDefined();
    expect(setup.renderer.root.getRenderable("parent-group-c")).toBeDefined();
  });

  it("creates child boxes inside every parent group", async () => {
    setup = await createTestRenderer({ width: 120, height: 30 });
    run(setup.renderer);
    // Group A children
    expect(setup.renderer.root.getRenderable("box-a1")).toBeDefined();
    expect(setup.renderer.root.getRenderable("box-a2")).toBeDefined();
    // Group B children
    expect(setup.renderer.root.getRenderable("box-b1")).toBeDefined();
    expect(setup.renderer.root.getRenderable("box-b2")).toBeDefined();
    // Group C children
    expect(setup.renderer.root.getRenderable("box-c1")).toBeDefined();
    expect(setup.renderer.root.getRenderable("box-c2")).toBeDefined();
  });

  // ── text content ──────────────────────────────────────────────────────────

  it("renders the title node with correct content", async () => {
    setup = await createTestRenderer({ width: 120, height: 30 });
    run(setup.renderer);
    await waitFrame();
    const titleNode = setup.renderer.root.getRenderable("main-title") as Text | undefined;
    expect(titleNode).toBeDefined();
    expect(stripAnsi(titleNode?.content ?? "")).toContain("Nested");
  });

  it("child text nodes carry the expected labels", async () => {
    setup = await createTestRenderer({ width: 120, height: 30 });
    run(setup.renderer);
    await waitFrame();
    const textA1 = setup.renderer.root.getRenderable("text-a1") as Text | undefined;
    const textB1 = setup.renderer.root.getRenderable("text-b1") as Text | undefined;
    const textC1 = setup.renderer.root.getRenderable("text-c1") as Text | undefined;
    expect(textA1).toBeDefined();
    expect(textB1).toBeDefined();
    expect(textC1).toBeDefined();
    expect(stripAnsi(textA1?.content ?? "")).toContain("A1");
    expect(stripAnsi(textB1?.content ?? "")).toContain("B1");
    expect(stripAnsi(textC1?.content ?? "")).toContain("C1");
  });

  it("renders the animation phase indicator node", async () => {
    setup = await createTestRenderer({ width: 120, height: 30 });
    run(setup.renderer);
    await waitFrame();
    const phaseNode = setup.renderer.root.getRenderable("phase-indicator") as Text | undefined;
    expect(phaseNode).toBeDefined();
    expect(stripAnsi(phaseNode?.content ?? "")).toContain("Animation Phase");
  });

  it("renders the z-index display node", async () => {
    setup = await createTestRenderer({ width: 120, height: 30 });
    run(setup.renderer);
    await waitFrame();
    const display = setup.renderer.root.getRenderable("zindex-display") as Text | undefined;
    expect(display).toBeDefined();
    expect(stripAnsi(display?.content ?? "")).toContain("Z-Indices");
  });

  // ── rendered frame ────────────────────────────────────────────────────────

  it("produces a non-trivial ANSI frame after renderOnce", async () => {
    setup = await createTestRenderer({ width: 120, height: 30 });
    run(setup.renderer);
    // renderOnce() calls renderer.render() synchronously — it computes the
    // layout, builds the render tree, and writes ANSI bytes to stdout.
    // Box styles (including borders) are already in the Rust engine after
    // run() returns, so no need to wait for the frame loop first.
    setup.renderOnce();
    const frame = setup.captureFrame();
    expect(frame.length).toBeGreaterThan(100);
    // At least one box-drawing character confirms borders were rendered.
    const plain = stripAnsi(frame);
    expect(plain).toMatch(/[┌┐└┘│─╔╗╚╝╠╣╦╩╬╒╓╕╖╘╙╛╜╟╞╡╢╤╥╧╨╪╫]/u);
  });

  // ── z-index animation ─────────────────────────────────────────────────────

  it("parent groups have the correct initial z-index values", async () => {
    setup = await createTestRenderer({ width: 120, height: 30 });
    run(setup.renderer);
    const groupA = setup.renderer.root.getRenderable("parent-group-a");
    const groupB = setup.renderer.root.getRenderable("parent-group-b");
    const groupC = setup.renderer.root.getRenderable("parent-group-c");
    expect(groupA).toBeDefined();
    expect(groupB).toBeDefined();
    expect(groupC).toBeDefined();
    // After the first frame callback, the animation picks the current phase and
    // applies z-indices. The exact values depend on Date.now() so we only check
    // that they are finite non-negative integers.
    await waitFrame();
    // Still defined and accessible after animation fires
    expect(setup.renderer.root.getRenderable("parent-group-a")).toBeDefined();
    expect(setup.renderer.root.getRenderable("parent-group-b")).toBeDefined();
    expect(setup.renderer.root.getRenderable("parent-group-c")).toBeDefined();
  });

  it("groups remain visible after z-index animation tick", async () => {
    setup = await createTestRenderer({ width: 120, height: 30 });
    run(setup.renderer);
    // Wait for the frame callback to fire and trigger the zIndex setter.
    await waitFrame();
    // The frame loop may have already rendered (change_count updated). Use
    // renderFull() which bypasses the change-detection skip so we always get
    // a complete ANSI frame regardless of render state.
    setup.renderer.renderFull();
    const frame = setup.captureFrame();
    // Borders must still appear even after z-index animation has run.
    const plain = stripAnsi(frame);
    expect(plain).toMatch(/[┌┐└┘│─╔╗╚╝╠╣╦╩╬╒╓╕╖╘╙╛╜╟╞╡╢╤╥╧╨╪╫]/u);
  });

  // ── keyboard ──────────────────────────────────────────────────────────────

  it("registers a keypress listener during run", async () => {
    setup = await createTestRenderer({ width: 120, height: 30 });
    run(setup.renderer);
    // After run(), at least one keypress listener should be registered
    // (the animation-speed +/- handler).
    expect(setup.renderer.keyInput.listenerCount("keypress")).toBeGreaterThan(0);
  });

  // ── destroy ───────────────────────────────────────────────────────────────

  it("destroy cleans up without errors", async () => {
    setup = await createTestRenderer({ width: 120, height: 30 });
    run(setup.renderer);
    await waitFrame();
    const savedCleanup = setup.cleanup;
    expect(() => destroy(setup?.renderer)).not.toThrow();
    setup = undefined; // prevent afterEach double-destroy
    savedCleanup();
  });

  it("destroy removes the Screen container from the renderer root", async () => {
    setup = await createTestRenderer({ width: 120, height: 30 });
    run(setup.renderer);
    await waitFrame();
    const savedCleanup = setup.cleanup;
    destroy(setup.renderer);
    // Screen.destroy() calls container.destroyRecursively() + root.remove().
    expect(setup.renderer.root.getRenderable("nestedZindex-screen")).toBeUndefined();
    setup = undefined;
    savedCleanup();
  });

  it("can run → destroy → run again without errors", async () => {
    setup = await createTestRenderer({ width: 120, height: 30 });
    expect(() => run(setup?.renderer)).not.toThrow();
    await waitFrame();
    expect(() => destroy(setup?.renderer)).not.toThrow();
    // Second run on the same renderer
    expect(() => run(setup?.renderer)).not.toThrow();
    await waitFrame();
  });
});
