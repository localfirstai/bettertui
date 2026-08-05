import { CliRenderer } from "@bettertui/core";
import { type Root, createRoot } from "@bettertui/react";
import { type ReactNode, act } from "react";
import { screen } from "../screen/screen";
import { TestTerminal, type TestTerminalOptions } from "../terminal/test-terminal";

export interface RenderOptions extends TestTerminalOptions {
  renderer?: CliRenderer;
}

export interface RenderResult {
  terminal: TestTerminal;
  unmount: () => void;
  rerender: (node: ReactNode) => void;
  debug: () => void;
}

function setIsReactActEnvironment(enabled: boolean): void {
  // @ts-expect-error - React 19 testing flag
  globalThis.IS_REACT_ACT_ENVIRONMENT = enabled;
}

export function render(node: ReactNode, options: RenderOptions = {}): RenderResult {
  setIsReactActEnvironment(true);

  const terminal = new TestTerminal(options);
  screen.setTerminal(terminal);

  const cliRenderer =
    options.renderer ?? new CliRenderer({ width: terminal.width, height: terminal.height });
  const root: Root = createRoot(cliRenderer);

  act(() => {
    root.render(node);
  });

  return {
    terminal,
    unmount: () => {
      act(() => {
        root.unmount();
      });
      setIsReactActEnvironment(false);
    },
    rerender: (newNode: ReactNode) => {
      act(() => {
        root.render(newNode);
      });
    },
    debug: () => screen.debug(),
  };
}
