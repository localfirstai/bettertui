import type { CodeOptions, TextOptions } from "@bettertui/core";
import { createElement } from "react";
import type { JSX, ReactNode } from "react";

export interface TextProps extends TextOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function Text(props: TextProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Text", { style: userStyle, ...rest }, children);
}

export interface CodeProps extends CodeOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function Code({ children, style, ...rest }: CodeProps): JSX.Element {
  return createElement("Code", { style: { bg: "bright_black", ...style }, ...rest }, children);
}
