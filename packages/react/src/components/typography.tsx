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

export interface HeadingProps {
  children?: ReactNode;
  level?: 1 | 2 | 3 | 4 | 5 | 6;
  style?: Record<string, unknown> | undefined;
}

export function Heading({ children, level = 1, style }: HeadingProps): JSX.Element {
  const styles: Record<number, { bold?: boolean; underline?: boolean }> = {
    1: { bold: true },
    2: { bold: true },
    3: { bold: true },
    4: { bold: true },
    5: { bold: true },
    6: { bold: true, underline: true },
  };
  return createElement("Text", { style: { ...styles[level], ...style } }, children);
}
