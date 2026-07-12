import { createElement } from "react";
import type { JSX, ReactNode } from "react";

export type FlexDirection = "row" | "column" | "row-reverse" | "column-reverse";

export type JustifyContent =
  | "flex-start"
  | "center"
  | "flex-end"
  | "space-between"
  | "space-around"
  | "space-evenly";

export type AlignItems = "flex-start" | "center" | "flex-end" | "stretch" | "baseline";

export type AlignSelf = "flex-start" | "center" | "flex-end" | "stretch" | "baseline";

export type Position = "relative" | "absolute";

export type Sizing = number | string;

export interface Padding {
  top?: number;
  right?: number;
  bottom?: number;
  left?: number;
}

export interface Margin {
  top?: number;
  right?: number;
  bottom?: number;
  left?: number;
}

export interface Inset {
  top?: number;
  right?: number;
  bottom?: number;
  left?: number;
}

export interface BoxProps {
  children?: ReactNode;
  flexDirection?: FlexDirection;
  justifyContent?: JustifyContent;
  alignItems?: AlignItems;
  alignSelf?: AlignSelf;
  flexWrap?: "nowrap" | "wrap";
  flexGrow?: number;
  flexShrink?: number;
  flexBasis?: Sizing;
  gap?: number | { row?: number; column?: number };
  padding?: number | Padding;
  paddingTop?: number;
  paddingRight?: number;
  paddingBottom?: number;
  paddingLeft?: number;
  margin?: number | Margin;
  marginTop?: number;
  marginRight?: number;
  marginBottom?: number;
  marginLeft?: number;
  width?: Sizing | undefined;
  height?: Sizing | undefined;
  minWidth?: Sizing;
  maxWidth?: Sizing;
  minHeight?: Sizing;
  maxHeight?: Sizing;
  position?: Position;
  inset?: Inset;
  top?: number;
  right?: number;
  bottom?: number;
  left?: number;
  zIndex?: number;
  visible?: boolean;
  style?: Record<string, unknown> | undefined;
}

export function Box(props: BoxProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Box", { style: userStyle, ...rest }, children);
}

export interface FlexProps {
  children?: ReactNode;
  flexDirection?: FlexDirection;
  justifyContent?: JustifyContent;
  alignItems?: AlignItems;
  alignSelf?: AlignSelf;
  flexWrap?: "nowrap" | "wrap";
  flexGrow?: number;
  flexShrink?: number;
  flexBasis?: Sizing;
  gap?: number | { row?: number; column?: number };
  padding?: number | Padding;
  paddingTop?: number;
  paddingRight?: number;
  paddingBottom?: number;
  paddingLeft?: number;
  margin?: number | Margin;
  marginTop?: number;
  marginRight?: number;
  marginBottom?: number;
  marginLeft?: number;
  width?: Sizing | undefined;
  height?: Sizing | undefined;
  minWidth?: Sizing;
  maxWidth?: Sizing;
  minHeight?: Sizing;
  maxHeight?: Sizing;
  position?: Position;
  inset?: Inset;
  top?: number;
  right?: number;
  bottom?: number;
  left?: number;
  zIndex?: number;
  visible?: boolean;
  style?: Record<string, unknown> | undefined;
}

export function Flex(props: FlexProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Flex", { style: userStyle, ...rest }, children);
}

export interface GridProps {
  children?: ReactNode;
  columns?: number;
  rows?: number;
  gap?: number | { row?: number; column?: number };
  columnGap?: number;
  rowGap?: number;
  padding?: number | Padding;
  paddingTop?: number;
  paddingRight?: number;
  paddingBottom?: number;
  paddingLeft?: number;
  margin?: number | Margin;
  marginTop?: number;
  marginRight?: number;
  marginBottom?: number;
  marginLeft?: number;
  width?: Sizing | undefined;
  height?: Sizing | undefined;
  minWidth?: Sizing;
  maxWidth?: Sizing;
  minHeight?: Sizing;
  maxHeight?: Sizing;
  position?: Position;
  inset?: Inset;
  top?: number;
  right?: number;
  bottom?: number;
  left?: number;
  zIndex?: number;
  visible?: boolean;
  style?: Record<string, unknown> | undefined;
}

export function Grid(props: GridProps): JSX.Element {
  const { children, style: userStyle, ...rest } = props;
  return createElement("Grid", { style: userStyle, ...rest }, children);
}

export interface StackProps {
  children?: ReactNode;
  width?: Sizing | undefined;
  height?: Sizing | undefined;
  padding?: number | Padding;
  margin?: number | Margin;
  position?: Position;
  zIndex?: number;
  visible?: boolean;
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

export interface SpacerProps {
  size?: number;
}

export function Spacer(props: SpacerProps): JSX.Element {
  return createElement("Spacer", { size: props.size });
}

export interface SeparatorProps {
  orientation?: "horizontal" | "vertical";
  style?: Record<string, unknown> | undefined;
}

export function Separator(props: SeparatorProps): JSX.Element {
  return createElement("Separator", { orientation: props.orientation, style: props.style });
}
