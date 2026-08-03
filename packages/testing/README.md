# `@bettertui/testing`

A specification-first testing framework and headless driver suite for BetterTUI applications.

`@bettertui/testing` provides everything needed to unit test, integration test, and end-to-end (E2E) test terminal user interfaces (TUIs)—including React 19 component rendering, headless 2D cell grid inspection, user event simulation, custom Vitest matchers, and real pseudo-terminal (`node-pty`) process testing.

---

## Features

- 🖥️ **Headless Terminal Engine (`TestTerminal`)**: High-performance in-memory 2D cell grid for instant terminal frame rendering and attribute assertions without spawning real processes.
- ⚛️ **React 19 Render Harness (`render`)**: Full React 19 reconciler integration with automatic `act()` state wrapping and DOM-like cleanup.
- 🔍 **Screen Query Engine (`screen`)**: Testing Library style queries (`getByText`, `findByText`, `getByRole`, `debug`) tailored for 2D terminal character matrixes.
- 🖱️ **User Event Simulator (`userEvent`)**: Real-world user input simulation (`user.click`, `user.type`, `user.keyboard`, `user.scroll`, `user.paste`).
- ⌨️ **VT100 & SGR Encoders (`MockKeyboard`, `MockMouse`)**: Precise ANSI escape sequence generators for key chords, modifiers, and mouse clicks/drags.
- 🚀 **Real Process E2E Runner (`PtyTestSession`)**: True process spawning inside a pseudo-terminal (`node-pty`) to test raw mode, stdout, and process signals (`SIGWINCH`, `SIGINT`).
- 🎯 **Golden Matchers (`toMatchGoldenFrame`, `toBeFocused`, `toHaveCell`)**: Custom Vitest matchers for visual snapshot regression testing and cell assertions.
- 📐 **Behavioral Spec Suite (`describeBehaviour`, `BetterTUIDriver`)**: Contract-driven test suite runner for UI primitives and interactive widgets.
- ⚡ **Performance Benchmarking (`runBenchmark`)**: Latency, render throughput, FPS, and memory allocation profiling.

---

## Table of Contents

