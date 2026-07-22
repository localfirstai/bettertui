import { CommandBuffer, generateId } from "@bettertui/core";
import { bench, describe } from "vitest";

describe("CommandBuffer", () => {
  bench("push single command", () => {
    const buffer = new CommandBuffer();
    buffer.push({ type: "Shutdown" });
  });

  bench("push 100 commands", () => {
    const buffer = new CommandBuffer();
    for (let i = 0; i < 100; i++) {
      buffer.push({ type: "CreateNode", id: generateId(), kind: "Box" });
    }
  });

  bench("push 1000 commands", () => {
    const buffer = new CommandBuffer();
    for (let i = 0; i < 1000; i++) {
      buffer.push({ type: "CreateNode", id: generateId(), kind: "Box" });
    }
  });

  bench("drain empty buffer", () => {
    const buffer = new CommandBuffer();
    buffer.drain();
  });

  bench("drain 100 commands", () => {
    const buffer = new CommandBuffer();
    for (let i = 0; i < 100; i++) {
      buffer.push({ type: "CreateNode", id: generateId(), kind: "Box" });
    }
    buffer.drain();
  });

  bench("peek 100 commands", () => {
    const buffer = new CommandBuffer();
    for (let i = 0; i < 100; i++) {
      buffer.push({ type: "CreateNode", id: generateId(), kind: "Box" });
    }
    buffer.peek();
  });

  bench("clear 100 commands", () => {
    const buffer = new CommandBuffer();
    for (let i = 0; i < 100; i++) {
      buffer.push({ type: "CreateNode", id: generateId(), kind: "Box" });
    }
    buffer.clear();
  });

  bench("mixed command types", () => {
    const buffer = new CommandBuffer();
    const id = generateId();
    buffer.push({ type: "CreateNode", id, kind: "Box" });
    buffer.push({ type: "AppendChild", parent: id, child: generateId() });
    buffer.push({ type: "SetStyle", id, style: { bold: true } });
    buffer.push({ type: "SetText", id: generateId(), text: "hello" });
    buffer.push({ type: "RemoveNode", id: generateId() });
  });
});

describe("generateId", () => {
  bench("generate 1 id", () => {
    generateId();
  });

  bench("generate 100 ids", () => {
    for (let i = 0; i < 100; i++) {
      generateId();
    }
  });

  bench("generate 1000 ids", () => {
    for (let i = 0; i < 1000; i++) {
      generateId();
    }
  });
});
