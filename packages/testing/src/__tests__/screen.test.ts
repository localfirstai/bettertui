import { describe, expect, it } from "vitest";
import { screen } from "../screen/screen";
import { TestTerminal } from "../terminal/test-terminal";

describe("ScreenQueryEngine", () => {
  it("queries elements by text from matrix frame", () => {
    const terminal = new TestTerminal({ width: 30, height: 5 });
    screen.setTerminal(terminal);

    terminal.matrix.writeString(2, 1, "Submit Button");

    const target = screen.getByText("Submit Button");
    expect(target).toBeDefined();
    expect(target.x).toBe(2);
    expect(target.y).toBe(1);
    expect(target.text).toBe("Submit Button");
  });

  it("returns null on queryByText mismatch", () => {
    const terminal = new TestTerminal();
    screen.setTerminal(terminal);

    expect(screen.queryByText("NonExistent")).toBeNull();
  });
});
