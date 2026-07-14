import { createElement } from "react";
import type { JSX, ReactNode } from "react";

export interface TextProps {
  children?: ReactNode;
  bold?: boolean | undefined;
  italic?: boolean | undefined;
  underline?: boolean | undefined;
  dim?: boolean | undefined;
  strikethrough?: boolean | undefined;
  color?: string | undefined;
  bgColor?: string | undefined;
  style?: Record<string, unknown> | undefined;
}

export function Text(props: TextProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Text", { style: userStyle, ...rest }, children);
}

export interface HeadingProps {
  children?: ReactNode;
  level?: 1 | 2 | 3 | 4 | 5 | 6;
  style?: Record<string, unknown> | undefined;
}

export function Heading(props: HeadingProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  const mergedStyle = { textAlign: "center" as const, ...userStyle };
  return createElement("Heading", { style: mergedStyle, ...rest }, children);
}

export interface LabelProps {
  children?: ReactNode;
  htmlFor?: string;
  style?: Record<string, unknown> | undefined;
}

export function Label(props: LabelProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Label", { style: userStyle, ...rest }, children);
}

export interface CodeProps {
  children?: ReactNode;
  inline?: boolean;
  language?: string;
  style?: Record<string, unknown> | undefined;
}

export function Code({ children, style, ...rest }: CodeProps): JSX.Element {
  return createElement(
    "Text",
    { style: { bg: "bright_black", ...style }, ...rest },
    ` ${children} `,
  );
}

export interface BlockquoteProps {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function Blockquote(props: BlockquoteProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Blockquote", { style: userStyle, ...rest }, children);
}
