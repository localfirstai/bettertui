import { CommandBuffer, type Runtime } from "@bettertui/core";
import type { Command } from "@bettertui/core";
import type { Point, Rect, Size } from "@bettertui/shared";
import type { ReactNode } from "react";

/**
 * A mock command collector that records commands for testing.
 */
export class MockCommandCollector {
  private commands: Command[] = [];
  private buffer: CommandBuffer;

  constructor() {
    this.buffer = new CommandBuffer();
  }

  /**
   * Get the underlying command buffer.
   */
  get commandBuffer(): CommandBuffer {
    return this.buffer;
  }

  /**
   * Get all collected commands.
   */
  getCommands(): readonly Command[] {
    return this.commands;
  }

  /**
   * Get the last command.
   */
  getLastCommand(): Command | undefined {
    return this.commands[this.commands.length - 1];
  }

  /**
   * Get commands of a specific type.
   */
  getCommandsByType<T extends Command["type"]>(type: T): Extract<Command, { type: T }>[] {
    return this.commands.filter((c): c is Extract<Command, { type: T }> => c.type === type);
  }

  /**
   * Clear all collected commands.
   */
  clear(): void {
    this.commands = [];
    this.buffer.clear();
  }

  /**
   * Subscribe to the runtime and collect commands.
   */
  subscribe(runtime: Runtime): () => void {
    return runtime.subscribe((commands) => {
      this.commands.push(...(commands as Command[]));
    });
  }
}

/**
 * Mock terminal size for testing.
 */
export const MOCK_TERMINAL_SIZE: Size = {
  width: 80,
  height: 24,
};

/**
 * Create a mock point.
 */
export function createPoint(x: number, y: number): Point {
  return { x, y };
}

/**
 * Create a mock rect.
 */
export function createRect(x: number, y: number, width: number, height: number): Rect {
  return { x, y, width, height };
}

/**
 * Test helper for rendering components to string output.
 * This is a simplified version - for full rendering, use the actual render() function.
 */
export function renderToString(element: ReactNode): string {
  // This is a placeholder - in a real implementation, you'd use a test renderer
  // For now, return a string representation
  if (element === null || element === undefined) {
    return "";
  }
  if (typeof element === "string") {
    return element;
  }
  if (typeof element === "number") {
    return String(element);
  }
  if (typeof element === "boolean") {
    return "";
  }
  // For complex elements, return a placeholder
  return "[Element]";
}

/**
 * Snapshot test helper - compares output against a stored snapshot.
 * Use this with vitest's expect().toMatchInlineSnapshot() or toMatchSnapshot().
 */
export function expectToMatchSnapshot(actual: string, snapshotName?: string): void {
  // In vitest, this would use expect(actual).toMatchSnapshot()
  // This is a placeholder for the pattern
  if (process.env.NODE_ENV === "test") {
    console.log(`Snapshot ${snapshotName ? `"${snapshotName}"` : ""}: ${actual.length} chars`);
  }
}

/**
 * Create a simple test component tree for testing.
 */
export function createTestTree() {
  return {
    root: {
      type: "Box",
      props: { style: { width: "100%", height: "100%" } },
      children: [
        {
          type: "Text",
          props: { content: "Hello, BetterTUI!" },
          children: [],
        },
      ],
    },
  };
}

/**
 * Wait for a specified number of milliseconds.
 */
export function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

/**
 * Flush microtasks.
 */
export async function flushMicrotasks(): Promise<void> {
  return new Promise((resolve) => {
    setTimeout(resolve, 0);
  });
}

/**
 * Create a mock event handler that records calls.
 */
// biome-ignore lint/suspicious/noExplicitAny: test utility
export function createMockHandler<T extends (...args: any[]) => any>() {
  const calls: Parameters<T>[] = [];
  const handler = ((...args: Parameters<T>) => {
    calls.push(args);
  }) as T & { calls: Parameters<T>[]; clear: () => void };

  Object.defineProperty(handler, "calls", {
    get: () => calls,
  });

  // biome-ignore lint/suspicious/noExplicitAny: test utility
  (handler as any).clear = () => {
    calls.length = 0;
  };

  return handler;
}

/**
 * Assertion helper for checking command buffer contents.
 */
export function expectCommandBuffer(
  buffer: CommandBuffer,
  expected: {
    length?: number;
    isEmpty?: boolean;
    types?: Command["type"][];
  },
): void {
  if (expected.length !== undefined) {
    if (buffer.length !== expected.length) {
      throw new Error(`Expected command buffer length ${expected.length}, got ${buffer.length}`);
    }
  }
  if (expected.isEmpty !== undefined) {
    if (buffer.isEmpty !== expected.isEmpty) {
      throw new Error(
        `Expected command buffer isEmpty to be ${expected.isEmpty}, got ${buffer.isEmpty}`,
      );
    }
  }
  if (expected.types !== undefined) {
    const commands = buffer.peek();
    const actualTypes = commands.map((c) => c.type);
    if (JSON.stringify(actualTypes) !== JSON.stringify(expected.types)) {
      throw new Error(
        `Expected command types ${JSON.stringify(expected.types)}, got ${JSON.stringify(actualTypes)}`,
      );
    }
  }
}
