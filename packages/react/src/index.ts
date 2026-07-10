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

export function Box(_props: BoxProps): JSX.Element {
  return null as unknown as JSX.Element;
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

export function Text(_props: TextProps): JSX.Element {
  return null as unknown as JSX.Element;
}

export interface FlexProps {
  children?: ReactNode;
  flexDirection?: "row" | "column";
  gap?: number;
  style?: Record<string, unknown>;
}

export function Flex(_props: FlexProps): JSX.Element {
  return null as unknown as JSX.Element;
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

export function Provider(_props: ProviderProps): JSX.Element {
  return null as unknown as JSX.Element;
}
