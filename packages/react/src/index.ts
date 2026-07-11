import type { JSX, ReactNode } from "react";

export interface BoxProps {
  children?: ReactNode;
  flexDirection?: "row" | "column";
  justifyContent?:
    | "flex-start"
    | "center"
    | "flex-end"
    | "space-between"
    | "space-around"
    | "space-evenly";
  alignItems?: "flex-start" | "center" | "flex-end" | "stretch";
  padding?: number;
  margin?: number;
  width?: number | string;
  height?: number | string;
  style?: Record<string, unknown>;
}

export function Box(props: BoxProps): JSX.Element {
  return props.children as unknown as JSX.Element;
}

export interface TextProps {
  children?: ReactNode;
  bold?: boolean;
  italic?: boolean;
  underline?: boolean;
  color?: string;
  bgColor?: string;
  style?: Record<string, unknown>;
}

export function Text(props: TextProps): JSX.Element {
  return props.children as unknown as JSX.Element;
}

export interface FlexProps {
  children?: ReactNode;
  flexDirection?: "row" | "column";
  gap?: number;
  style?: Record<string, unknown>;
}

export function Flex(props: FlexProps): JSX.Element {
  return props.children as unknown as JSX.Element;
}

export interface SpacerProps {
  size?: number;
}

export function Spacer(_props: SpacerProps): JSX.Element {
  return null as unknown as JSX.Element;
}

export interface ProviderProps {
  children?: ReactNode;
  theme?: Record<string, unknown>;
}

export function Provider(props: ProviderProps): JSX.Element {
  return props.children as unknown as JSX.Element;
}
