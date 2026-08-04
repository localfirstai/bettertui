/**
 * ScrollBox — a scrollable container widget.
 */

import { RenderableEvents } from "../lib/renderableEvents";
import type { ColorInput } from "../lib/rgba";
import type { CliRenderer } from "../platform/cliRenderer";
import type { RawKeyEvent } from "../platform/cliRenderer";
import { Box, type BoxOptions } from "./Box";

export interface ScrollBarOptions extends BoxOptions {
  orientation?: "vertical" | "horizontal";
  showArrows?: boolean;
  thumbColor?: ColorInput;
  trackColor?: ColorInput;
  trackOptions?: {
    foregroundColor?: ColorInput;
    backgroundColor?: ColorInput;
  };
}

export class ScrollBar extends Box {
  private _orientation: "vertical" | "horizontal";
  private _showArrows: boolean;
  private _scrollPosition = 0;
  private _scrollSize = 0;
  private _viewSize = 0;

  constructor(renderer: CliRenderer, options: ScrollBarOptions = {}) {
    super(renderer, options);
    this._orientation = options.orientation ?? "vertical";
    this._showArrows = options.showArrows !== false;
  }

  get showArrows(): boolean {
    return this._showArrows;
  }

  set showArrows(v: boolean) {
    this._showArrows = v;
  }

  get scrollPosition(): number {
    return this._scrollPosition;
  }

  set scrollPosition(v: number) {
    this._scrollPosition = Math.max(0, v);
  }

  get scrollSize(): number {
    return this._scrollSize;
  }

  set scrollSize(v: number) {
    this._scrollSize = v;
  }

  get viewSize(): number {
    return this._viewSize;
  }

  set viewSize(v: number) {
    this._viewSize = v;
  }
}

export interface ScrollBoxOptions extends BoxOptions {
  rootOptions?: BoxOptions;
  wrapperOptions?: BoxOptions;
  viewportOptions?: BoxOptions;
  contentOptions?: BoxOptions;
  scrollbarOptions?: ScrollBarOptions;
  verticalScrollbarOptions?: ScrollBarOptions;
  horizontalScrollbarOptions?: ScrollBarOptions;
  stickyScroll?: boolean;
  stickyStart?: "bottom" | "top" | "left" | "right";
  scrollX?: boolean;
  scrollY?: boolean;
  viewportCulling?: boolean;
}

let _scrollBoxCounter = 0;

export class ScrollBox extends Box {
  public readonly content: Box;
  public readonly viewport: Box;
  public readonly verticalScrollBar: ScrollBar;
  public readonly horizontalScrollBar: ScrollBar;
  private _scrollTop = 0;
  private _scrollLeft = 0;
  private _stickyScroll: boolean;
  private readonly _keyHandler: (key: RawKeyEvent) => void;

  constructor(renderer: CliRenderer, options: ScrollBoxOptions = {}) {
    _scrollBoxCounter++;
    super(renderer, {
      ...options,
      id: options.id ?? `scrollbox-${_scrollBoxCounter}`,
      overflow: "hidden",
      focusable: true,
    });

    // Create viewport (clip container)
    this.viewport = new Box(renderer, {
      id: `${this._id}-viewport`,
      width: "100%",
      height: "100%",
      overflow: "hidden",
      ...(options.viewportOptions ?? {}),
    });
    super.add(this.viewport);

    // Create content (scrollable inner)
    this.content = new Box(renderer, {
      id: `${this._id}-content`,
      flexDirection: "column",
      ...(options.contentOptions ?? {}),
    });
    this.viewport.add(this.content);

    // Scroll bars
    this.verticalScrollBar = new ScrollBar(renderer, {
      id: `${this._id}-vscroll`,
      orientation: "vertical",
      width: 1,
      visible: options.scrollY !== false,
      ...(options.verticalScrollbarOptions ?? options.scrollbarOptions ?? {}),
    });

    this.horizontalScrollBar = new ScrollBar(renderer, {
      id: `${this._id}-hscroll`,
      orientation: "horizontal",
      height: 1,
      visible: options.scrollX === true,
      ...(options.horizontalScrollbarOptions ?? options.scrollbarOptions ?? {}),
    });

    this._stickyScroll = options.stickyScroll ?? false;
    this._keyHandler = this._handleKey.bind(this);
  }

  get scrollTop(): number {
    return this._scrollTop;
  }

  set scrollTop(v: number) {
    this._scrollTop = Math.max(0, v);
    this._applyScroll();
  }

  get scrollLeft(): number {
    return this._scrollLeft;
  }

  set scrollLeft(v: number) {
    this._scrollLeft = Math.max(0, v);
    this._applyScroll();
  }

  get scrollHeight(): number {
    return this.verticalScrollBar.scrollSize;
  }

  get scrollWidth(): number {
    return this.horizontalScrollBar.scrollSize;
  }

  get stickyScroll(): boolean {
    return this._stickyScroll;
  }

  set stickyScroll(v: boolean) {
    this._stickyScroll = v;
  }

  // Delegate add/remove to content
  override add(child: Box, index?: number): void {
    this.content.add(child, index);
  }

  override remove(child: Box): void {
    this.content.remove(child);
  }

  override getRenderable(id: string): Box | undefined {
    if (this._id === id) return this;
    return this.viewport.getRenderable(id) ?? super.getRenderable(id);
  }

  override focus(): void {
    if (this._isDestroyed || this._focused) return;
    this._focused = true;
    this.emit(RenderableEvents.FOCUSED, this);
    this._renderer.keyInput.off("keypress", this._keyHandler);
    this._renderer.keyInput.on("keypress", this._keyHandler);
  }

  override blur(): void {
    if (this._isDestroyed) return;
    this._renderer.keyInput.off("keypress", this._keyHandler);
    if (!this._focused) return;
    this._focused = false;
    this.emit(RenderableEvents.BLURRED, this);
  }

  scrollBy(delta: number, axis: "x" | "y" = "y"): void {
    if (axis === "y") {
      this.scrollTop += delta;
    } else {
      this.scrollLeft += delta;
    }
  }

  private _applyScroll(): void {
    // Update content position via margin/inset
    this._renderer.setNodeLayout(this.content.nodeId, {
      marginTop: -this._scrollTop,
      marginLeft: -this._scrollLeft,
    });
  }

  private _handleKey(key: RawKeyEvent): void {
    if (!this._focused || this._isDestroyed) return;

    if (key.name === "up" || (key.ctrl && key.name === "p")) {
      this.scrollBy(-1);
    } else if (key.name === "down" || (key.ctrl && key.name === "n")) {
      this.scrollBy(1);
    } else if (key.name === "pageup") {
      this.scrollBy(-10);
    } else if (key.name === "pagedown") {
      this.scrollBy(10);
    } else if (key.name === "home" || (key.ctrl && key.name === "home")) {
      this.scrollTop = 0;
    } else if (key.name === "end" || (key.ctrl && key.name === "end")) {
      // Scroll to bottom
      this.scrollTop = 9999;
    }
  }

  override destroy(): void {
    if (this._isDestroyed) return;
    this._renderer.keyInput.off("keypress", this._keyHandler);
    this.viewport.destroyRecursively();
    this.verticalScrollBar.destroy();
    this.horizontalScrollBar.destroy();
    super.destroy();
  }
}
