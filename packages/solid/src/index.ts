/**
 * @bettertui/solid
 *
 * SolidJS renderer adapter for BetterTUI.
 *
 * Implements SolidJS's universal renderer via `createRenderer` so that Solid
 * reactive primitives (`createSignal`, `createEffect`, `For`, `Show`, etc.)
 * drive the BetterTUI native engine directly — without React.
 *
 * @example
 * ```ts
 * import { createRoot } from "@bettertui/solid"
 * import { CliRenderer } from "@bettertui/core"
 * import { createSignal } from "solid-js"
 *
 * const renderer = new CliRenderer()
 * const root = createRoot(renderer)
 *
 * renderer.start()
 * root.render(() => <box fg="cyan">Hello from Solid!</box>)
 * ```
 */

// ── Renderer ──────────────────────────────────────────────────────────────────

export { createRoot, render } from "./renderer/createRoot";
export type { Root } from "./renderer/createRoot";

// ── Context ───────────────────────────────────────────────────────────────────

export { RendererContext, useRenderer } from "./context/rendererContext";

// ── Hooks ─────────────────────────────────────────────────────────────────────

export {
  useFocus,
  useKeyboard,
  useTerminalDimensions,
  useTheme,
  useTimeline,
} from "./hooks/index";
export type {
  UseFocusResult,
  UseKeyboardOptions,
  TerminalDimensions,
  ThemeMode,
} from "./hooks/index";

// ── JSX types ─────────────────────────────────────────────────────────────────

export type { BetterTUIElementType } from "./types/jsx.types";
