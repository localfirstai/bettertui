/** A 2D coordinate in the terminal grid. */
export interface Point {
  /** Column position (0-indexed from left) */
  x: number;
  /** Row position (0-indexed from top) */
  y: number;
}

/** Width and height dimensions. */
export interface Size {
  /** Width in columns */
  width: number;
  /** Height in rows */
  height: number;
}

/** A rectangular region defined by position and size. */
export interface Rect {
  /** Left column offset */
  x: number;
  /** Top row offset */
  y: number;
  /** Width in columns */
  width: number;
  /** Height in rows */
  height: number;
}

/** Direction of the main axis in a flex layout. */
export type FlexDirection = "row" | "column" | "row-reverse" | "column-reverse";

/** Alignment of children along the main axis. */
export type JustifyContent =
  | "flex-start"
  | "center"
  | "flex-end"
  | "space-between"
  | "space-around"
  | "space-evenly";

/** Alignment of children along the cross axis. */
export type AlignItems = "flex-start" | "center" | "flex-end" | "stretch" | "baseline";

/** Alignment of a single child along the cross axis, overriding AlignItems. */
export type AlignSelf = "flex-start" | "center" | "flex-end" | "stretch" | "baseline";

/** Positioning strategy for a layout node. */
export type Position = "relative" | "absolute";

/** A dimension value — number for fixed columns/rows, string for percentage or calc. */
export type Sizing = number | string;

/** Behavior when content overflows the container bounds. */
export type Overflow = "visible" | "hidden" | "scroll";

/** Shorthand for all four padding sides. */
export interface Padding {
  top?: number;
  right?: number;
  bottom?: number;
  left?: number;
}

/** Shorthand for all four margin sides. */
export interface Margin {
  top?: number;
  right?: number;
  bottom?: number;
  left?: number;
}

/** Shorthand for all four inset offsets (used with absolute positioning). */
export interface Inset {
  top?: number;
  right?: number;
  bottom?: number;
  left?: number;
}

/** Row and column gap values for flex layouts. */
export interface Gap {
  row?: number;
  column?: number;
}

/** All layout-affecting properties for a UI node. */
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

/** A keyboard event from the terminal. */
export interface KeyEvent {
  /** The key value (e.g. "a", "Enter", "Escape") */
  key: string;
  /** Physical key code (e.g. "KeyA", "Enter") */
  code: string;
  /** Whether Ctrl was held */
  ctrl: boolean;
  /** Whether Shift was held */
  shift: boolean;
  /** Whether Alt was held */
  alt: boolean;
  /** Whether Meta (Cmd/Windows) was held */
  meta: boolean;
}

/** Mouse button identifier. */
export type MouseButton = "left" | "right" | "middle" | "none";

/** A mouse event from the terminal. */
export interface MouseEvent {
  button: MouseButton;
  /** Terminal-grid position where the event occurred */
  position: Point;
  ctrl: boolean;
  shift: boolean;
  alt: boolean;
}

/** A CSS-like color value: named color, hex, rgb(), or rgba(). */
export type ColorValue = string;

/** Visual style properties for a UI node. */
export interface Style {
  /** Foreground (text) color */
  fg?: ColorValue;
  /** Background color */
  bg?: ColorValue;
  bold?: boolean;
  italic?: boolean;
  underline?: boolean;
  dim?: boolean;
  strikethrough?: boolean;
  /** Swap foreground and background colors */
  inverse?: boolean;
  /** Text alignment */
  textAlign?: "left" | "center" | "right" | "justify";
}

/** All possible border visual styles. */
export type BorderStyleKind = "none" | "solid" | "dashed" | "dotted" | "double";

/** Border visual configuration. */
export interface BorderStyle {
  style: BorderStyleKind;
  /** Border foreground color */
  fg?: ColorValue;
}

/** Semantic color slots used by a Theme. Matches the Rust engine's ThemeColors struct. */
export interface ThemeColors {
  /** Primary background for the entire application */
  background: string;
  /** Default surface background for containers */
  surface: string;
  /** Elevated surface with higher emphasis */
  surfaceHigh: string;
  /** Lower-emphasis surface for subtle backgrounds */
  surfaceLow: string;
  /** Primary brand color for interactive elements */
  primary: string;
  /** Text on primary backgrounds */
  primaryForeground: string;
  /** Secondary brand accent */
  secondary: string;
  /** Text on secondary backgrounds */
  secondaryForeground: string;
  /** Primary text color */
  text: string;
  /** Muted text for less prominent content */
  textMuted: string;
  /** Dimmed text for placeholders and disabled state */
  textDim: string;
  /** Default border color */
  border: string;
  /** Border color for focused/active elements */
  borderFocused: string;
  /** Accent color for highlights and call-to-actions */
  accent: string;
  /** Text on accent backgrounds */
  accentForeground: string;
  /** Error/semantic red */
  error: string;
  /** Warning/semantic yellow */
  warning: string;
  /** Success/semantic green */
  success: string;
  /** Info/semantic blue */
  info: string;
  /** Scrollbar track background */
  scrollbar: string;
  /** Scrollbar thumb (draggable handle) */
  scrollbarThumb: string;
}

/** Spacing scale tokens. Maps to the Rust engine's ThemeSpacing struct. */
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

/** A complete theme definition. Mirrors the Rust engine's Theme struct exactly. */
export interface Theme {
  /** Human-readable theme identifier (e.g. "dark", "light") */
  name: string;
  colors: ThemeColors;
  spacing: ThemeSpacing;
  borders: BorderStyle;
}

/** Describes a single validation failure. */
export interface ValidationError {
  /** The name of the property that failed validation */
  field: string;
  /** Human-readable description of the failure */
  message: string;
}

/** Aggregated result of running validations. */
export interface ValidationResult {
  /** Whether all validations passed */
  valid: boolean;
  /** List of individual validation errors (empty when valid) */
  errors: ValidationError[];
}
