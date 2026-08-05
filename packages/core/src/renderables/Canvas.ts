/**
 * Screen — full-terminal layout manager.
 *
 * Creates a full-viewport container with optional header/footer slots and a
 * body that always fills the remaining height. All layout is applied atomically
 * through Box constructor options to avoid the partial-setLayout reset issue.
 */

import { EventEmitter } from "node:events";
import { CliRenderEvents } from "../lib/renderableEvents";
import type { ColorInput } from "../lib/rgba";
import type { CliRenderer } from "../platform/cliRenderer";
import { Box } from "./Box";
import type { BorderSide, BorderStyleKind, BoxOptions } from "./Box";

// ── Option interfaces ──────────────────────────────────────────────────────────

export interface CanvasHeaderOptions {
  id?: string;
  height?: number;
  backgroundColor?: ColorInput;
  border?: boolean | BorderSide[];
  borderStyle?: BorderStyleKind;
  borderColor?: ColorInput;
  title?: string;
  titleAlignment?: "left" | "center" | "right";
  alignItems?: BoxOptions["alignItems"];
  justifyContent?: BoxOptions["justifyContent"];
  padding?: number;
  paddingX?: number;
  paddingY?: number;
  paddingLeft?: number;
  paddingRight?: number;
}

export type CanvasFooterOptions = CanvasHeaderOptions;

export interface CanvasBodyOptions {
  id?: string;
  backgroundColor?: ColorInput;
  flexDirection?: "row" | "column";
  alignItems?: BoxOptions["alignItems"];
  justifyContent?: BoxOptions["justifyContent"];
  overflow?: BoxOptions["overflow"];
  gap?: number;
  padding?: number;
  paddingX?: number;
  paddingY?: number;
}

export interface CanvasOptions {
  id?: string;
  backgroundColor?: ColorInput;
  header?: CanvasHeaderOptions;
  body?: CanvasBodyOptions;
  footer?: CanvasFooterOptions;
}

// ── Events ────────────────────────────────────────────────────────────────────

export interface ScreenResizeEvent {
  width: number;
  height: number;
}

export const ScreenEvents = { RESIZE: "resize" } as const;

// ── Helpers ───────────────────────────────────────────────────────────────────

function buildHeaderBoxOptions(opts: CanvasHeaderOptions): BoxOptions {
  return {
    id: opts.id,
    height: opts.height ?? 3,
    flexGrow: 0,
    flexShrink: 0,
    flexDirection: "row",
    alignItems: opts.alignItems ?? "center",
    justifyContent: opts.justifyContent ?? "flex-start",
    backgroundColor: opts.backgroundColor,
    border: opts.border,
    borderStyle: opts.borderStyle,
    borderColor: opts.borderColor,
    title: opts.title,
    titleAlignment: opts.titleAlignment,
    padding: opts.padding,
    paddingX: opts.paddingX,
    paddingY: opts.paddingY,
    paddingLeft: opts.paddingLeft,
    paddingRight: opts.paddingRight,
  };
}

function buildBodyBoxOptions(opts: CanvasBodyOptions): BoxOptions {
  return {
    id: opts.id,
    flexGrow: 1,
    flexShrink: 1,
    flexDirection: opts.flexDirection ?? "column",
    alignItems: opts.alignItems ?? "stretch",
    justifyContent: opts.justifyContent,
    overflow: opts.overflow,
    gap: opts.gap,
    backgroundColor: opts.backgroundColor,
    padding: opts.padding,
    paddingX: opts.paddingX,
    paddingY: opts.paddingY,
    position: "relative",
  };
}

// ── Screen ────────────────────────────────────────────────────────────────────

export class Screen extends EventEmitter {
  readonly container: Box;
  readonly header: Box | null;
  readonly body: Box;
  readonly footer: Box | null;

  private readonly _renderer: CliRenderer;
  private readonly _resizeHandler: (width: number, height: number) => void;

  constructor(renderer: CliRenderer, options: CanvasOptions = {}) {
    super();
    this._renderer = renderer;

    // Outer container — fills the full terminal viewport.
    this.container = new Box(renderer, {
      id: options.id,
      flexDirection: "column",
      width: "100%",
      height: "100%",
      backgroundColor: options.backgroundColor,
    });

    // Header slot (optional).
    this.header = options.header ? new Box(renderer, buildHeaderBoxOptions(options.header)) : null;

    // Body slot — always present; flexGrow: 1 ensures it fills remaining height.
    this.body = new Box(renderer, buildBodyBoxOptions(options.body ?? {}));

    // Footer slot (optional).
    this.footer = options.footer ? new Box(renderer, buildHeaderBoxOptions(options.footer)) : null;

    // Attach children in document order.
    if (this.header) this.container.add(this.header);
    this.container.add(this.body);
    if (this.footer) this.container.add(this.footer);

    // Mount the container into the renderer root.
    renderer.root.add(this.container);

    // Forward renderer resize events.
    this._resizeHandler = (width: number, height: number) => {
      this.emit(ScreenEvents.RESIZE, {
        width,
        height,
      } satisfies ScreenResizeEvent);
    };
    renderer.on(CliRenderEvents.RESIZE, this._resizeHandler);
  }

  // ── Accessors ────────────────────────────────────────────────────────────────

  get terminalWidth(): number {
    return this._renderer.terminalWidth;
  }

  get terminalHeight(): number {
    return this._renderer.terminalHeight;
  }

  // ── Layout helpers ────────────────────────────────────────────────────────────

  /** Re-apply body layout atomically. Always sets flexGrow: 1, flexShrink: 1. */
  setBodyLayout(opts: CanvasBodyOptions): void {
    this.body.setLayout(buildBodyBoxOptions(opts));
  }

  /** Re-apply header visual options (background / border color) after a theme change. */
  applyHeaderOptions(opts: Pick<CanvasHeaderOptions, "backgroundColor" | "borderColor">): void {
    if (!this.header) return;
    if (opts.backgroundColor !== undefined) this.header.backgroundColor = opts.backgroundColor;
    if (opts.borderColor !== undefined) this.header.borderColor = opts.borderColor;
  }

  /** Re-apply footer visual options (background / border color) after a theme change. */
  applyFooterOptions(opts: Pick<CanvasFooterOptions, "backgroundColor" | "borderColor">): void {
    if (!this.footer) return;
    if (opts.backgroundColor !== undefined) this.footer.backgroundColor = opts.backgroundColor;
    if (opts.borderColor !== undefined) this.footer.borderColor = opts.borderColor;
  }

  // ── Event helpers ─────────────────────────────────────────────────────────────

  onResize(cb: (e: ScreenResizeEvent) => void): this {
    this.on(ScreenEvents.RESIZE, cb);
    return this;
  }

  offResize(cb: (e: ScreenResizeEvent) => void): this {
    this.off(ScreenEvents.RESIZE, cb);
    return this;
  }

  // ── Lifecycle ─────────────────────────────────────────────────────────────────

  destroy(): void {
    this._renderer.off(CliRenderEvents.RESIZE, this._resizeHandler);
    this.removeAllListeners();
    this.container.destroyRecursively();
    this._renderer.root.remove(this.container);
  }
}
