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

export type FlexDirection = NonNullable<BoxOptions["flexDirection"]>;
export type JustifyContent = NonNullable<BoxOptions["justifyContent"]>;
export type AlignItems = NonNullable<BoxOptions["alignItems"]>;
export type Position = NonNullable<BoxOptions["position"]>;
export type Sizing = BoxOptions["width"];
export type Padding = NonNullable<BoxOptions["padding"]>;
export type Margin = NonNullable<BoxOptions["margin"]>;
export type Overflow = NonNullable<BoxOptions["overflow"]>;
