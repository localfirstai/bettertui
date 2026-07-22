/**
 * Unit tests for the Solid universal renderer host config.
 * All tests use a mock CliRenderer — no native binary required.
 */
import { describe, expect, it, vi } from "vitest";
import { makeUniversalRenderer } from "../renderer/hostConfig";
import { NO_NODE, createTreeState } from "../renderer/treeState";

// ── TreeState tests ───────────────────────────────────────────────────────────

describe("createTreeState", () => {
  it("insertChild appends when anchor is NO_NODE", () => {
    const tree = createTreeState();
    tree.insertChild(1, 10, NO_NODE);
    tree.insertChild(1, 20, NO_NODE);
    expect(tree.getFirstChild(1)).toBe(10);
    expect(tree.getNextSibling(10)).toBe(20);
    expect(tree.getNextSibling(20)).toBe(NO_NODE);
  });

  it("insertChild inserts before anchor", () => {
    const tree = createTreeState();
    tree.insertChild(1, 10, NO_NODE);
    tree.insertChild(1, 30, NO_NODE);
    tree.insertChild(1, 20, 30);
    // order should be [10, 20, 30]
    expect(tree.getFirstChild(1)).toBe(10);
    expect(tree.getNextSibling(10)).toBe(20);
    expect(tree.getNextSibling(20)).toBe(30);
    expect(tree.getNextSibling(30)).toBe(NO_NODE);
  });

  it("removeNode clears parent and child tracking", () => {
    const tree = createTreeState();
    tree.insertChild(1, 10, NO_NODE);
    tree.insertChild(1, 20, NO_NODE);
    tree.removeNode(10);
    expect(tree.getParent(10)).toBe(NO_NODE);
    expect(tree.getFirstChild(1)).toBe(20);
  });

  it("insertChild moves node to new parent if already parented", () => {
    const tree = createTreeState();
    tree.insertChild(1, 10, NO_NODE);
    tree.insertChild(2, 10, NO_NODE);
    expect(tree.getParent(10)).toBe(2);
    expect(tree.getFirstChild(1)).toBe(NO_NODE);
    expect(tree.getFirstChild(2)).toBe(10);
  });

  it("markTextNode / isTextNode", () => {
    const tree = createTreeState();
    expect(tree.isTextNode(42)).toBe(false);
    tree.markTextNode(42);
    expect(tree.isTextNode(42)).toBe(true);
    tree.removeNode(42);
    expect(tree.isTextNode(42)).toBe(false);
  });

  it("getParent returns NO_NODE for unknown node", () => {
    const tree = createTreeState();
    expect(tree.getParent(999)).toBe(NO_NODE);
  });

  it("getFirstChild returns NO_NODE for leaf node", () => {
    const tree = createTreeState();
    tree.insertChild(1, 10, NO_NODE);
    expect(tree.getFirstChild(10)).toBe(NO_NODE);
  });
});

// ── makeUniversalRenderer smoke tests ─────────────────────────────────────────

function makeMockRenderer() {
  let _nextId = 100;
  const textContent = new Map<number, string>();

  return {
    rootNodeId: 0,
    createNode: vi.fn((_type: string) => _nextId++),
    appendChild: vi.fn((_parent: number, _child: number) => true),
    removeNode: vi.fn((_id: number) => {}),
    setText: vi.fn((id: number, text: string) => {
      textContent.set(id, text);
    }),
    insertNodeBefore: vi.fn((_parent: number, _child: number, _before: number) => {}),
    setNodeStyle: vi.fn(),
    setNodeLayout: vi.fn(),
    render: vi.fn(),
    getChildrenOf: vi.fn(() => [] as number[]),
    keyInput: { on: vi.fn(), off: vi.fn() },
    terminalWidth: 80,
    terminalHeight: 24,
    getText: (id: number) => textContent.get(id) ?? "",
  };
}

describe("makeUniversalRenderer (mock engine)", () => {
  it("createElement calls renderer.createNode with the given type", () => {
    const mock = makeMockRenderer();
    const { createElement } = makeUniversalRenderer(mock as never);
    const id = createElement("box");
    expect(mock.createNode).toHaveBeenCalledWith("box");
    expect(typeof id).toBe("number");
  });

  it("createTextNode creates a text node and sets text", () => {
    const mock = makeMockRenderer();
    const { createTextNode } = makeUniversalRenderer(mock as never);
    const id = createTextNode("hello");
    expect(mock.createNode).toHaveBeenCalledWith("text");
    expect(mock.setText).toHaveBeenCalledWith(id, "hello");
  });

  it("insertNode with no anchor calls appendChild", () => {
    const mock = makeMockRenderer();
    const { createElement, insertNode } = makeUniversalRenderer(mock as never);
    const parent = createElement("box");
    const child = createElement("text");
    insertNode(parent, child, undefined);
    expect(mock.appendChild).toHaveBeenCalledWith(parent, child);
  });

  it("insertNode with anchor calls insertNodeBefore", () => {
    const mock = makeMockRenderer();
    const { createElement, insertNode } = makeUniversalRenderer(mock as never);
    const parent = createElement("box");
    const child1 = createElement("text");
    const child2 = createElement("text");
    insertNode(parent, child1, undefined);
    insertNode(parent, child2, child1);
    expect(mock.insertNodeBefore).toHaveBeenCalledWith(parent, child2, child1);
  });

  it("treeState tracks removal correctly", () => {
    // removeNode is an RendererOptions input, not a createRenderer output.
    // Verify it via the tree state directly (already tested in createTreeState suite).
    const tree = createTreeState();
    tree.insertChild(1, 10, NO_NODE);
    tree.removeNode(10);
    expect(tree.getParent(10)).toBe(NO_NODE);
    expect(tree.getFirstChild(1)).toBe(NO_NODE);
  });
});
