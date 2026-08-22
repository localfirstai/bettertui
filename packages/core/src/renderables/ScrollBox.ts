/**
 * ScrollBox — a scrollable container widget with a proportional scrollbar.
 *
 * Internal layout (flexDirection: "row"):
 *   ├── viewport  (flexGrow: 1, overflow: "hidden")
 *   │   └── content  (flexDirection: "column")
 *   └── verticalScrollBar  (width: 1)
 *       ├── topSpacer   (flexGrow: scrollTop)
 *       ├── thumb       (flexGrow: viewLines, min-height: 1)
 *       └── bottomSpacer (flexGrow: maxScroll - scrollTop)
 *
 * The three-section flex approach means the thumb size and position are
 * always proportional to the content/viewport ratio without needing
 * the absolute track height.
 */

import type { KeyEvent } from "../lib/keyHandler";
import { RenderableEvents } from "../lib/renderableEvents";
import type { ColorInput } from "../lib/rgba";
import type { CliRenderer } from "../platform/cliRenderer";
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

  private readonly _topSpacer: Box;
  private readonly _thumb: Box;
  private readonly _bottomSpacer: Box;

  constructor(renderer: CliRenderer, options: ScrollBarOptions = {}) {
    const trackColor = options.trackColor ?? options.backgroundColor ?? "#1e2030";
    const thumbColor = options.thumbColor ?? "#565f89";

    super(renderer, {
      ...options,
      backgroundColor: trackColor,
      flexDirection: options.orientation === "horizontal" ? "row" : "column",
    });

    this._orientation = options.orientation ?? "vertical";
    this._showArrows = options.showArrows !== false;

    this._topSpacer = new Box(renderer, {
      id: `${this._id}-top`,
      flexGrow: 0,
      flexBasis: 0,
      flexShrink: 0,
    });
    this.add(this._topSpacer);

    this._thumb = new Box(renderer, {
      id: `${this._id}-thumb`,
      flexGrow: 1,
      flexBasis: 0,
      flexShrink: 0,
      minHeight: 1,
      minWidth: 1,
      backgroundColor: thumbColor,
    });
    this.add(this._thumb);

    this._bottomSpacer = new Box(renderer, {
      id: `${this._id}-bottom`,
      flexGrow: 0,
      flexBasis: 0,
      flexShrink: 0,
    });
    this.add(this._bottomSpacer);
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

  /**
   * Update thumb position and size using proportional flex-grow weights.
   *
   * Total flex weight always equals `totalLines`, so the thumb proportion
   * (viewLines / totalLines) is constant regardless of scroll position.
   */
  updateScrollBar(scrollTop: number, totalLines: number, viewLines: number): void {
    if (totalLines <= viewLines || totalLines <= 0) {
      this._topSpacer.flexGrow = 0;
      this._thumb.flexGrow = 1;
      this._bottomSpacer.flexGrow = 0;
      return;
    }
    const maxScroll = totalLines - viewLines;
    const clamped = Math.max(0, Math.min(scrollTop, maxScroll));
    const below = Math.max(0, maxScroll - clamped);
    this._topSpacer.flexGrow = clamped;
    this._thumb.flexGrow = viewLines;
    this._bottomSpacer.flexGrow = below;
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
  private _contentLines = 0;

  private _lastScrollTop = -1;
  private _lastContentLines = -1;
  private _lastViewLines = -1;

  private readonly _keyHandler: (key: KeyEvent) => void;
  private readonly _lifecyclePass: () => void;

  constructor(renderer: CliRenderer, options: ScrollBoxOptions = {}) {
    _scrollBoxCounter++;
    super(renderer, {
      ...options,
      id: options.id ?? `scrollbox-${_scrollBoxCounter}`,
      overflow: "hidden",
      focusable: true,
      flexDirection: "row",
    });

    this.viewport = new Box(renderer, {
      id: `${this._id}-viewport`,
      flexGrow: 1,
      flexShrink: 1,
      flexBasis: 0,
      minWidth: 0,
      minHeight: 0,
      overflow: "hidden",
      ...(options.viewportOptions ?? {}),
    });
    super.add(this.viewport);

    this.content = new Box(renderer, {
      id: `${this._id}-content`,
      flexDirection: "column",
      width: "100%",
      ...(options.contentOptions ?? {}),
    });
    this.viewport.add(this.content);

    this.verticalScrollBar = new ScrollBar(renderer, {
      id: `${this._id}-vscroll`,
      orientation: "vertical",
      width: 1,
      flexShrink: 0,
      visible: options.scrollY !== false,
      ...(options.verticalScrollbarOptions ?? options.scrollbarOptions ?? {}),
    });

    this.horizontalScrollBar = new ScrollBar(renderer, {
      id: `${this._id}-hscroll`,
      orientation: "horizontal",
      height: 1,
      flexShrink: 0,
      visible: options.scrollX === true,
      ...(options.horizontalScrollbarOptions ?? options.scrollbarOptions ?? {}),
    });

    if (options.scrollY !== false) {
      super.add(this.verticalScrollBar);
    }

    this._stickyScroll = options.stickyScroll ?? false;
    this._keyHandler = this._handleKey.bind(this);
    this._lifecyclePass = (): void => {
      if (!this._isDestroyed) this._updateScrollbar();
    };
    renderer.registerLifecyclePass(this._lifecyclePass);
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

  override add(child: Box, index?: number): void {
    this.content.add(child, index);
    this._contentLines += child.getEstimatedHeight();
    this._updateScrollbar();
  }

  override remove(child: Box): void {
    this._contentLines = Math.max(0, this._contentLines - child.getEstimatedHeight());
    this.content.remove(child);
    this._updateScrollbar();
  }

  override getRenderable(id: string): Box | undefined {
    if (this._id === id) return this;
    return this.viewport.getRenderable(id) ?? super.getRenderable(id);
  }

  override focus(): void {
    if (this._isDestroyed || this._focused) return;
    this._focused = true;
    this.emit(RenderableEvents.FOCUSED, this);
    this._applyStyle();
    this._renderer.keyHandler.offInternal("keypress", this._keyHandler);
    this._renderer.keyHandler.onInternal("keypress", this._keyHandler);
  }

  override blur(): void {
    if (this._isDestroyed) return;
    this._renderer.keyHandler.offInternal("keypress", this._keyHandler);
    if (!this._focused) return;
    this._focused = false;
    this.emit(RenderableEvents.BLURRED, this);
    this._applyStyle();
  }

  scrollBy(delta: number, axis: "x" | "y" = "y"): void {
    if (axis === "y") {
      this.scrollTop += delta;
    } else {
      this.scrollLeft += delta;
    }
  }

  private _applyScroll(): void {
    // Use the engine's native scroll-offset mechanism on the viewport node.
    // The engine shifts all children of `viewport` by (-scrollX, -scrollY) during
    // render-tree building, and the viewport's overflow:hidden clip stops painting
    // outside its bounds.  This avoids negative-margin hacks that were clamped to 0.
    this._renderer.setScrollOffset(this.viewport.nodeId, 0, this._scrollTop);
    this._updateScrollbar();
  }

  private _estimateViewLines(): number {
    const borderOverhead = this._options.border === true ? 2 : 0;
    return Math.max(1, this._renderer.viewportHeight - borderOverhead);
  }

  private _updateScrollbar(): void {
    const viewLines = this._estimateViewLines();
    if (
      this._lastScrollTop === this._scrollTop &&
      this._lastContentLines === this._contentLines &&
      this._lastViewLines === viewLines
    ) {
      return;
    }
    this._lastScrollTop = this._scrollTop;
    this._lastContentLines = this._contentLines;
    this._lastViewLines = viewLines;
    this.verticalScrollBar.updateScrollBar(this._scrollTop, this._contentLines, viewLines);
  }

  private _handleKey(key: KeyEvent): void {
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
      this.scrollTop = Math.max(0, this._contentLines - this._estimateViewLines());
    }
  }

  override destroy(): void {
    if (this._isDestroyed) return;
    this._renderer.keyHandler.offInternal("keypress", this._keyHandler);
    this._renderer.unregisterLifecyclePass(this._lifecyclePass);
    this.viewport.destroyRecursively();
    this.verticalScrollBar.destroyRecursively();
    this.horizontalScrollBar.destroyRecursively();
    super.destroy();
  }
}
