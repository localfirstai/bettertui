/**
 * Saha theme system following shadcn/ui semantic token conventions.
 *
 * Provides two handcrafted themes — `saha-light` and `saha-dark` — built on
 * the Saha brand's signature green palette (`#27AD60` / `#007B5A`).
 *
 * Token naming follows the shadcn/ui CSS-variable contract:
 *   background · foreground · primary · secondary · muted · accent ·
 *   destructive · border · input · ring · success · warning · info
 *
 * @see https://ui.shadcn.com/docs/theming
 */

import { RGBA } from "@bettertui/core";

/** Theme mode type */
export type ThemeMode = "dark" | "light";

/**
 * Component theme interface - semantic colors without suffixes.
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
 * Semantic color tokens following shadcn/ui naming conventions.
 *
 * Every token mirrors the shadcn/ui CSS variable surface:
 * `--background`, `--foreground`, `--primary`, etc.
 */
export interface ThemeTokens {
  /** Page / root background */
  background: string;
  /** Default body text */
  foreground: string;

  /**
   * Brand primary — Saha green `#27AD60`.
   * Used for CTAs, active states, and focus indicators.
   */
  primary: string;
  /** High-contrast text rendered on a `primary` surface */
  primaryForeground: string;

  /** Subtle surface one step above background */
  secondary: string;
  /** Default text on `secondary` surface */
  secondaryForeground: string;

  /** Lowest-contrast surface for placeholders and dividers */
  muted: string;
  /** De-emphasised text rendered on any background */
  mutedForeground: string;

  /**
   * Accent tint — lightest green (`#E6FFEB` light / deep teal dark).
   * Highlights selected rows, hover states, and badges.
   */
  accent: string;
  /** Text rendered on an `accent` surface */
  accentForeground: string;

  /** Destructive / error background */
  destructive: string;
  /** Text rendered on a `destructive` surface */
  destructiveForeground: string;

  /** Default border and rule colour */
  border: string;
  /** Input field background */
  input: string;
  /** Keyboard focus ring — brand green in both modes */
  ring: string;

  /** Semantic success state */
  success: string;
  /** Semantic warning state */
  warning: string;
  /** Semantic informational state */
  info: string;
}

/**
 * Complete theme configuration bundling tokens and resolved component values.
 */
export interface AppTheme {
  name: string;
  mode: ThemeMode;
  tokens: ThemeTokens;
  components: ComponentTheme;
}

// ============================================================================
// Saha Dark Theme — "Midnight Ink"
//
// Design rationale (color theory + human psychology)
// ─────────────────────────────────────────────────
// • Background is near-neutral cool-dark (#0D1117), NOT green-tinted.
//   Saturated hues in large background areas cause chromatic fatigue — the
//   green-sensitive cones exhaust faster than the others. A cool neutral
//   base (slight blue cast) is psychologically associated with focus and
//   calm without triggering color fatigue.
//
// • Brand green (#27AD60) is reserved for the 10% interactive layer:
//   CTAs, cursors, focus rings, selected text. This is the 60-30-10 rule
//   applied — green at 10% feels energetic; at 60% it becomes oppressive.
//
// • Three-level elevation model gives the UI a readable depth axis:
//     L0 background (#0D1117) → L1 secondary (#161B22) → L2 muted (#1C2128)
//   Humans perceive lighter = closer. This depth cue reduces cognitive load.
//
// • Accent (#0D2B1E) is a very dark forest green used only as a selection
//   background. Brand green text on it creates a "green spotlight" effect
//   psychologically signalling "this item is active."
//
// • Warning is muted amber (#D29922) rather than loud yellow — advisory
//   tone without triggering a fight-or-flight alarm response.
//
// • Destructive is warm bright red (#F85149) — the warm hue creates
//   temperature contrast against the cool base, demanding attention.
//
// Contrast ratios (WCAG)
//   foreground / background  : ~16 : 1  (AAA)
//   primary / background     :  ~6.6 : 1  (AAA)
//   accentFg / accent bg     :  ~5.6 : 1  (AA)
//   mutedForeground / bg     :  ~5.2 : 1  (AA)
// ============================================================================
function createSahaDarkTheme(): AppTheme {
  const tokens: ThemeTokens = {
    // Midnight cool-neutral — no green tint, promotes focus
    background: "#0D1117",
    // Cool moonlight off-white — avoids halation vs pure white
    foreground: "#E6EDF3",

    // Saha brand green — pops vividly against neutral dark
    primary: "#27AD60",
    primaryForeground: "#0D1117",

    // Elevation layer 1 — cards, panels, sidebars
    secondary: "#161B22",
    secondaryForeground: "#E6EDF3",

    // Elevation layer 2 — subtle raised areas, dividers
    muted: "#1C2128",
    // Cool-shifted gray — quiet, doesn't compete with green accents
    mutedForeground: "#848D97",

    // Dark forest green — used only as selected-row background
    accent: "#0D2B1E",
    // Brand green text on dark green bg — "green spotlight" effect
    accentForeground: "#27AD60",

    // Warm alert red — temperature contrast against cool base = urgency
    destructive: "#F85149",
    destructiveForeground: "#FFFFFF",

    // Structural border — visible without aggression
    border: "#30363D",
    // Input bg same as secondary — raised, clearly interactive
    input: "#161B22",
    // Brand green ring — "you are here" signal
    ring: "#27AD60",

    // Slightly brighter than primary — brightness spike = reward signal
    success: "#3FB950",
    // Muted amber — advisory caution, not alarming
    warning: "#D29922",
    // Calm blue — informational, trustworthy, analogous to cool base
    info: "#58A6FF",
  };

  return {
    name: "saha-dark",
    mode: "dark",
    tokens,
    components: {
      appBackground: tokens.background,
      title: RGBA.fromInts(230, 237, 243, 255),
      border: tokens.border,
      focusedBorder: tokens.ring,
      inputText: tokens.foreground,
      inputFocusedText: tokens.foreground,
      inputPlaceholder: tokens.mutedForeground,
      inputCursor: tokens.primary,
      // Dark forest green row bg — selected item glows green
      selectSelectedBackground: tokens.accent,
      selectText: tokens.foreground,
      // Brand green on dark green — clear activation signal
      selectSelectedText: tokens.accentForeground,
      selectDescription: tokens.mutedForeground,
      // Muted gray — clearly subordinate to the brand-green title on the dark-green bg
      selectSelectedDescription: tokens.mutedForeground,
      instructions: tokens.mutedForeground,
      notImplemented: "#D29922",
    },
  };
}

