import type { KeyEvent } from "@bettertui/core";
import { useEffect } from "react";
import { useEffectEvent } from "./useEvent";
import { useRuntime } from "./useRuntime";

export interface UseKeyboardOptions {
  /** Also fire the handler on key-release events (default: false). */
  release?: boolean;
}

/**
 * Subscribe to keyboard events in the current terminal.
 *
 * ```ts
 * useKeyboard((key) => {
 *   if (key.name === "q") process.exit(0);
 * });
 * ```
 */
export function useKeyboard(
  handler: (key: KeyEvent) => void,
  options: UseKeyboardOptions = {},
): void {
  const renderer = useRuntime();
  const stableHandler = useEffectEvent(handler);

  // biome-ignore lint/correctness/useExhaustiveDependencies: stableHandler has stable identity
  useEffect(() => {
    const keyInput = renderer.keyInput;
    keyInput.on("keypress", stableHandler as (e: KeyEvent) => void);
    return () => {
      keyInput.off("keypress", stableHandler as (e: KeyEvent) => void);
    };
  }, [renderer, options.release]);
}
