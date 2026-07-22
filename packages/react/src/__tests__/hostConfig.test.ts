import { describe, expect, it, vi } from "vitest";
import { makeHostConfig } from "../reconciler/hostConfig";

// ── Minimal CliRenderer mock ──────────────────────────────────────────────────
let _nativeCounter = 10_000;

function makeMockRenderer() {
  const nodes = new Map<number, { parent: number | null; children: number[] }>();
  const rootId = _nativeCounter++;
  nodes.set(rootId, { parent: null, children: [] });

  return {
    rootNodeId: rootId,
    createNode: vi.fn((_kind: string) => {
      const id = _nativeCounter++;
      nodes.set(id, { parent: null, children: [] });
      return id;
    }),
    appendChild: vi.fn((parent: number, child: number) => {
      const p = nodes.get(parent);
      const c = nodes.get(child);
      if (p && c) {
        p.children.push(child);
        c.parent = parent;
      }
      return true;
    }),
    removeNode: vi.fn((id: number) => {
      const node = nodes.get(id);
      if (node?.parent !== null && node?.parent !== undefined) {
        const parent = nodes.get(node.parent);
        if (parent) parent.children = parent.children.filter((c) => c !== id);
      }
      nodes.delete(id);
    }),
    setText: vi.fn(),
    setNodeStyle: vi.fn(),
    setNodeLayout: vi.fn(),
    insertNodeBefore: vi.fn(),
    getChildrenOf: vi.fn((id: number) => nodes.get(id)?.children ?? []),
    render: vi.fn(),
    terminalWidth: 80,
    terminalHeight: 24,
  };
}

describe("makeHostConfig", () => {
  it("createInstance creates a native node and returns BetterTUIInstance", () => {
    const renderer = makeMockRenderer();
    const cfg = makeHostConfig(renderer as never);

    const instance = cfg.createInstance("box", { width: 10 });
    expect(renderer.createNode).toHaveBeenCalledWith("box");
    expect(instance.type).toBe("box");
    expect(instance.nativeId).toBeTypeOf("number");
    expect(instance.children).toEqual([]);
    expect(instance.id).toMatch(/^btui-/);
  });

  it("createInstance calls applyLayout for layout props", () => {
    const renderer = makeMockRenderer();
    const cfg = makeHostConfig(renderer as never);

    cfg.createInstance("box", { width: 10, flexDirection: "row" });
    expect(renderer.setNodeLayout).toHaveBeenCalled();
  });

  it("createInstance calls applyStyle for style props", () => {
    const renderer = makeMockRenderer();
    const cfg = makeHostConfig(renderer as never);

    cfg.createInstance("text", { fg: "red", bold: true });
    expect(renderer.setNodeStyle).toHaveBeenCalled();
  });

  it("createTextInstance creates a text node and sets text", () => {
    const renderer = makeMockRenderer();
    const cfg = makeHostConfig(renderer as never);

    const ti = cfg.createTextInstance("hello");
    expect(renderer.createNode).toHaveBeenCalledWith("text");
    expect(renderer.setText).toHaveBeenCalledWith(ti.nativeId, "hello");
    expect(ti.type).toBe("#text");
    expect(ti.text).toBe("hello");
  });

  it("appendChild adds child to parent's native tree", () => {
    const renderer = makeMockRenderer();
    const cfg = makeHostConfig(renderer as never);

    const parent = cfg.createInstance("box", {});
    const child = cfg.createInstance("text", {});
    cfg.appendChild(parent, child);

    expect(renderer.appendChild).toHaveBeenCalledWith(parent.nativeId, child.nativeId);
    expect(parent.children).toContain(child);
    expect(child.parent).toBe(parent);
  });

  it("appendChildToContainer appends to the root node", () => {
    const renderer = makeMockRenderer();
    const cfg = makeHostConfig(renderer as never);
    const container = { renderer: renderer as never, rootNativeId: renderer.rootNodeId };

    const child = cfg.createInstance("box", {});
    cfg.appendChildToContainer(container, child);

    expect(renderer.appendChild).toHaveBeenCalledWith(renderer.rootNodeId, child.nativeId);
  });

  it("removeChild removes from parent and calls removeNode", () => {
    const renderer = makeMockRenderer();
    const cfg = makeHostConfig(renderer as never);

    const parent = cfg.createInstance("box", {});
    const child = cfg.createInstance("box", {});
    cfg.appendChild(parent, child);
    cfg.removeChild(parent, child);

    expect(renderer.removeNode).toHaveBeenCalledWith(child.nativeId);
    expect(parent.children).not.toContain(child);
    expect(child.parent).toBeNull();
  });

  it("commitTextUpdate updates text content", () => {
    const renderer = makeMockRenderer();
    const cfg = makeHostConfig(renderer as never);

    const ti = cfg.createTextInstance("old");
    cfg.commitTextUpdate(ti, "old", "new");

    expect(ti.text).toBe("new");
    expect(renderer.setText).toHaveBeenLastCalledWith(ti.nativeId, "new");
  });

  it("commitUpdate merges props and calls applyProps", () => {
    const renderer = makeMockRenderer();
    const cfg = makeHostConfig(renderer as never);

    const instance = cfg.createInstance("box", { width: 10 });
    vi.clearAllMocks();
    cfg.commitUpdate(instance, { width: 20, fg: "blue" });

    expect(instance.props.width).toBe(20);
    expect(renderer.setNodeStyle).toHaveBeenCalled();
    expect(renderer.setNodeLayout).toHaveBeenCalled();
  });

  it("resetAfterCommit calls renderer.render()", () => {
    const renderer = makeMockRenderer();
    const cfg = makeHostConfig(renderer as never);

    cfg.resetAfterCommit();
    expect(renderer.render).toHaveBeenCalledOnce();
  });

  it("clearContainer removes all root children", () => {
    const renderer = makeMockRenderer();
    const cfg = makeHostConfig(renderer as never);
    const container = { renderer: renderer as never, rootNativeId: renderer.rootNodeId };

    const a = cfg.createInstance("box", {});
    const b = cfg.createInstance("box", {});
    cfg.appendChildToContainer(container, a);
    cfg.appendChildToContainer(container, b);

    // Simulate getChildrenOf returning the two children
    renderer.getChildrenOf.mockReturnValueOnce([a.nativeId, b.nativeId]);
    cfg.clearContainer(container);

    expect(renderer.removeNode).toHaveBeenCalledWith(a.nativeId);
    expect(renderer.removeNode).toHaveBeenCalledWith(b.nativeId);
  });

  it("insertBefore maintains correct TS children order", () => {
    const renderer = makeMockRenderer();
    const cfg = makeHostConfig(renderer as never);

    const parent = cfg.createInstance("box", {});
    const a = cfg.createInstance("box", {});
    const b = cfg.createInstance("box", {});
    const c = cfg.createInstance("box", {});

    cfg.appendChild(parent, a);
    cfg.appendChild(parent, c);
    cfg.insertBefore(parent, b, c);

    expect(parent.children).toEqual([a, b, c]);
    expect(renderer.insertNodeBefore).toHaveBeenCalledWith(parent.nativeId, b.nativeId, c.nativeId);
  });
});
