import type { CliRenderer, CliRendererOptions } from "../platform/cliRenderer";
import { CliRenderer as CliRendererClass, createCliRenderer } from "../platform/cliRenderer";
import { createMockKeys } from "./mockKeys";
import { createMockMouse } from "./mockMouse";
import { createTestStdin, createTestStdout } from "./testStreams";
import type { TestStdin, TestStdout } from "./testStreams";

export interface TestRendererOptions extends CliRendererOptions {
  width?: number;
  height?: number;
  kittyKeyboard?: boolean;
}

export type TestRenderer = CliRenderer;
export type MockInput = ReturnType<typeof createMockKeys>;
export type MockMouse = ReturnType<typeof createMockMouse>;

export interface TestRendererSetup {
  renderer: TestRenderer;
  mockInput: MockInput;
  mockMouse: MockMouse;
  stdin: TestStdin;
  stdout: TestStdout;
  renderOnce: () => void;
  captureFrame: () => string;
  resize: (width: number, height: number) => void;
  cleanup: () => void;
}

export async function createTestRenderer(
  options: TestRendererOptions = {},
): Promise<TestRendererSetup> {
  const width = options.width ?? 80;
  const height = options.height ?? 24;

  const stdin = createTestStdin();
  const stdout = createTestStdout(width, height);

  const originalStdin = process.stdin;
  const originalStdout = process.stdout;

  Object.defineProperty(process, "stdin", { value: stdin, writable: true, configurable: true });
  Object.defineProperty(process, "stdout", { value: stdout, writable: true, configurable: true });

  const renderer = await createCliRenderer({
    width,
    height,
    ...options,
  });

  const mockInput = createMockKeys(renderer, { kittyKeyboard: options.kittyKeyboard });
  const mockMouse = createMockMouse();

  const renderOnce = (): void => {
    renderer.render();
  };

  const captureFrame = (): string => {
    return stdout.getOutput();
  };

  const resize = (newWidth: number, newHeight: number): void => {
    stdout.columns = newWidth;
    stdout.rows = newHeight;
    renderer.resize(newWidth, newHeight);
  };

  const cleanup = (): void => {
    renderer.stop();
    Object.defineProperty(process, "stdin", {
      value: originalStdin,
      writable: true,
      configurable: true,
    });
    Object.defineProperty(process, "stdout", {
      value: originalStdout,
      writable: true,
      configurable: true,
    });
    stdout.clear();
  };

  return {
    renderer,
    mockInput,
    mockMouse,
    stdin,
    stdout,
    renderOnce,
    captureFrame,
    resize,
    cleanup,
  };
}

export function createTestRendererSync(options: TestRendererOptions = {}): TestRendererSetup {
  const width = options.width ?? 80;
  const height = options.height ?? 24;

  const stdin = createTestStdin();
  const stdout = createTestStdout(width, height);

  const originalStdin = process.stdin;
  const originalStdout = process.stdout;

  Object.defineProperty(process, "stdin", { value: stdin, writable: true, configurable: true });
  Object.defineProperty(process, "stdout", { value: stdout, writable: true, configurable: true });

  const renderer = new CliRendererClass({
    width,
    height,
    ...options,
  });

  const mockInput = createMockKeys(renderer, { kittyKeyboard: options.kittyKeyboard });
  const mockMouse = createMockMouse();

  const renderOnce = (): void => {
    renderer.render();
  };

  const captureFrame = (): string => {
    return stdout.getOutput();
  };

  const resize = (newWidth: number, newHeight: number): void => {
    stdout.columns = newWidth;
    stdout.rows = newHeight;
    renderer.resize(newWidth, newHeight);
  };

  const cleanup = (): void => {
    renderer.stop();
    Object.defineProperty(process, "stdin", {
      value: originalStdin,
      writable: true,
      configurable: true,
    });
    Object.defineProperty(process, "stdout", {
      value: originalStdout,
      writable: true,
      configurable: true,
    });
    stdout.clear();
  };

  return {
    renderer,
    mockInput,
    mockMouse,
    stdin,
    stdout,
    renderOnce,
    captureFrame,
    resize,
    cleanup,
  };
}
