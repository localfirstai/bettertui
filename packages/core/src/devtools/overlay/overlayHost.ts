import type { DiagnosticSnapshot } from "../../platform/logger";
import type { DevTools } from "../index";
import { RESET, RESTORE_CURSOR, SAVE_CURSOR, displayWidth, drawBox, moveTo } from "./ansiUtils";
import { DebugPanel, type Panel, type PanelContext } from "./panel.types";
import { dirtyRegionsPanel } from "./panels/dirtyRegionsPanel";
import { eventsPanel } from "./panels/eventsPanel";
import { layoutPanel } from "./panels/layoutPanel";
import { performancePanel } from "./panels/performancePanel";
import { treePanel } from "./panels/treePanel";

/** Corner the overlay is anchored to. */
export type OverlayCorner = "top-right" | "top-left" | "bottom-right" | "bottom-left";

/** The minimal renderer surface the overlay reads from. */
export interface OverlayRenderer {
  readonly terminalWidth: number;
  readonly viewportHeight: number;
  getDiagnostics(): DiagnosticSnapshot;
  write(text: string): void;
}

export interface OverlayHostOptions {
  /** Corner to anchor panels to. Defaults to top-right. */
  corner?: OverlayCorner;
  /** Inner width of each panel in columns. Defaults to 28. */
  panelWidth?: number;
  /** Max rows a scrolling panel (events/tree) may show. Defaults to 8. */
  panelBodyRows?: number;
}

const PANEL_ORDER: DebugPanel[] = [
  DebugPanel.Performance,
  DebugPanel.Tree,
  DebugPanel.Layout,
  DebugPanel.Events,
  DebugPanel.DirtyRegions,
];

const PANELS: Record<DebugPanel, Panel> = {
  [DebugPanel.Performance]: performancePanel,
  [DebugPanel.Tree]: treePanel,
  [DebugPanel.Layout]: layoutPanel,
  [DebugPanel.Events]: eventsPanel,
  [DebugPanel.DirtyRegions]: dirtyRegionsPanel,
};

/**
 * Owns per-frame ANSI compositing of the debug overlay.
 *
 * The overlay is written *over* the engine's incremental output and is not part
 * of the engine's dirty-diff, so the host tracks the rect it painted last frame
 * and clears any rows that are no longer covered — preventing trails when a
 * panel shrinks, moves, or is toggled off.
 */
export class OverlayHost {
  private readonly renderer: OverlayRenderer;
  private readonly devtools: DevTools;
  private corner: OverlayCorner;
  private panelWidth: number;
  private panelBodyRows: number;
  /** Rows (1-based) painted on the previous frame, and their painted width. */
  private previousRows: Map<number, number> = new Map();
  private lastDirtyRegionCount = 0;

  constructor(renderer: OverlayRenderer, devtools: DevTools, options: OverlayHostOptions = {}) {
    this.renderer = renderer;
    this.devtools = devtools;
    this.corner = options.corner ?? "top-right";
    this.panelWidth = options.panelWidth ?? 28;
    this.panelBodyRows = options.panelBodyRows ?? 8;
  }

  /** Whether any panel is currently visible. */
  get visible(): boolean {
    return this.devtools.visiblePanels.size > 0;
  }

  configure(options: OverlayHostOptions): void {
    if (options.corner !== undefined) this.corner = options.corner;
    if (options.panelWidth !== undefined) this.panelWidth = options.panelWidth;
    if (options.panelBodyRows !== undefined) this.panelBodyRows = options.panelBodyRows;
  }

  /** Record the engine's reported dirty-region count for the current frame. */
  setDirtyRegionCount(count: number): void {
    this.lastDirtyRegionCount = count;
  }

  /**
   * Composite the visible panels over the current frame. Saves the cursor,
   * clears rows vacated since the last paint, draws each visible panel, then
   * restores the cursor. A no-op when nothing is visible (but still clears any
   * previously-painted rows exactly once).
   */
  paint(): void {
    const width = this.renderer.terminalWidth;
    const height = this.renderer.viewportHeight;
    const innerWidth = Math.max(4, Math.min(this.panelWidth, width - 2));

    const framed = this.buildFrame(innerWidth, height);

    // Nothing visible: clear leftovers from the previous paint and stop.
    if (framed.length === 0) {
      if (this.previousRows.size > 0) {
        this.renderer.write(this.clearPreviousOnly());
        this.previousRows.clear();
      }
      return;
    }

    const boxWidth = innerWidth + 2;
    const startCol = this.startColumn(width, boxWidth);
    const startRow = this.startRow(height, framed.length);

    let out = SAVE_CURSOR;

    const nextRows = new Map<number, number>();
    for (let i = 0; i < framed.length; i++) {
      const row = startRow + i;
      if (row < 1 || row > height) continue;
      out += moveTo(row, startCol) + RESET + framed[i];
      nextRows.set(row, boxWidth);
    }

    // Clear rows painted last frame that we are not painting now.
    for (const [row, prevW] of this.previousRows) {
      if (!nextRows.has(row) && row >= 1 && row <= height) {
        out += moveTo(row, this.startColumn(width, prevW)) + " ".repeat(prevW);
      }
    }

    out += RESTORE_CURSOR + RESET;
    this.renderer.write(out);
    this.previousRows = nextRows;
  }

  /**
   * Force-clear the whole overlay region (used on toggle-off before a full
   * redraw). Returns nothing; writes directly.
   */
  clear(): void {
    if (this.previousRows.size === 0) return;
    this.renderer.write(this.clearPreviousOnly());
    this.previousRows.clear();
  }

  private clearPreviousOnly(): string {
    const width = this.renderer.terminalWidth;
    let out = SAVE_CURSOR;
    for (const [row, prevW] of this.previousRows) {
      out += moveTo(row, this.startColumn(width, prevW)) + " ".repeat(prevW);
    }
    out += RESTORE_CURSOR + RESET;
    return out;
  }

  /** Build the framed lines for all visible panels, stacked vertically. */
  private buildFrame(innerWidth: number, maxHeight: number): string[] {
    const diagnostics = this.renderer.getDiagnostics();
    const ctx: PanelContext = {
      devtools: this.devtools,
      diagnostics,
      dirtyRegionCount: this.lastDirtyRegionCount,
      maxWidth: innerWidth,
      maxHeight: this.panelBodyRows,
    };

    const out: string[] = [];
    for (const id of PANEL_ORDER) {
      if (!this.devtools.visiblePanels.has(id)) continue;
      const panel = PANELS[id];
      const body = panel.render(ctx);
      const framed = drawBox(body, {
        title: panel.title,
        width: innerWidth,
        borderSgr: [90],
      });
      for (const line of framed) {
        if (out.length >= maxHeight) return out;
        out.push(line);
      }
    }
    return out;
  }

  private startColumn(width: number, boxWidth: number): number {
    if (this.corner === "top-left" || this.corner === "bottom-left") return 1;
    return Math.max(1, width - boxWidth + 1);
  }

  private startRow(height: number, frameHeight: number): number {
    if (this.corner === "bottom-left" || this.corner === "bottom-right") {
      return Math.max(1, height - frameHeight + 1);
    }
    return 1;
  }
}

/** Exposed for tests: the visible width of a framed line. */
export function frameLineWidth(line: string): number {
  return displayWidth(line);
}
