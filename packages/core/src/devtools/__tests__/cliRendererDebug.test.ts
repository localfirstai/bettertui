import { afterEach, describe, expect, it, vi } from "vitest";
import { CliRenderer } from "../../platform/cliRenderer";
import { DebugPanel } from "../overlay/panel.types";

afterEach(() => {
  vi.restoreAllMocks();
});

/** Silence stdout so tests don't emit ANSI to the reporter. */
function muteStdout() {
  return vi.spyOn(process.stdout, "write").mockImplementation(() => true);
}

describe("CliRenderer debug wiring", () => {
  it("uses a no-op DevTools by default", () => {
    const r = new CliRenderer({ width: 40, height: 12 });
    expect(r.devtools.enabled).toBe(false);
    expect(r.debugEnabled).toBe(false);
    r.destroy();
  });

  it("enables DevTools + overlay with debug: true", () => {
    const r = new CliRenderer({ width: 40, height: 12, debug: true });
    expect(r.devtools.enabled).toBe(true);
    expect(r.debugEnabled).toBe(true);
    // debug: true starts with the performance panel visible.
    expect(r.devtools.isVisible(DebugPanel.Performance)).toBe(true);
    r.destroy();
  });

  it("accepts a DevToolsOptions object", () => {
    const r = new CliRenderer({ width: 40, height: 12, debug: { enabled: true, maxEvents: 10 } });
    expect(r.devtools.enabled).toBe(true);
    // A configured (non-bare-true) debug option does not auto-show a panel.
    expect(r.devtools.visiblePanels.size).toBe(0);
    r.destroy();
  });

  it("records frames into the performance tracker when enabled", () => {
    muteStdout();
    const r = new CliRenderer({ width: 40, height: 12, debug: true });
    r.render();
    r.render();
    expect(r.devtools.performance.count).toBeGreaterThanOrEqual(2);
    r.destroy();
  });

  it("does not record frames when debug is disabled", () => {
    muteStdout();
    const r = new CliRenderer({ width: 40, height: 12 });
    r.render();
    expect(r.devtools.performance.count).toBe(0);
    r.destroy();
  });

  it("toggleDebugOverlay flips panel visibility", () => {
    muteStdout();
    const r = new CliRenderer({ width: 40, height: 12, debug: { enabled: true } });
    expect(r.devtools.isVisible(DebugPanel.Events)).toBe(false);
    r.toggleDebugOverlay(DebugPanel.Events);
    expect(r.devtools.isVisible(DebugPanel.Events)).toBe(true);
    r.toggleDebugOverlay(DebugPanel.Events);
    expect(r.devtools.isVisible(DebugPanel.Events)).toBe(false);
    r.destroy();
  });

  it("toggleDebugOverlay is a no-op when debug is disabled", () => {
    const r = new CliRenderer({ width: 40, height: 12 });
    expect(() => r.toggleDebugOverlay()).not.toThrow();
    expect(r.devtools.visiblePanels.size).toBe(0);
    r.destroy();
  });

  it("honors the BTUI_DEBUG env var", () => {
    const prev = process.env.BTUI_DEBUG;
    process.env.BTUI_DEBUG = "1";
    try {
      const r = new CliRenderer({ width: 40, height: 12 });
      expect(r.devtools.enabled).toBe(true);
      expect(r.debugEnabled).toBe(true);
      expect(r.devtools.isVisible(DebugPanel.Performance)).toBe(true);
      r.destroy();
    } finally {
      process.env.BTUI_DEBUG = prev ?? "";
    }
  });

  it("paints overlay content during render", () => {
    const spy = muteStdout();
    const r = new CliRenderer({ width: 60, height: 16, debug: true });
    r.render();
    const written = spy.mock.calls.map((c) => String(c[0])).join("");
    // Overlay writes a DEC save-cursor before drawing panels.
    expect(written).toContain("\x1b7");
    r.destroy();
  });
});
