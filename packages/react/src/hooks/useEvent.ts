import { useCallback, useLayoutEffect, useRef } from "react";

/**
 * Returns a stable function reference that always delegates to the latest
 * version of `handler`. Prevents stale-closure bugs in event subscriptions
 * without listing the handler in `useEffect` dependency arrays.
 *
 * Pattern modelled after React's experimental `useEffectEvent`.
 */
export function useEffectEvent<T extends (...args: Parameters<T>) => ReturnType<T>>(handler: T): T {
  const ref = useRef<T>(handler);

  useLayoutEffect(() => {
    ref.current = handler;
  });

  return useCallback((...args: Parameters<T>): ReturnType<T> => {
    return ref.current(...args);
  }, []) as T;
}
