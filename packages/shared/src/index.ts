export type NodeId = string;

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

export type Direction = "horizontal" | "vertical";

export type Alignment = "start" | "center" | "end" | "stretch";

export type Overflow = "visible" | "hidden" | "scroll";

export interface LayoutConstraints {
  minWidth?: number;
  maxWidth?: number;
  minHeight?: number;
  maxHeight?: number;
}

export interface LayoutResult {
  rect: Rect;
  children: LayoutResult[];
}

export interface RenderCommand {
  type: "text" | "rect" | "clear";
  rect?: Rect;
  text?: string;
  style?: Style;
}

// ─── Events ──────────────────────────────────────────────

export type EventType = "key" | "mouse" | "resize" | "focus" | "blur" | "paste" | "custom";

export interface Event {
  type: EventType;
  timestamp: number;
  data: unknown;
}

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

export interface ResizeEvent {
  columns: number;
  rows: number;
}

export interface PasteEvent {
  text: string;
  bracketed: boolean;
}

// ─── Styling ─────────────────────────────────────────────

export type ColorValue = string;

export interface Color {
  r: number;
  g: number;
  b: number;
  a?: number;
}

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

// ─── Frame ───────────────────────────────────────────────

export interface Frame {
  width: number;
  height: number;
  cells: FrameCell[];
}

export interface FrameCell {
  char: string;
  style: Style;
}

export interface RenderNode {
  id: NodeId;
  type: string;
  props: Record<string, unknown>;
  children: RenderNode[];
  style: Style;
  layout: LayoutConstraints;
}
