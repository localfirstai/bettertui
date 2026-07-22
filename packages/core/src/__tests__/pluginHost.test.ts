import { describe, expect, it } from "vitest";
import { createPluginHost } from "../platform";

// These tests exercise the real native addon via the plugin-host napi bridge.
describe("NapiPluginHost", () => {
  it("registers plugins and reports duplicates", () => {
    const host = createPluginHost();
    expect(host.register("p", "1.0", "me", ["commands"])).toBeNull();
    // Duplicate registration returns an error string.
    expect(host.register("p", "1.0", "me", [])).toMatch(/already registered/);
    expect(host.pluginNames()).toContain("p");
  });

  it("enforces the lifecycle transition order", () => {
    const host = createPluginHost();
    host.register("p", "1.0", "me", []);
    expect(host.state("p")).toBe("registered");
    // Cannot start before initializing.
    expect(host.start("p")).toMatch(/cannot start/);
    expect(host.initialize("p")).toBeNull();
    expect(host.state("p")).toBe("initialized");
    expect(host.start("p")).toBeNull();
    expect(host.state("p")).toBe("running");
    expect(host.stop("p")).toBeNull();
    expect(host.state("p")).toBe("stopped");
  });

  it("marks a plugin as errored", () => {
    const host = createPluginHost();
    host.register("p", "1.0", "me", []);
    expect(host.markError("p")).toBeNull();
    expect(host.state("p")).toBe("error");
  });

  it("composes append slots by priority", () => {
    const host = createPluginHost();
    host.ensureSlot("statusBar", "append");
    const token = host.slotRegister("statusBar", "p", 0, "low");
    host.slotRegister("statusBar", "p", 10, "high");
    host.slotRegister("statusBar", "p", 5, "mid");
    expect(host.slotResolve("statusBar")).toEqual(["high", "mid", "low"]);
    // Removing the low-priority contribution leaves the rest.
    expect(host.slotRemove("statusBar", token)).toBe(true);
    expect(host.slotResolve("statusBar")).toEqual(["high", "mid"]);
  });

  it("supports single-winner and replace modes", () => {
    const host = createPluginHost();
    host.ensureSlot("header", "single-winner");
    host.slotRegister("header", "a", 1, "a");
    host.slotRegister("header", "b", 9, "b");
    expect(host.slotResolve("header")).toEqual(["b"]);

    host.ensureSlot("footer", "replace");
    host.slotRegister("footer", "a", 0, "old");
    host.slotRegister("footer", "b", 0, "new");
    expect(host.slotResolve("footer")).toEqual(["new"]);
  });

  it("coalesces slot changes via the dirty flag", () => {
    const host = createPluginHost();
    host.ensureSlot("s", "append");
    host.slotRegister("s", "p", 0, "x");
    host.slotRegister("s", "p", 0, "y");
    // Multiple mutations collapse to a single dirty signal.
    expect(host.slotTakeDirty("s")).toBe(true);
    expect(host.slotTakeDirty("s")).toBe(false);
  });

  it("removes a plugin's slot contributions on unregister", () => {
    const host = createPluginHost();
    host.register("p", "1.0", "me", []);
    host.ensureSlot("s", "append");
    host.slotRegister("s", "p", 0, "x");
    host.slotRegister("s", "other", 0, "y");
    expect(host.unregister("p")).toBeNull();
    expect(host.slotResolve("s")).toEqual(["y"]);
  });
});
