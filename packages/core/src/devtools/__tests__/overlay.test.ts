import { describe, expect, it } from "vitest";
import { createDevTools } from "../index";
import { stripAnsi } from "../overlay/ansiUtils";
import { OverlayHost, type OverlayRenderer } from "../overlay/overlayHost";
import { DebugPanel } from "../overlay/panel.types";

function fakeRenderer(): OverlayRenderer & { buffer: string; writes: number } {
  return {
    terminalWidth: 80,
    viewportHeight: 24,
    buffer: "",
    writes: 0,
    getDiagnostics() {
      return {
        renderCalls: 42,
        renderBytes: 4096,
        eventDispatches: 7,
        layoutComputations: 3,
        cacheHits: 8,
        cacheMisses: 2,
        allocations: 12,
        averageFrameTime: 4.2,
        fps: 60,
      };
    },
    write(text: string) {
      this.buffer += text;
      this.writes += 1;
    },
  };
}

describe("OverlayHost", () => {
  it("is not visible when no panels are shown", () => {
    const dt = createDevTools({ enabled: true });
    const host = new OverlayHost(fakeRenderer(), dt);
    expect(host.visible).toBe(false);
  });

  it("becomes visible once a panel is shown", () => {
    const dt = createDevTools({ enabled: true });
    dt.show(DebugPanel.Performance);
    const host = new OverlayHost(fakeRenderer(), dt);
    expect(host.visible).toBe(true);
  });

  it("paints panel content over the frame", () => {
    const dt = createDevTools({ enabled: true });
    dt.show(DebugPanel.Performance);
    dt.recordFrame({ duration: 4, dirtyRegionCount: 2 });
    const r = fakeRenderer();
    const host = new OverlayHost(r, dt);
    host.setDirtyRegionCount(2);
    host.paint();
    const plain = stripAnsi(r.buffer);
    expect(plain).toContain("Performance");
    expect(plain).toContain("Renders");
  });

  it("does not write when nothing is visible and nothing was painted", () => {
    const dt = createDevTools({ enabled: true });
    const r = fakeRenderer();
    const host = new OverlayHost(r, dt);
    host.paint();
    expect(r.writes).toBe(0);
  });

  it("clears vacated rows when a panel is hidden between paints", () => {
    const dt = createDevTools({ enabled: true });
    dt.show(DebugPanel.Performance);
    const r = fakeRenderer();
    const host = new OverlayHost(r, dt);
    host.paint();
    const firstLen = r.buffer.length;
    expect(firstLen).toBeGreaterThan(0);

    dt.hide(DebugPanel.Performance);
    host.paint();
    // The second paint must emit clearing writes for the previously painted rows.
    expect(r.buffer.length).toBeGreaterThan(firstLen);
    expect(host.visible).toBe(false);
  });

  it("clear() wipes the painted region", () => {
    const dt = createDevTools({ enabled: true });
    dt.show(DebugPanel.Events);
    const r = fakeRenderer();
    const host = new OverlayHost(r, dt);
    host.paint();
    r.buffer = "";
    host.clear();
    expect(r.buffer.length).toBeGreaterThan(0); // emitted clearing sequence
    // A subsequent clear is a no-op.
    r.buffer = "";
    host.clear();
    expect(r.buffer).toBe("");
  });

  it("respects the configured corner", () => {
    const dt = createDevTools({ enabled: true });
    dt.show(DebugPanel.Performance);
    const r = fakeRenderer();
    const host = new OverlayHost(r, dt, { corner: "top-left" });
    host.paint();
    // top-left anchors at column 1.
    expect(r.buffer).toContain("\x1b[1;1H");
  });
});

describe("panels", () => {
  it("tree panel renders the recorded tree with highlight marker", () => {
    const dt = createDevTools({ enabled: true });
    dt.tree.buildTree([
      { id: "root", type: "Box" },
      { id: "child", type: "Text", parent: "root" },
    ]);
    dt.highlight("child");
    dt.show(DebugPanel.Tree);
    const r = fakeRenderer();
    const host = new OverlayHost(r, dt);
    host.paint();
    const plain = stripAnsi(r.buffer);
    expect(plain).toContain("Box#root");
    expect(plain).toContain("▸ Text#child");
  });

  it("layout panel reports the highlighted node's box", () => {
    const dt = createDevTools({ enabled: true });
    dt.tree.buildTree([{ id: "root", type: "Box", layout: { x: 1, y: 2, width: 10, height: 4 } }]);
    dt.highlight("root");
    dt.show(DebugPanel.Layout);
    const r = fakeRenderer();
    const host = new OverlayHost(r, dt);
    host.paint();
    const plain = stripAnsi(r.buffer);
    expect(plain).toContain("10×4");
  });

  it("events panel lists recent events", () => {
    const dt = createDevTools({ enabled: true });
    dt.recordKeyboard("a", { ctrl: false, shift: false, alt: false, meta: false });
    dt.show(DebugPanel.Events);
    const r = fakeRenderer();
    const host = new OverlayHost(r, dt);
    host.paint();
    const plain = stripAnsi(r.buffer);
    expect(plain).toContain("key a");
  });

  it("dirty-regions panel shows the current count", () => {
    const dt = createDevTools({ enabled: true });
    dt.show(DebugPanel.DirtyRegions);
    const r = fakeRenderer();
    const host = new OverlayHost(r, dt);
    host.setDirtyRegionCount(5);
    host.paint();
    const plain = stripAnsi(r.buffer);
    expect(plain).toContain("Current");
    expect(plain).toContain("5");
  });
});
