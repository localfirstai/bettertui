// ─── Timeline ────────────────────────────────────────────────────────────────
// TimelineOptions and TweenConfig are shared because @bettertui/react and
// @bettertui/solid both expose a useTimeline hook typed against these.

export interface TimelineOptions {
  duration?: number;
  looping?: boolean;
  autoPlay?: boolean;
  onComplete?: () => void;
}

export interface TweenConfig {
  from: number;
  to: number;
  duration: number;
  startTime?: number;
  easing?: string;
}