// ============================================================================
// Saha Light Theme
//
// Palette roots
//   Brand green   : #27AD60  (primary / ring / success / focusedBorder)
//   Accent green  : #178951  (accentForeground — deeper on white)
//   Lightest green: #E6FFEB  (accent — tint for selected rows)
//   Background    : #FFFFFF  (pure white)
//   Foreground    : #293142  (Saha dark-navy — login input text)
//   Secondary bg  : #F4F2FB  (Saha secondary — soft lavender-tinted surface)
//   Muted bg      : #F5F5F5  (Saha paper / input background)
//   Muted text    : #6B7280  (Tailwind gray-500)
//   Border        : #D4D4D4  (Saha borderColor.light)
//   Input bg      : #EEEFF0  (Saha searchBar.background)
//   Destructive   : #EF4444  (shadcn standard red)
//   Warning       : #C99D00  (Saha statusWarning.primary)
//   Info          : #3B82F6  (Tailwind blue-500)
// ============================================================================
function createSahaLightTheme(): AppTheme {
  const tokens: ThemeTokens = {
    background: "#FFFFFF",
    foreground: "#293142",

    primary: "#27AD60",
    primaryForeground: "#FFFFFF",

    secondary: "#F4F2FB",
    secondaryForeground: "#293142",

    muted: "#F5F5F5",
    mutedForeground: "#6B7280",

    accent: "#E6FFEB",
    accentForeground: "#178951",

    destructive: "#EF4444",
    destructiveForeground: "#FFFFFF",

    border: "#D4D4D4",
    input: "#EEEFF0",
    ring: "#27AD60",

    success: "#27AE60",
    warning: "#C99D00",
    info: "#3B82F6",
  };

  return {
    name: "saha-light",
    mode: "light",
    tokens,
    components: {
      appBackground: tokens.background,
      title: RGBA.fromInts(41, 49, 66, 255),
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
      // Dark forest green — subordinate to the white title on the green selection bg
      selectSelectedDescription: "#0D2B1E",
      instructions: tokens.mutedForeground,
      notImplemented: "#C99D00",
    },
  };
}

// ============================================================================
// Predefined Themes
// ============================================================================

/** Saha dark theme — brand green on deep green-tinted dark surfaces */
export const sahaDark = createSahaDarkTheme();

/** Saha light theme — brand green on clean white and lavender-tinted surfaces */
export const sahaLight = createSahaLightTheme();

// ============================================================================
// Theme Collection - usage: APP_THEME.dark.saha or APP_THEME.light.saha
// ============================================================================

export const APP_THEME = {
  dark: {
    saha: sahaDark,
  },
  light: {
    saha: sahaLight,
  },
} as const;

/** Default theme */
export const DEFAULT_THEME = APP_THEME.dark.saha;

/** Default theme mode */
export const DEFAULT_THEME_MODE: ThemeMode = "dark";

/** Backward-compatible theme record */
export const MENU_THEMES: Record<ThemeMode, AppTheme> = {
  dark: APP_THEME.dark.saha,
  light: APP_THEME.light.saha,
};

/**
 * Get theme by mode.
 */
export function getTheme(mode: ThemeMode): AppTheme {
  return mode === "dark" ? APP_THEME.dark.saha : APP_THEME.light.saha;
}

/**
 * Get theme tokens for a specific mode.
 */
export function getThemeTokens(mode: ThemeMode): ThemeTokens {
  return getTheme(mode).tokens;
}

/**
 * Get component theme for a specific mode.
 */
export function getComponentTheme(mode: ThemeMode): ComponentTheme {
  return getTheme(mode).components;
}

/**
 * Create a new theme by extending an existing one.
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
