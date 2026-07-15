import { describe, expect, it } from "vitest";
import { KeyInput } from "../src/lib/keyboard";

function collect(input: string): string[] {
  const ki = new KeyInput();
  const keys: string[] = [];
  ki.on((e) => keys.push(e.key));
  const anyKi = ki as unknown as { handleChunk: (chunk: string) => void };
  anyKi.handleChunk(input);
  ki.stop();
  return keys;
}

function collectAsync(input: string): Promise<string[]> {
  const ki = new KeyInput();
  const keys: string[] = [];
  ki.on((e) => keys.push(e.key));
  const anyKi = ki as unknown as { handleChunk: (chunk: string) => void };
  anyKi.handleChunk(input);
  return new Promise((resolve) => {
    setTimeout(() => {
      ki.stop();
      resolve(keys);
    }, 40);
  });
}

describe("KeyInput parser", () => {
  it("maps single printable characters", () => {
    expect(collect("q")).toEqual(["q"]);
  });

  it("maps arrow keys", () => {
    expect(collect("\x1b[A")).toEqual(["ArrowUp"]);
    expect(collect("\x1b[B")).toEqual(["ArrowDown"]);
  });

  it("maps Enter and Tab", () => {
    expect(collect("\r")).toEqual(["Enter"]);
    expect(collect("\t")).toEqual(["Tab"]);
  });

  it("maps Escape", async () => {
    const keys = await collectAsync("\x1b");
    expect(keys).toEqual(["Escape"]);
  });

  it("strips a trailing newline from piped input", () => {
    expect(collect("q\n")).toEqual(["q"]);
  });

  it("carries modifier flags for ctrl sequences", () => {
    const ki = new KeyInput();
    const events: Array<{ key: string; ctrl: boolean }> = [];
    ki.on((e) => {
      events.push({ key: e.key, ctrl: e.ctrl });
    });
    const anyKi = ki as unknown as { handleChunk: (chunk: string) => void };
    anyKi.handleChunk("\x03"); // ctrl+c
    ki.stop();
    expect(events.length).toBe(1);
    expect(events[0]?.ctrl).toBe(true);
    expect(events[0]?.key).toBe("c");
  });
});
