import { describe, expect, it, vi } from "vitest";
import { createBetterTUIReconciler, createContainer } from "../renderer";

describe("createBetterTUIReconciler", () => {
  it("creates a reconciler with required methods", () => {
    const reconciler = createBetterTUIReconciler({ push: vi.fn() });
    expect(reconciler).toBeDefined();
    expect(typeof reconciler.createContainer).toBe("function");
    expect(typeof reconciler.updateContainer).toBe("function");
  });
});

describe("createContainer", () => {
  it("creates a container with default id", () => {
    const buffer = { push: vi.fn() };
    const reconciler = createBetterTUIReconciler(buffer);
    const c = createContainer(reconciler, buffer);
    expect(c).toBeDefined();
  });

  it("creates a container with custom id and onCommit", () => {
    const onCommit = vi.fn();
    const buffer = { push: vi.fn() };
    const reconciler = createBetterTUIReconciler(buffer);
    const c = createContainer(reconciler, buffer, { id: "root", onCommit });
    expect(c).toBeDefined();
  });
});
