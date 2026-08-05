/**
 * Comprehensive theme system inspired by shadcn/ui
 * Provides semantic color tokens for consistent UI theming
 */

import { RGBA } from "@bettertui/core";

/** Theme mode type */
export type ThemeMode = "dark" | "light";

/**
 * Component theme interface - semantic colors without suffixes
 */
export interface ComponentTheme {
  // App
  appBackground: string;
  title: RGBA;

  // Box/Border
  border: string;
  focusedBorder: string;

  // Input
  inputText: string;
  inputFocusedText: string;
  inputPlaceholder: string;
  inputCursor: string;

  // Select
  selectSelectedBackground: string;
  selectText: string;
  selectSelectedText: string;
  selectDescription: string;
  selectSelectedDescription: string;

  // Text
  instructions: string;
  notImplemented: string;
}

/**
 * Semantic color tokens following shadcn/ui naming conventions
 */
export interface ThemeTokens {
  /** Primary background color */
  background: string;
  /** Default text color */
  foreground: string;

  /** Primary brand color */
  primary: string;
  /** Text color on primary background */
  primaryForeground: string;

  /** Secondary background color */
  secondary: string;
  secondaryForeground: string;

  /** Subdued background */
  muted: string;
  mutedForeground: string;

  /** Accent color */
  accent: string;
  accentForeground: string;

  /** Destructive/error color */
  destructive: string;
  destructiveForeground: string;

  /** Border color */
  border: string;
  /** Input field background */
  input: string;
  /** Focus ring color */
  ring: string;

  /** Success state */
  success: string;
  /** Warning state */
  warning: string;
  /** Info state */
  info: string;
}

/**
 * Complete theme configuration
 */
export interface AppTheme {
  name: string;
  mode: ThemeMode;
  tokens: ThemeTokens;
  components: ComponentTheme;
}

// ============================================================================
// Shadcn-inspired Dark Theme
// Color palette from shadcn/ui dark mode
// ============================================================================
function createDarkTheme(): AppTheme {
  const tokens: ThemeTokens = {
    background: "#0A0A0A",
    foreground: "#FAFAFA",
    primary: "#FAFAFA",
    primaryForeground: "#171717",
    secondary: "#262626",
    secondaryForeground: "#FAFAFA",
    muted: "#171717",
    mutedForeground: "#A3A3A3",
    accent: "#262626",
    accentForeground: "#FAFAFA",
    destructive: "#7F1D1D",
    destructiveForeground: "#FEE2E2",
    border: "#262626",
    input: "#262626",
    ring: "#A3A3A3",
    success: "#166534",
    warning: "#92400E",
    info: "#1E40AF",
  };

  return {
    name: "neutral-dark",
    mode: "dark",
    tokens,
    components: {
      appBackground: tokens.background,
      title: RGBA.fromInts(250, 250, 250, 255),
      border: tokens.border,
      focusedBorder: tokens.ring,
      inputText: tokens.foreground,
      inputFocusedText: tokens.foreground,
      inputPlaceholder: tokens.mutedForeground,
      inputCursor: tokens.primary,
      selectSelectedBackground: tokens.secondary,
      selectText: tokens.foreground,
      selectSelectedText: tokens.primary,
      selectDescription: tokens.mutedForeground,
      selectSelectedDescription: tokens.accentForeground,
      instructions: tokens.mutedForeground,
      notImplemented: "#EAB308",
    },
  };
}

// ============================================================================
// Shadcn-inspired Light Theme
// ============================================================================
function createLightTheme(): AppTheme {
  const tokens: ThemeTokens = {
    background: "#FFFFFF",
    foreground: "#0A0A0A",
    primary: "#171717",
    primaryForeground: "#FAFAFA",
    secondary: "#F5F5F5",
    secondaryForeground: "#171717",
    muted: "#F5F5F5",
    mutedForeground: "#737373",
    accent: "#F5F5F5",
    accentForeground: "#171717",
    destructive: "#EF4444",
    destructiveForeground: "#FAFAFA",
    border: "#E5E5E5",
    input: "#E5E5E5",
    ring: "#A3A3A3",
    success: "#22C55E",
    warning: "#F59E0B",
    info: "#3B82F6",
  };

  return {
    name: "neutral-light",
    mode: "light",
    tokens,
    components: {
      appBackground: tokens.background,
      title: RGBA.fromInts(10, 10, 10, 255),
      border: tokens.border,
      focusedBorder: tokens.ring,
      inputText: tokens.foreground,
      inputFocusedText: tokens.foreground,
      inputPlaceholder: tokens.mutedForeground,
      inputCursor: tokens.primary,
      selectSelectedBackground: tokens.primary,
      selectText: tokens.foreground,
      selectSelectedText: tokens.primaryForeground,
      selectDescription: tokens.mutedForeground,
      selectSelectedDescription: tokens.primaryForeground,
      instructions: tokens.mutedForeground,
      notImplemented: "#CA8A04",
    },
  };
}

// ============================================================================
// Predefined Themes
// ============================================================================

/** Neutral dark theme (shadcn default dark) */
export const neutralDark = createDarkTheme();

/** Neutral light theme (shadcn default light) */
export const neutralLight = createLightTheme();

// ============================================================================
// Theme Collection - usage: APP_THEME.dark.neutral or APP_THEME.light.neutral
// ============================================================================

export const APP_THEME = {
  dark: {
    neutral: neutralDark,
  },
  light: {
    neutral: neutralLight,
  },
} as const;

/** Default theme */
export const DEFAULT_THEME = APP_THEME.dark.neutral;

/** Default theme mode */
export const DEFAULT_THEME_MODE: ThemeMode = "dark";

/** Backward-compatible theme record */
export const MENU_THEMES: Record<ThemeMode, AppTheme> = {
  dark: APP_THEME.dark.neutral,
  light: APP_THEME.light.neutral,
};

/**
 * Get theme by mode
 */
export function getTheme(mode: ThemeMode): AppTheme {
  return mode === "dark" ? APP_THEME.dark.neutral : APP_THEME.light.neutral;
}

/**
 * Get theme tokens for a specific mode
 */
export function getThemeTokens(mode: ThemeMode): ThemeTokens {
  return getTheme(mode).tokens;
}

/**
 * Get component theme for a specific mode
 */
export function getComponentTheme(mode: ThemeMode): ComponentTheme {
  return getTheme(mode).components;
}

/**
 * Create a new theme by extending an existing one
 */
export function createTheme(
  base: AppTheme,
  componentOverrides: Partial<ComponentTheme>,
  tokenOverrides: Partial<ThemeTokens>,
  name: string,
): AppTheme {
  return {
    name,
    mode: base.mode,
    tokens: { ...base.tokens, ...tokenOverrides },
    components: { ...base.components, ...componentOverrides },
  };
}
