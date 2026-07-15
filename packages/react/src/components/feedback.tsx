import type { BadgeOptions, ProgressOptions, SpinnerOptions } from "@bettertui/core";
import { createElement } from "react";
import type { JSX, ReactNode } from "react";

export interface BadgeProps extends BadgeOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function Badge(props: BadgeProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Badge", { style: userStyle, ...rest }, children);
}

export interface ProgressProps extends ProgressOptions {
  style?: Record<string, unknown> | undefined;
}

export function Progress(props: ProgressProps): JSX.Element {
  return createElement("Progress", props);
}

export interface SpinnerProps extends SpinnerOptions {
  style?: Record<string, unknown> | undefined;
}

export function Spinner(props: SpinnerProps): JSX.Element {
  return createElement("Spinner", props);
}
