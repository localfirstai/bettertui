import { createElement } from "react";
import type { JSX, ReactNode } from "react";

export interface BadgeProps {
  children?: ReactNode;
  variant?: "default" | "primary" | "success" | "warning" | "danger" | "info";
  style?: Record<string, unknown> | undefined;
}

export function Badge(props: BadgeProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Badge", { style: userStyle, ...rest }, children);
}

export interface ProgressProps {
  value?: number;
  max?: number;
  style?: Record<string, unknown> | undefined;
}

export function Progress(props: ProgressProps): JSX.Element {
  return createElement("Progress", props);
}

export interface SpinnerProps {
  label?: string;
  type?: "dots" | "line" | "braille" | "arc";
  style?: Record<string, unknown> | undefined;
}

export function Spinner(props: SpinnerProps): JSX.Element {
  return createElement("Spinner", props);
}
