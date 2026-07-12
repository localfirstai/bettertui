import { createElement } from "react";
import type { JSX, ReactNode } from "react";

export interface TerminalProps {
  program?: string;
  args?: string[];
  cwd?: string;
  env?: Record<string, string>;
  cols?: number;
  rows?: number;
  autoFocus?: boolean;
  cursorStyle?: "block" | "underline" | "bar";
  cursorBlink?: boolean;
  mouseTracking?: boolean;
  onInput?: (data: string) => void;
  onResize?: (cols: number, rows: number) => void;
  onExit?: (code: number) => void;
  style?: Record<string, unknown> | undefined;
}

export function Terminal(props: TerminalProps): JSX.Element {
  return createElement("Terminal", props);
}

export interface TerminalViewportProps {
  children?: ReactNode;
  scrollOffset?: number;
  scrollMode?: "fixed" | "scrollable" | "infinite";
  style?: Record<string, unknown> | undefined;
}

export function TerminalViewport(props: TerminalViewportProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("TerminalViewport", { style: userStyle, ...rest }, children);
}

export interface TerminalProcessProps {
  command: string;
  args?: string[];
  cwd?: string;
  env?: Record<string, string>;
  style?: Record<string, unknown> | undefined;
}

export function TerminalProcess(props: TerminalProcessProps): JSX.Element {
  return createElement("TerminalProcess", props);
}
