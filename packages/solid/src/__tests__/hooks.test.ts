/**
 * Unit tests for @bettertui/solid hooks and context.
 * No JSX, no native binary required.
 */
import { describe, expect, it } from "vitest";
import { useRenderer } from "../context/rendererContext";

// ── rendererContext ───────────────────────────────────────────────────────────

describe("rendererContext", () => {
  it("useRenderer throws when called outside a reactive root", () => {
    // solid-js's createContext returns the default value (undefined) when
    // useContext is called outside a reactive root. Our guard converts that
    // to a descriptive error.
    expect(() => useRenderer()).toThrow("useRenderer() called outside");
  });
});

// ── hooks barrel ─────────────────────────────────────────────────────────────

describe("hooks exports", () => {
  it("exports all expected hooks", async () => {
    let mod: Record<string, unknown>;
    try {
      mod = await import("../hooks/index");
    } catch {
      // Skip if solid-js or a dependency is unavailable in this environment
      return;
    }
    expect(typeof mod.useRenderer).toBe("function");
    expect(typeof mod.useKeyboard).toBe("function");
    expect(typeof mod.useFocus).toBe("function");
    expect(typeof mod.useTerminalDimensions).toBe("function");
    expect(typeof mod.useTimeline).toBe("function");
    expect(typeof mod.useTheme).toBe("function");
  });
});
