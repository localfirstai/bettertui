import type { DevTools } from "../index";

/**
 * Identifiers for the built-in debug panels.
 *
 * Panels map to the six diagnostics surfaces from the design proposal (§6.2):
 * performance, tree, layout, events, dirty-regions, and render statistics.
 * Panels 1 and 6 (performance + render statistics) share a single rendered
 * panel; both live under {@link DebugPanel.Performance}.
 */
export enum DebugPanel {
  /** Panel 1 + 6 — FPS, frame timing, render calls, bytes, cache, memory. */
  Performance = "performance",
  /** Panel 2 — node tree viewer (display-only in the all-TS pass). */
  Tree = "tree",
  /** Panel 3 — layout inspector (box model, flex, computed dims). */
  Layout = "layout",
  /** Panel 4 — event tracer (key/mouse/focus/resize log). */
  Events = "events",
  /** Panel 5 — dirty-region visualizer (stats-level in the all-TS pass). */
  DirtyRegions = "dirtyRegions",
}

/**
 * Context handed to a {@link Panel} each frame. Panels are pure renderers:
 * they read from the DevTools facade and diagnostics and return lines.
 */
export interface PanelContext {
  /** The live DevTools facade (inspectors + queries). */
  readonly devtools: DevTools;
  /** Engine diagnostics snapshot for the current frame. */
  readonly diagnostics: {
    renderCalls: number;
    renderBytes: number;
    eventDispatches: number;
    layoutComputations: number;
    cacheHits: number;
    cacheMisses: number;
    allocations: number;
    averageFrameTime: number;
    fps: number;
  };
  /** Dirty-region count reported by the last engine frame. */
  readonly dirtyRegionCount: number;
  /** Maximum width a panel may occupy (columns). */
  readonly maxWidth: number;
  /** Maximum height a panel may occupy (rows). */
  readonly maxHeight: number;
}

/**
 * A debug panel: a pure function from state to lines. The {@link OverlayHost}
 * positions the returned lines; a panel never writes to stdout itself.
 */
export interface Panel {
  /** Which panel this renders. */
  readonly id: DebugPanel;
  /** Title shown in the panel's header. */
  readonly title: string;
  /** Produce the panel body as an array of plain (unpositioned) lines. */
  render(ctx: PanelContext): string[];
}
