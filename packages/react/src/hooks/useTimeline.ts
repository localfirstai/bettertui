import { Timeline } from "@bettertui/core";
import type { TimelineOptions } from "@bettertui/shared";
import { useEffect, useMemo } from "react";
import { useRuntime } from "./useRuntime";

/**
 * Creates and manages a {@link Timeline} animation.
 * The timeline is automatically paused and destroyed on unmount.
 *
 * ```ts
 * const tl = useTimeline({ duration: 1000 });
 *
 * useEffect(() => {
 *   tl.addTween({ from: 0, to: 100, duration: 500 });
 *   tl.play();
 * }, [tl]);
 * ```
 */
export function useTimeline(options: TimelineOptions = {}): Timeline {
  const _renderer = useRuntime(); // validates we are inside a root

  // biome-ignore lint/correctness/useExhaustiveDependencies: timeline stable ref
  const timeline = useMemo(
    () => new Timeline((options as { duration?: number }).duration ?? 1, options),
    [],
  );

  useEffect(() => {
    return () => {
      timeline.pause();
    };
  }, [timeline]);

  return timeline;
}