1. [Installation](#installation)
2. [Quick Start: Component Unit Testing](#quick-start-component-unit-testing)
3. [Testing Interactive Components](#testing-interactive-components)
4. [Headless Terminal Driver (`TestTerminal`)](#headless-terminal-driver-testterminal)
5. [User Event Simulation (`userEvent`)](#user-event-simulation-userevent)
6. [Real End-to-End Testing (`PtyTestSession`)](#real-end-to-end-testing-ptytestsession)
7. [Custom Vitest Matchers](#custom-vitest-matchers)
8. [Behavioral Specification Suite](#behavioral-specification-suite)
9. [Performance Benchmarking](#performance-benchmarking)
10. [API Reference](#api-reference)

---

## Installation

Add `@bettertui/testing` as a dev dependency in your package:

```bash
pnpm add -D @bettertui/testing
```

If you are using Vitest, extend Vitest matchers in your setup file (e.g., `vitest.setup.ts`):

```ts
import { expect } from "vitest";
import { customMatchers } from "@bettertui/testing";

expect.extend(customMatchers);
```

---

## Quick Start: Component Unit Testing

Test React TUI components cleanly using `render`, `screen`, and `userEvent`:

```tsx
import { render, screen, userEvent } from "@bettertui/testing";
import { useState } from "react";
import { expect, test } from "vitest";

function Counter() {
  const [count, setCount] = useState(0);
  return (
    <box borderStyle="single">
      <text>Count: {count}</text>
      <button onClick={() => setCount((c) => c + 1)}>Increment</button>
    </box>
  );
}

test("increments count when button is clicked", async () => {
  render(<Counter />);

  // Assert initial render text
  expect(screen.getByText("Count: 0")).toBeDefined();

  // Find button and simulate click
  const button = screen.getByText("Increment");
  await userEvent.click(button);

  // Assert state updated in terminal frame
  expect(screen.getByText("Count: 1")).toBeDefined();
});
```

---

## Testing Interactive Components

### Testing Keyboard Inputs & Typing
Simulate text input, backspace, and navigation keys:

```tsx
function TextInputDemo() {
  const [value, setValue] = useState("");
  return (
    <box>
      <input value={value} onChange={setValue} placeholder="Type here..." />
      <text>Output: {value}</text>
    </box>
  );
}

test("handles text input typing", async () => {
  render(<TextInputDemo />);

  const input = screen.getByRole("input");
  await userEvent.click(input);
  await userEvent.type("Hello BetterTUI");

  expect(screen.getByText("Output: Hello BetterTUI")).toBeDefined();
});
```

---

## Headless Terminal Driver (`TestTerminal`)

For low-level testing without React, use `TestTerminal` to inspect raw ANSI streams, character frames, and cell attributes:

```ts
import { TestTerminal } from "@bettertui/testing";

const terminal = new TestTerminal(80, 24);

// Feed ANSI sequences or raw output
terminal.write("\x1b[31mHello World!\x1b[0m");

// Render plain text string representation of the frame
console.log(terminal.renderText());

// Query individual 2D grid cells
const cell = terminal.getCell(0, 0);
expect(cell.char).toBe("H");
expect(cell.fg).toBe("\x1b[31m");
```

---

## User Event Simulation (`userEvent`)

The `userEvent` instance provides high-level APIs for realistic user interaction:

```ts
import { userEvent } from "@bettertui/testing";

// Click element
await userEvent.click(targetElement);

// Double click
await userEvent.doubleClick(targetElement);

// Type text into focused input
await userEvent.type("Hello world");

// Send key chords
await userEvent.keyboard("{Control>}{c}{/Control}");
await userEvent.keyboard("{Enter}");
await userEvent.keyboard("{Backspace}");

// Scroll viewport
await userEvent.scroll(targetElement, { deltaY: 3 });

// Clipboard paste
await userEvent.paste("Pasted clipboard text");
```

---

## Real End-to-End Testing (`PtyTestSession`)

For true E2E testing, `PtyTestSession` spawns your application inside a real pseudo-terminal (`node-pty`). This verifies raw terminal mode, process stdout/stderr, terminal resize events (`SIGWINCH`), and termination signals (`SIGINT`).

### E2E Test Example:

```ts
import { PtyTestSession } from "@bettertui/testing";
import { expect, test } from "vitest";

test("E2E: launches CLI application and responds to user input", async () => {
  // 1. Spawn application inside a real PTY
  const pty = new PtyTestSession({
    command: "node",
    args: ["./dist/cli.js"],
    cols: 80,
    rows: 24,
  });

  try {
    // 2. Wait for initial startup text
    await pty.waitForOutput("Welcome to BetterTUI");

    // 3. Send keyboard enter sequence
    pty.write("\r");

    // 4. Wait for menu output
    await pty.waitForOutput("Select Option");

    // 5. Test window resize (SIGWINCH)
    pty.resize(120, 40);

    // 6. Assert terminal content updated after resize
    const output = pty.getOutput();
    expect(output).toContain("Select Option");
  } finally {
    // Clean up process session
    await pty.close();
  }
});
```

---

## Custom Vitest Matchers

`@bettertui/testing` includes custom matchers for visual snapshot regression and cell assertions:

```ts
// 1. Assert visual snapshot matches golden frame
expect(terminal).toMatchGoldenFrame("dashboard-initial");

// 2. Assert component element has active focus
expect(inputElement).toBeFocused();

// 3. Assert specific coordinate cell state
expect(terminal).toHaveCell(0, 0, {
  char: "A",
  bold: true,
});
```

---

## Behavioral Specification Suite

Behavioral specs allow defining contract test suites for components using `describeBehaviour` and `BetterTUIDriver`:

```ts
import { describeBehaviour, BetterTUIDriver } from "@bettertui/testing";

describeBehaviour(
  "Button Component Contract",
  () => new BetterTUIDriver(),
  (driver) => {
    it("renders label and captures click events", async () => {
      await driver.render("<button>Click Me</button>");

      const btn = await driver.getByText("Click Me");
      expect(btn).toBeDefined();

      await driver.click(btn);
    });
  }
);
```

---

## Performance Benchmarking

Measure component render latency, frames per second (FPS), and memory allocation throughput:

```ts
import { runBenchmark } from "@bettertui/testing";

const result = await runBenchmark({
  name: "Large List Render Benchmark",
  iterations: 100,
  fn: async () => {
    render(<LargeList items={Array.from({ length: 1000 }, (_, i) => i)} />);
  },
});

console.log(`FPS: ${result.fps}`);
console.log(`Average Latency: ${result.avgLatencyMs} ms`);
```

---

## API Reference

### React Harness & Queries
- `render(ui, options?)`: Mounts React TUI component tree. Returns `{ container, rerender, unmount, terminal }`.
- `screen.getByText(matcher)`: Finds element matching text in the 2D grid.
- `screen.findByText(matcher)`: Async wait for text element to appear.
- `screen.getByRole(role)`: Finds element by structural role (`button`, `input`, etc.).
- `screen.debug()`: Prints current 2D terminal character matrix to stdout.

### Terminal & Inputs
- `TestTerminal(cols, rows)`: Headless terminal instance.
- `MockKeyboard(terminal)`: ANSI keyboard sequence encoder.
- `MockMouse(terminal)`: SGR 1006 mouse encoder.
- `userEvent`: High-level interaction helper (`click`, `type`, `keyboard`, `scroll`, `paste`).
- `PtyTestSession(options)`: Real pseudo-terminal process spawner for E2E testing.
- `runBenchmark(options)`: Latency and throughput benchmarking harness.
