import { createTestRenderer } from "@bettertui/core";
import type { TestRendererSetup } from "@bettertui/core";
import type { Text } from "@bettertui/core";
/**
 * E2E tests for the nestedZindex example.
 *
 * NOTE on rendering strategy:
 * The Rust engine renders flow-positioned elements into the captured frame
 * correctly. Absolutely positioned elements (like the Text labels and parent
 * groups in this example) DO appear in the captured frame once the containing
 * block (rootContainer) has explicit width/height. Frame-content assertions for
 * box-drawing characters are checked in the "three parent groups" test.
 * Text node content is verified via node.content for precise string assertions.
 */
import { afterEach, describe, expect, it } from "vitest";
import { destroy, run } from "./nestedZindex.example";

// Strip all ANSI/CSI/OSC escape sequences.
// Build via new RegExp to avoid biome's noControlCharactersInRegex on regex literals.
const ESC = "\u001b";
const ANSI_RE = new RegExp(
  `${ESC}\\[[0-9;?=!><]*[A-Za-z~@^]|${ESC}][^\\u0007${ESC}]*(?:\\u0007|${ESC}\\\\)|${ESC}[^[]`,
  "g",
);
function stripAnsi(str: string): string {
  return str.replace(ANSI_RE, "").replace(new RegExp(ESC, "g"), "");
}

// Await one event-loop tick so the frame loop runs at least once.
// The frame loop executes lifecycle passes (which sync Text nodes to the Rust
// engine) and then calls render() — both steps are required for engine state
// to be up to date.
function waitFrame(): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, 0));
}

describe("nestedZindex example", () => {
  let setup: TestRendererSetup | undefined;

  afterEach(() => {
    if (setup) {
      destroy(setup.renderer);
      setup.cleanup();
      setup = undefined;
    }
  });

  it("mounts without errors", async () => {
    setup = await createTestRenderer({ width: 120, height: 30 });
    const { renderer } = setup;
    expect(() => run(renderer)).not.toThrow();
  });

  it("renders the title text", async () => {
    setup = await createTestRenderer({ width: 120, height: 30 });
    run(setup.renderer);
    await waitFrame();
    // Inspect the title Text node directly.
    const titleNode = setup.renderer.root.getRenderable("main-title") as Text | undefined;
    expect(titleNode).toBeDefined();
    expect(stripAnsi(titleNode?.content ?? "")).toContain("Nested");
  });

  it("renders the three parent groups with borders in the captured frame", async () => {
    setup = await createTestRenderer({ width: 120, height: 30 });
    run(setup.renderer);
    await waitFrame();
    // Parent groups have explicit dimensions — confirm they appear in the tree.
    expect(setup.renderer.root.getRenderable("parent-group-a")).toBeDefined();
    expect(setup.renderer.root.getRenderable("parent-group-b")).toBeDefined();
    expect(setup.renderer.root.getRenderable("parent-group-c")).toBeDefined();
    // The rendered frame must be non-trivial (at a minimum the background fill).
    const frame = setup.captureFrame();
    expect(frame.length).toBeGreaterThan(100);
    // At least one box-drawing character must appear (confirms borders rendered).
    const plain = stripAnsi(frame);
    expect(plain).toMatch(/[┌┐└┘│─╔╗╚╝╠╣╦╩╬╒╓╕╖╘╙╛╜╟╞╡╢╤╥╧╨╪╫]/u);
  });

  it("renders child boxes within groups", async () => {
    setup = await createTestRenderer({ width: 120, height: 30 });
    run(setup.renderer);
    await waitFrame();
    // Text label nodes are in the tree and carry the right content.
    const textA1 = setup.renderer.root.getRenderable("text-a1") as Text | undefined;
    const textB1 = setup.renderer.root.getRenderable("text-b1") as Text | undefined;
    const textC1 = setup.renderer.root.getRenderable("text-c1") as Text | undefined;
    expect(textA1).toBeDefined();
    expect(textB1).toBeDefined();
    expect(textC1).toBeDefined();
    expect(stripAnsi(textA1?.content ?? "")).toContain("Child A1");
    expect(stripAnsi(textB1?.content ?? "")).toContain("Child B1");
    expect(stripAnsi(textC1?.content ?? "")).toContain("Child C1");
  });

  it("renders the animation phase indicator", async () => {
    setup = await createTestRenderer({ width: 120, height: 30 });
    run(setup.renderer);
    await waitFrame();
    const phaseNode = setup.renderer.root.getRenderable("phase-indicator") as Text | undefined;
    expect(phaseNode).toBeDefined();
    expect(stripAnsi(phaseNode?.content ?? "")).toContain("Animation Phase");
  });

  it("destroy cleans up without errors", async () => {
    setup = await createTestRenderer({ width: 120, height: 30 });
    run(setup.renderer);
    await waitFrame();
    const savedCleanup = setup.cleanup;
    expect(() => destroy(setup?.renderer)).not.toThrow();
    // Verify root container was removed from the tree.
    expect(setup.renderer.root.getRenderable("root-container")).toBeUndefined();
    setup = undefined; // prevent afterEach double-destroy
    savedCleanup();
  });
});
