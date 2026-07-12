import { createElement } from "react";
import type { JSX, ReactNode } from "react";

export interface ToastProps {
  message: string;
  variant?: "default" | "success" | "warning" | "error" | "info";
  duration?: number;
  onDismiss?: () => void;
  style?: Record<string, unknown> | undefined;
}

export function Toast(props: ToastProps): JSX.Element {
  return createElement("Toast", props);
}

export interface StatusLineProps {
  children?: ReactNode;
  items?: Array<{ label: string; value?: string; separator?: boolean }>;
  style?: Record<string, unknown> | undefined;
}

export function StatusLine(props: StatusLineProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("StatusLine", { style: userStyle, ...rest }, children);
}
