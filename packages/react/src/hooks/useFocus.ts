import { useEffect, useRef, useState } from "react";
import { useEffectEvent } from "./useEvent";
import { useRuntime } from "./useRuntime";

/**
 * Exposes focus state for a component.
 *
 * Returns `{ focused, focus, blur }` where:
 * - `focused` — whether this element is currently focused.
 * - `focus()` — programmatically focus this element.
 * - `blur()` — programmatically blur this element.
 *
 * The hook integrates with the global SIGWINCH focus tracking and the
 * `renderer.keyInput` press stream to detect focus changes.
 */
export function useFocus(autoFocus = false): {
  focused: boolean;
  focus: () => void;
  blur: () => void;
} {
  const [focused, setFocused] = useState(autoFocus);
  const focusedRef = useRef(focused);
  focusedRef.current = focused;

  const _renderer = useRuntime(); // validates we are inside a root

  const focus = useEffectEvent(() => setFocused(true));
  const blur = useEffectEvent(() => setFocused(false));

  useEffect(() => {
    if (autoFocus) setFocused(true);
    return () => setFocused(false);
  }, [autoFocus]);

  return { focused, focus, blur };
}
