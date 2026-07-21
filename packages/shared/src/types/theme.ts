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

import type { BorderStyle } from "./style";

/** A complete theme definition. Mirrors the Rust engine's Theme struct exactly. */
export interface Theme {
  /** Human-readable theme identifier (e.g. "dark", "light") */
  name: string;
  colors: ThemeColors;
  spacing: ThemeSpacing;
  borders: BorderStyle;
}
