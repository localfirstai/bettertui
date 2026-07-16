import type { BoxOptions } from "@bettertui/core";
import { createElement } from "react";
import type { JSX, ReactNode } from "react";

export interface BoxProps extends BoxOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function Box(props: BoxProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Box", { style: userStyle, ...rest }, children);
}

export interface FlexProps extends BoxProps {
  flexDirection?: "row" | "column" | "row-reverse" | "column-reverse";
  justifyContent?:
    | "flex-start"
    | "flex-end"
    | "center"
    | "space-between"
    | "space-around"
    | "space-evenly";
  alignItems?: "flex-start" | "flex-end" | "center" | "stretch" | "baseline";
  gap?: number;
}

export function Flex(props: FlexProps): JSX.Element {
  const {
    children,
    flexDirection = "row",
    justifyContent,
    alignItems,
    gap,
    style,
    ...rest
  } = props;
  return createElement(
    "Box",
    { style: { ...style, flexDirection, justifyContent, alignItems, gap }, ...rest },
    children,
  );
}

export type FlexDirection = NonNullable<BoxOptions["flexDirection"]>;
export type JustifyContent = NonNullable<BoxOptions["justifyContent"]>;
export type AlignItems = NonNullable<BoxOptions["alignItems"]>;
export type Position = NonNullable<BoxOptions["position"]>;
export type Sizing = BoxOptions["width"];
export type Padding = NonNullable<BoxOptions["padding"]>;
export type Margin = NonNullable<BoxOptions["margin"]>;
export type Overflow = NonNullable<BoxOptions["overflow"]>;
