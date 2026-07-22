/**
 * useFocus — local focus state for a terminal UI element.
 *
 * Returns a reactive `isFocused` signal accessor and `focus`/`blur` functions.
 * If `autoFocus` is true the element focuses immediately on mount.
 *
 * Note: naming follows OpenTUI Solid convention — use `useFocus` (not
 * `onFocus`) for state-bearing focus, while `onFocus`/`onBlur` are separate
 * event-only hooks in the OpenTUI pattern.
 */

import { createSignal, onCleanup, onMount } from "solid-js";

export interface UseFocusResult {
  /** Reactive accessor — true when this element is focused. */
  isFocused: () => boolean;
  /** Request focus on this element. */
  focus: () => void;
  /** Relinquish focus from this element. */
  blur: () => void;
}

export function useFocus(autoFocus = false): UseFocusResult {
  const [isFocused, setIsFocused] = createSignal(false);

  const focus = () => setIsFocused(true);
  const blur = () => setIsFocused(false);

  onMount(() => {
    if (autoFocus) setIsFocused(true);
  });

  onCleanup(() => {
    setIsFocused(false);
  });

  return { isFocused, focus, blur };
}
