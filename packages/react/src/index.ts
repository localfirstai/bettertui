// ── Renderer entry point ──────────────────────────────────────────────────────
export { createRoot } from "./reconciler/renderer";
export type { Root } from "./reconciler/renderer";

// ── Runtime context ───────────────────────────────────────────────────────────
export { RuntimeContext, useRuntimeContext } from "./context/runtimeContext";
export type { RuntimeContextValue } from "./context/runtimeContext";

// ── Hooks ─────────────────────────────────────────────────────────────────────
export {
  useEffectEvent,
  useFocus,
  useKeyboard,
  useRuntime,
  useTerminalDimensions,
  useTheme,
  useTimeline,
} from "./hooks/index";
export type { TerminalDimensions, ThemeMode, UseKeyboardOptions } from "./hooks/index";

// ── DevTools ──────────────────────────────────────────────────────────────────
export { initReactDevTools } from "./reconciler/devtools";
export type {
  BaseProps,
  BoxProps,
  BetterTUIElementType,
  InputProps,
  ScrollBoxProps,
  TextProps,
} from "./types/jsx.types";
