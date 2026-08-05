import type { ExampleCategory } from "../types/exampleList.types";
import {
  APP_THEME,
  type AppTheme,
  DEFAULT_THEME,
  DEFAULT_THEME_MODE,
  MENU_THEMES,
  type ThemeMode,
} from "./theme";

export { APP_THEME, DEFAULT_THEME, DEFAULT_THEME_MODE, MENU_THEMES };
export { getTheme, getComponentTheme, getThemeTokens } from "./theme";

export const MENU_TERMINAL_TITLE = "BetterTUI Examples";
export const EXAMPLES_BOX_TITLE = "Examples";
export const EXAMPLES_INDENT = "  ";

export const CATEGORY_LABELS: Record<ExampleCategory, string> = {
  "Layout & Composition": "Layout & Composition",
  "Input & Editing": "Input & Editing",
  "Scroll & Navigation": "Scroll & Navigation",
  "Text & Documents": "Text & Documents",
  "Rendering & Effects": "Rendering & Effects",
  "Runtime & Tooling": "Runtime & Tooling",
  "Terminal & Native": "Terminal & Native",
};

/**
 * Get menu theme for a specific mode
 * @deprecated Use getComponentTheme(mode) for new code
 */
export function getMenuTheme(mode: ThemeMode): AppTheme {
  return MENU_THEMES[mode];
}
