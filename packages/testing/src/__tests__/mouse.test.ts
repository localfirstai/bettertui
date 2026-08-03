import { describe, expect, it } from "vitest";
import { MockMouse } from "../mouse/mouse";
import { TestTerminal } from "../terminal/test-terminal";

describe("MockMouse", () => {
  it("encodes SGR mouse clicks with 1-based indexing", async () => {
    const terminal = new TestTerminal();
    const mouse = new MockMouse(terminal);

    const emitted: string[] = [];
    terminal.on("input", (_, seq) => emitted.push(seq as string));

    await mouse.click(5, 10);
    // x=5 (1-based: 6), y=10 (1-based: 11)
    expect(emitted).toEqual(["\x1b[<0;6;11M", "\x1b[<0;6;11m"]);
  });

  it("encodes mouse scroll sequences", () => {
    const terminal = new TestTerminal();
    const mouse = new MockMouse(terminal);

    const emitted: string[] = [];
    terminal.on("input", (_, seq) => emitted.push(seq as string));

    mouse.scroll(0, 0, "down", 1);
    expect(emitted).toEqual(["\x1b[<65;1;1M"]);
  });
});
