import { describe, expect, it, vi } from "vitest";
import { StdinParser } from "../lib/stdinParser";
import { CliRenderer } from "../platform/cliRenderer";
import { ScrollBox } from "../renderables/ScrollBox";
import { Select } from "../renderables/Select";
import { createMockKeys } from "../testing/mockKeys";
import { createTestStdin, createTestStdout } from "../testing/testStreams";

function muteStdout() {
  return vi.spyOn(process.stdout, "write").mockImplementation(() => true);
}

function setup(width = 80, height = 24) {
  const spy = muteStdout();
  const stdin = createTestStdin();
  const stdout = createTestStdout(width, height);
  const origStdin = process.stdin;
  const origStdout = process.stdout;
  Object.defineProperty(process, "stdin", { value: stdin, writable: true, configurable: true });
  Object.defineProperty(process, "stdout", { value: stdout, writable: true, configurable: true });

  const renderer = new CliRenderer({ width, height, autoStart: false });
  renderer.start();
  const mockInput = createMockKeys(renderer);

  const cleanup = () => {
    renderer.stop();
    renderer.destroy();
    spy.mockRestore();
    Object.defineProperty(process, "stdin", {
      value: origStdin,
      writable: true,
      configurable: true,
    });
    Object.defineProperty(process, "stdout", {
      value: origStdout,
      writable: true,
      configurable: true,
    });
  };

  return { renderer, mockInput, cleanup, stdin };
}

describe("StdinParser key parsing", () => {
  it("emits 'key' for arrow down (CSI B)", () => {
    const parser = new StdinParser({});
    const events: Array<{ type: string; name?: string }> = [];
    parser.push(new Uint8Array([0x1b, 0x5b, 0x42])); // ESC [ B
    parser.drain((e) =>
      events.push({ type: e.type, name: e.type === "key" ? e.key.name : undefined }),
    );
    expect(events.some((e) => e.type === "key" && e.name === "down")).toBe(true);
  });

  it("emits 'key' for all arrow keys", () => {
    const arrows = [
      { bytes: [0x1b, 0x5b, 0x41], name: "up" },
      { bytes: [0x1b, 0x5b, 0x42], name: "down" },
      { bytes: [0x1b, 0x5b, 0x43], name: "right" },
      { bytes: [0x1b, 0x5b, 0x44], name: "left" },
    ];
    for (const { bytes, name } of arrows) {
      const parser = new StdinParser({});
      const events: Array<{ type: string; name?: string }> = [];
      parser.push(new Uint8Array(bytes));
      parser.drain((e) =>
        events.push({ type: e.type, name: e.type === "key" ? e.key.name : undefined }),
      );
      expect(events.some((e) => e.type === "key" && e.name === name)).toBe(true);
    }
  });
});

describe("keyboard navigation", () => {
  it("Select: down arrow increments selectedIndex", () => {
    const { renderer, mockInput, cleanup } = setup();

    const select = new Select(renderer, {
      options: [
        { name: "A", description: "" },
        { name: "B", description: "" },
        { name: "C", description: "" },
      ],
      selectedIndex: 0,
    });
    renderer.appendChild(renderer.rootNodeId, select.nodeId);
    select.focus();

    expect(select.selectedIndex).toBe(0);
    mockInput.pressArrow("down");
    expect(select.selectedIndex).toBe(1);
    mockInput.pressArrow("down");
    expect(select.selectedIndex).toBe(2);
    mockInput.pressArrow("up");
    expect(select.selectedIndex).toBe(1);

    cleanup();
  });

  it("ScrollBox: down/up arrows change scrollTop", () => {
    const { renderer, mockInput, cleanup } = setup();

    const scrollBox = new ScrollBox(renderer, {});
    renderer.appendChild(renderer.rootNodeId, scrollBox.nodeId);
    scrollBox.focus();

    expect(scrollBox.scrollTop).toBe(0);
    mockInput.pressArrow("down");
    expect(scrollBox.scrollTop).toBe(1);
    mockInput.pressArrow("up");
    expect(scrollBox.scrollTop).toBe(0);

    cleanup();
  });

  it("Tab navigation: focus switches between widgets", () => {
    const { renderer, stdin, cleanup } = setup();

    const select1 = new Select(renderer, {
      options: [
        { name: "A", description: "" },
        { name: "B", description: "" },
      ],
      selectedIndex: 0,
    });
    const select2 = new Select(renderer, {
      options: [
        { name: "X", description: "" },
        { name: "Y", description: "" },
      ],
      selectedIndex: 0,
    });
    renderer.appendChild(renderer.rootNodeId, select1.nodeId);
    renderer.appendChild(renderer.rootNodeId, select2.nodeId);

    // Manually manage focus
    select1.focus();
    expect(select1.selectedIndex).toBe(0);

    // Down arrow on select1
    stdin.emit("data", Buffer.from("\x1b[B"));
    expect(select1.selectedIndex).toBe(1);

    // Switch focus: blur select1, focus select2
    select1.blur();
    select2.focus();

    // Down arrow now goes to select2
    stdin.emit("data", Buffer.from("\x1b[B"));
    expect(select2.selectedIndex).toBe(1);
    // select1 should NOT change
    expect(select1.selectedIndex).toBe(1);

    cleanup();
  });
});
