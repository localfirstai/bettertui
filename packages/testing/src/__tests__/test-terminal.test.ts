import { describe, expect, it } from "vitest";
import { TestTerminal } from "../terminal/test-terminal";

describe("TestTerminal & CellMatrix", () => {
  it("initializes with default dimensions and empty cells", () => {
    const terminal = new TestTerminal({ width: 20, height: 5 });
    expect(terminal.width).toBe(20);
    expect(terminal.height).toBe(5);

    const cell = terminal.getCell(0, 0);
    expect(cell).toBeDefined();
    expect(cell?.char).toBe(" ");
  });

  it("writes strings and captures text frame output", () => {
    const terminal = new TestTerminal({ width: 10, height: 2 });
    terminal.matrix.writeString(0, 0, "Hello");
    terminal.matrix.writeString(0, 1, "World");

    const frame = terminal.captureFrame();
    expect(frame.textFrame).toBe("Hello     \nWorld     ");
  });

  it("resizes matrix buffer cleanly", () => {
    const terminal = new TestTerminal({ width: 10, height: 2 });
    terminal.matrix.writeString(0, 0, "Hello");
    terminal.resize(5, 1);

    expect(terminal.width).toBe(5);
    expect(terminal.height).toBe(1);
    expect(terminal.captureFrame().textFrame).toBe("Hello");
  });
});
