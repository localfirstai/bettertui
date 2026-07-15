import type {
  BoxOptions,
  FlexOptions as CoreFlexOptions,
  GridOptions,
  SeparatorOptions,
  SpacerOptions,
  StackOptions,
} from "@bettertui/core";
import { createElement } from "react";
import type { JSX, ReactNode } from "react";

export type FlexDirection = NonNullable<BoxOptions["flexDirection"]>;
export type JustifyContent = NonNullable<BoxOptions["justifyContent"]>;
export type AlignItems = NonNullable<BoxOptions["alignItems"]>;
export type AlignSelf = NonNullable<BoxOptions["alignSelf"]>;
export type Position = NonNullable<BoxOptions["position"]>;
export type Sizing = BoxOptions["width"];
export type Padding = NonNullable<BoxOptions["padding"]>;
export type Margin = NonNullable<BoxOptions["margin"]>;
export type Inset = NonNullable<BoxOptions["inset"]>;

export interface BoxProps extends BoxOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function Box(props: BoxProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Box", { style: userStyle, ...rest }, children);
}

export interface FlexProps extends CoreFlexOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function Flex(props: FlexProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Flex", { style: userStyle, ...rest }, children);
}

export interface GridProps extends GridOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export function Grid(props: GridProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Grid", { style: userStyle, ...rest }, children);
}

export interface StackProps extends StackOptions {
  children?: ReactNode;
  style?: Record<string, unknown> | undefined;
}

export interface StackChildProps {
  zIndex?: number;
  offsetX?: number;
  offsetY?: number;
}

export function Stack(props: StackProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Stack", { style: userStyle, ...rest }, children);
}

export interface SpacerProps extends SpacerOptions {}

export function Spacer(props: SpacerProps): JSX.Element {
  return createElement("Spacer", { size: props.size });
}

export interface SeparatorProps extends SeparatorOptions {
  style?: Record<string, unknown> | undefined;
}

export function Separator(props: SeparatorProps): JSX.Element {
  return createElement("Separator", { orientation: props.orientation, style: props.style });
}
