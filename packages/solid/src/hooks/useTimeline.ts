/**
 * useTimeline — memoised NativeTimeline widget with automatic lifecycle.
 *
 * Creates a `Timeline` widget on first call, starts it if `autoPlay` is set
 * (default true), and pauses it when the owning component is unmounted.
 * The Timeline instance is stable — it is NOT recreated on re-renders.
 */

import { Timeline } from "@bettertui/core";
import type { TimelineOptions } from "@bettertui/shared";
import { onCleanup, onMount } from "solid-js";

export function useTimeline(options: TimelineOptions = {}): Timeline {
  const timeline = new Timeline(options);

  onMount(() => {
    if (options.autoPlay !== false) {
      timeline.play();
    }
  });

  onCleanup(() => {
    timeline.pause();
  });

  return timeline;
}
