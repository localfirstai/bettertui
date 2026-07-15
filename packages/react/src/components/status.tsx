import type { StatusLineOptions, ToastOptions } from "@bettertui/core";
import { createElement } from "react";
import type { JSX, ReactNode } from "react";

export interface ToastProps extends ToastOptions {
  style?: Record<string, unknown> | undefined;
}

export function Toast(props: ToastProps): JSX.Element {
  return createElement("Toast", props);
}

export interface StatusLineProps extends StatusLineOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function StatusLine(props: StatusLineProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("StatusLine", { style: userStyle, ...rest }, children);
}
