import type {
  BlockquoteOptions,
  CodeOptions,
  HeadingOptions,
  LabelOptions,
  TextOptions,
} from "@bettertui/core";
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

export interface HeadingProps extends HeadingOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function Heading(props: HeadingProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  const mergedStyle = { textAlign: "center" as const, ...userStyle };
  return createElement("Heading", { style: mergedStyle, ...rest }, children);
}

export interface LabelProps extends LabelOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function Label(props: LabelProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Label", { style: userStyle, ...rest }, children);
}

export interface CodeProps extends CodeOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function Code({ children, style, ...rest }: CodeProps): JSX.Element {
  return createElement(
    "Text",
    { style: { bg: "bright_black", ...style }, ...rest },
    ` ${children} `,
  );
}

export interface BlockquoteProps extends BlockquoteOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function Blockquote(props: BlockquoteProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Blockquote", { style: userStyle, ...rest }, children);
}
