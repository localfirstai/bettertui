import type {
  TerminalOptions,
  TerminalProcessOptions,
  TerminalViewportOptions,
} from "@bettertui/core";
import { createElement } from "react";
import type { JSX, ReactNode } from "react";

export interface TerminalProps extends TerminalOptions {
  style?: Record<string, unknown> | undefined;
}

export function Terminal(props: TerminalProps): JSX.Element {
  return createElement("Terminal", props);
}

export interface TerminalViewportProps extends TerminalViewportOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function TerminalViewport(props: TerminalViewportProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("TerminalViewport", { style: userStyle, ...rest }, children);
}

export interface TerminalProcessProps extends TerminalProcessOptions {
  style?: Record<string, unknown> | undefined;
}

export function TerminalProcess(props: TerminalProcessProps): JSX.Element {
  return createElement("TerminalProcess", props);
}
