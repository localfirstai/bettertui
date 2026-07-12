import { describe, expect, it } from "vitest";
import {
  MockCommandCollector,
  createMockHandler,
  createPoint,
  createRect,
  createTestTree,
  expectCommandBuffer,
} from "../index";

describe("MockCommandCollector", () => {
  it("starts with empty commands", () => {
    const collector = new MockCommandCollector();
    expect(collector.getCommands()).toHaveLength(0);
  });

  it("collects commands from buffer", () => {
    const collector = new MockCommandCollector();
    collector.commandBuffer.push({ type: "Shutdown" });
    const commands = collector.getCommands();
    expect(commands).toHaveLength(1);
    expect(commands[0].type).toBe("Shutdown");
  });

  it("gets last command", () => {
    const collector = new MockCommandCollector();
    collector.commandBuffer.push({ type: "Shutdown" });
    collector.commandBuffer.push({ type: "CreateNode", id: "1", kind: "Box" });
    expect(collector.getLastCommand()?.type).toBe("CreateNode");
  });

  it("gets commands by type", () => {
    const collector = new MockCommandCollector();
    collector.commandBuffer.push({ type: "CreateNode", id: "1", kind: "Box" });
    collector.commandBuffer.push({ type: "CreateNode", id: "2", kind: "Text" });
    collector.commandBuffer.push({ type: "Shutdown" });
    const createNodes = collector.getCommandsByType("CreateNode");
    expect(createNodes).toHaveLength(2);
  });

  it("clears commands", () => {
    const collector = new MockCommandCollector();
    collector.commandBuffer.push({ type: "Shutdown" });
    collector.clear();
    expect(collector.getCommands()).toHaveLength(0);
  });
});

describe("createPoint", () => {
  it("creates a point", () => {
    const point = createPoint(10, 20);
    expect(point).toEqual({ x: 10, y: 20 });
  });
});

describe("createRect", () => {
  it("creates a rect", () => {
    const rect = createRect(10, 20, 100, 50);
    expect(rect).toEqual({ x: 10, y: 20, width: 100, height: 50 });
  });
});

describe("createMockHandler", () => {
  it("creates a handler that records calls", () => {
    const handler = createMockHandler<(x: number) => void>();
    handler(1);
    handler(2);
    handler(3);
    expect(handler.calls).toHaveLength(3);
    expect(handler.calls[0]).toEqual([1]);
    expect(handler.calls[1]).toEqual([2]);
    expect(handler.calls[2]).toEqual([3]);
  });

  it("can be cleared", () => {
    const handler = createMockHandler<(x: number) => void>();
    handler(1);
    handler.clear();
    expect(handler.calls).toHaveLength(0);
  });
});

describe("expectCommandBuffer", () => {
  it("checks length", () => {
    const { CommandBuffer } = require("@bettertui/core");
    const buffer = new CommandBuffer();
    buffer.push({ type: "Shutdown" });
    expect(() => expectCommandBuffer(buffer, { length: 1 })).not.toThrow();
    expect(() => expectCommandBuffer(buffer, { length: 2 })).toThrow();
  });

  it("checks isEmpty", () => {
    const { CommandBuffer } = require("@bettertui/core");
    const buffer = new CommandBuffer();
    expect(() => expectCommandBuffer(buffer, { isEmpty: true })).not.toThrow();
    buffer.push({ type: "Shutdown" });
    expect(() => expectCommandBuffer(buffer, { isEmpty: false })).not.toThrow();
  });

  it("checks types", () => {
    const { CommandBuffer } = require("@bettertui/core");
    const buffer = new CommandBuffer();
    buffer.push({ type: "CreateNode", id: "1", kind: "Box" });
    buffer.push({ type: "Shutdown" });
    expect(() => expectCommandBuffer(buffer, { types: ["CreateNode", "Shutdown"] })).not.toThrow();
  });
});

describe("createTestTree", () => {
  it("creates a simple tree", () => {
    const tree = createTestTree();
    expect(tree.root.type).toBe("Box");
    expect(tree.root.children).toHaveLength(1);
    expect(tree.root.children[0].type).toBe("Text");
  });
});
