// ─── Geometry ─────────────────────────────────────────────

export interface Point {
  x: number;
  y: number;
}

export interface Size {
  width: number;
  height: number;
}

export interface Rect {
  x: number;
  y: number;
  width: number;
  height: number;
}

// ─── Layout ───────────────────────────────────────────────

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

export type Overflow = "visible" | "hidden" | "scroll";

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

export interface Gap {
  row?: number;
  column?: number;
}

export interface LayoutConstraints {
  display?: "flex" | "none";
  flexDirection?: FlexDirection;
  justifyContent?: JustifyContent;
  alignItems?: AlignItems;
  alignSelf?: AlignSelf;
  flexWrap?: "nowrap" | "wrap";
  flexGrow?: number;
  flexShrink?: number;
  flexBasis?: Sizing;
  gap?: number | Gap;
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
  width?: Sizing;
  height?: Sizing;
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
  overflow?: Overflow;
}

// ─── Events ──────────────────────────────────────────────

export interface KeyEvent {
  key: string;
  code: string;
  ctrl: boolean;
  shift: boolean;
  alt: boolean;
  meta: boolean;
}

export type MouseButton = "left" | "right" | "middle" | "none";

export interface MouseEvent {
  button: MouseButton;
  position: Point;
  ctrl: boolean;
  shift: boolean;
  alt: boolean;
}

// ─── Styling ─────────────────────────────────────────────

export type ColorValue = string;

export interface Style {
  fg?: ColorValue;
  bg?: ColorValue;
  bold?: boolean;
  italic?: boolean;
  underline?: boolean;
  dim?: boolean;
  strikethrough?: boolean;
  inverse?: boolean;
}

export interface BorderStyle {
  style: "none" | "single" | "double" | "rounded" | "thick" | "block";
  fg?: ColorValue;
}

// ─── Theme ───────────────────────────────────────────────

export interface ThemeColors {
  background: string;
  surface: string;
  surfaceHigh: string;
  surfaceLow: string;
  primary: string;
  primaryForeground: string;
  secondary: string;
  secondaryForeground: string;
  text: string;
  textMuted: string;
  textDim: string;
  border: string;
  borderFocused: string;
  accent: string;
  accentForeground: string;
  error: string;
  warning: string;
  success: string;
  info: string;
}

export interface ThemeSpacing {
  none: number;
  xxs: number;
  xs: number;
  sm: number;
  md: number;
  lg: number;
  xl: number;
  xxl: number;
}

export interface Theme {
  name: string;
  colors: ThemeColors;
  spacing: ThemeSpacing;
  borders: BorderStyle;
}
