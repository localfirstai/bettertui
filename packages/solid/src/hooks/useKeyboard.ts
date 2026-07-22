/**
 * useKeyboard — subscribe to terminal key events.
 *
 * Follows SolidJS lifecycle: registers on mount, cleans up on cleanup.
 * The handler is called for every keypress; pass `options.release = true`
 * to also receive key-release events (kitty keyboard protocol required).
 */

import type { RawKeyEvent } from "@bettertui/core";
import { createEffect, onCleanup } from "solid-js";
import { useRenderer } from "../context/rendererContext";

export interface UseKeyboardOptions {
  /** When true, also subscribe to key-release events. Default false. */
  release?: boolean;
}

export function useKeyboard(
  handler: (event: RawKeyEvent) => void,
  options: UseKeyboardOptions = {},
): void {
  const renderer = useRenderer();
  const keyInput = renderer.keyInput;

  createEffect(() => {
    keyInput.on("keypress", handler);
    if (options.release) keyInput.on("keyrelease", handler);

    onCleanup(() => {
      keyInput.off("keypress", handler);
      if (options.release) keyInput.off("keyrelease", handler);
    });
  });
}
