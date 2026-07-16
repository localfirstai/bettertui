import type { JSX, ReactNode } from "react";

export interface ScrollBarProps {
  orientation?: "vertical" | "horizontal";
  thumbSize?: number;
  trackSize?: number;
  position?: number;
  style?: Record<string, unknown> | undefined;
}

export function ScrollBar(_props: ScrollBarProps): JSX.Element {
  return <div />;
}

export interface ScrollBoxProps {
  width?: number | "auto" | "100%";
  height?: number | "auto" | "100%";
  scrollX?: boolean;
  scrollY?: boolean;
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function ScrollBox(_props: ScrollBoxProps): JSX.Element {
  return <div />;
}
