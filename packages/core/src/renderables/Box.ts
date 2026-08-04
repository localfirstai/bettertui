/**
 * Box — the primary container widget.
 * Wraps an engine "Box" node with layout, styling, and event support.
 */

import { EventEmitter } from "node:events";
import type { LayoutConstraints } from "@bettertui/shared";
import { RenderableEvents } from "../lib/renderableEvents";
import { type ColorInput, type RGBA, parseColor, rgbaToEngineColor } from "../lib/rgba";
import type { CliRenderer } from "../platform/cliRenderer";

export type BorderSide = "top" | "right" | "bottom" | "left";
export type BorderStyleKind = "single" | "double" | "round" | "thick" | "dashed" | "ascii" | "none";

// Border character sets
const BORDER_CHARS: Record<BorderStyleKind, string[]> = {
  single: ["┌", "─", "┐", "│", "└", "─", "┘", "│"],
  double: ["╔", "═", "╗", "║", "╚", "═", "╝", "║"],
  round: ["╭", "─", "╮", "│", "╰", "─", "╯", "│"],
  thick: ["┏", "━", "┓", "┃", "┗", "━", "┛", "┃"],
  dashed: ["╌", "╌", "╌", "┆", "╌", "╌", "╌", "┆"],
  ascii: ["+", "-", "+", "|", "+", "-", "+", "|"],
  none: [" ", " ", " ", " ", " ", " ", " ", " "],
};

export interface BoxOptions {
  id?: string;
  width?: number | string;
  height?: number | string;
  minWidth?: number | string;
  maxWidth?: number | string;
  minHeight?: number | string;
  maxHeight?: number | string;
  position?: "relative" | "absolute";
  top?: number | string;
  right?: number | string;
  bottom?: number | string;
  left?: number | string;
  zIndex?: number;
  flexDirection?: "row" | "column" | "row-reverse" | "column-reverse";
  flexGrow?: number;
  flexShrink?: number;
  flexBasis?: number | string;
  flexWrap?: "nowrap" | "wrap";
  alignItems?: "flex-start" | "center" | "flex-end" | "stretch" | "baseline";
  alignSelf?: "flex-start" | "center" | "flex-end" | "stretch" | "baseline";
  justifyContent?:
    | "flex-start"
    | "center"
    | "flex-end"
    | "space-between"
    | "space-around"
    | "space-evenly";
  overflow?: "visible" | "hidden" | "scroll";
  gap?: number;
  rowGap?: number;
  columnGap?: number;
  padding?: number;
  paddingX?: number;
  paddingY?: number;
  paddingTop?: number;
  paddingRight?: number;
  paddingBottom?: number;
  paddingLeft?: number;
  margin?: number;
  marginX?: number;
  marginY?: number;
  marginTop?: number;
  marginRight?: number;
  marginBottom?: number;
  marginLeft?: number;
  backgroundColor?: ColorInput;
  borderStyle?: BorderStyleKind;
  border?: boolean | BorderSide[];
  borderColor?: ColorInput;
  focusedBorderColor?: ColorInput;
  title?: string;
  titleColor?: ColorInput;
  titleAlignment?: "left" | "center" | "right";
  bottomTitle?: string;
  bottomTitleAlignment?: "left" | "center" | "right";
  opacity?: number;
  visible?: boolean;
  buffered?: boolean;
  focusable?: boolean;
  onMouseDown?: (event: unknown) => void;
  onMouseUp?: (event: unknown) => void;
  onMouseMove?: (event: unknown) => void;
  onMouseDrag?: (event: unknown) => void;
  onMouseDragEnd?: (event: unknown) => void;
  onMouseDrop?: (event: unknown) => void;
  onMouseOver?: (event: unknown) => void;
  onMouseOut?: (event: unknown) => void;
  onMouseScroll?: (event: unknown) => void;
  onMouse?: (event: unknown) => void;
  onKeyDown?: (key: unknown) => void;
  onClick?: (event: unknown) => void;
  onSizeChange?: () => void;
  /**
   * Called after each render frame with a buffer handle.
   * Bound to the Box instance (`this` = the renderable).
   * Use this for custom per-frame drawing on top of the box.
   */
  renderAfter?: (this: Box, buffer: unknown, deltaTime?: number) => void;
}

let _boxCounter = 0;

/** Minimal stub buffer passed to renderAfter callbacks. */
function createStubBuffer(box: Box): unknown {
  const bg = new Uint16Array(0);
  const fg = new Uint16Array(0);
  const char = new Uint32Array(0);
  const attributes = new Uint32Array(0);
  return {
    get width() {
      return typeof box.width === "number" ? box.width : 0;
    },
    get height() {
      return typeof box.height === "number" ? box.height : 0;
    },
    buffers: { bg, fg, char, attributes },
    setCell() {},
    drawText() {},
    fillRect() {},
    colorMatrix() {},
    pushScissorRect() {},
    popScissorRect() {},
    pushOpacity() {},
    popOpacity() {},
    clear() {},
  };
}

export class Box extends EventEmitter {
  protected readonly _renderer: CliRenderer;
  protected _nodeId: number;
  protected readonly _id: string;
  protected _focused = false;
  protected _visible: boolean;
  protected _isDestroyed = false;
  protected _opacity: number;
  protected _backgroundColor: RGBA | null = null;
  protected _borderStyle: BorderStyleKind;
  protected _border: boolean | BorderSide[];
  protected _borderColor: RGBA;
  protected _focusedBorderColor: RGBA;
  protected _focusable: boolean;
  protected _title: string | undefined;
  protected _titleColor: RGBA | undefined;
  protected _titleAlignment: "left" | "center" | "right";
  protected _children: Map<string, Box> = new Map();
  protected _childList: Box[] = [];
  protected _options: BoxOptions;
  private _renderAfterCallback: ((dt: number) => void) | null = null;

  constructor(renderer: CliRenderer, options: BoxOptions = {}, existingNodeId?: number) {
    super();
    _boxCounter++;
    this._id = options.id ?? `box-${_boxCounter}`;
    this._renderer = renderer;
    this._options = options;
    this._nodeId = existingNodeId ?? renderer.createNode("Box");
    this._visible = options.visible !== false;
    this._opacity = options.opacity ?? 1;
    this._focusable = options.focusable ?? false;
    this._borderStyle = options.borderStyle ?? "single";
    this._border = options.border ?? false;
    this._borderColor = parseColor(options.borderColor ?? "#ffffff");
    this._focusedBorderColor = parseColor(options.focusedBorderColor ?? "#00aaff");
    this._title = options.title;
    this._titleColor = options.titleColor ? parseColor(options.titleColor) : undefined;
    this._titleAlignment = options.titleAlignment ?? "left";

    if (options.backgroundColor) {
      this._backgroundColor = parseColor(options.backgroundColor);
    }

    this._applyLayout(options);
    this._applyStyle();

    // Register renderAfter frame callback if provided
    if (options.renderAfter) {
      const buf = createStubBuffer(this);
      this._renderAfterCallback = (dt: number) => {
        if (!this._isDestroyed && options.renderAfter) {
          options.renderAfter.call(this, buf, dt);
        }
      };
      renderer.setFrameCallback(this._renderAfterCallback);
    }
  }

  get id(): string {
    return this._id;
  }
  get nodeId(): number {
    return this._nodeId;
  }
  get focused(): boolean {
    return this._focused;
  }
  get visible(): boolean {
    return this._visible;
  }
  get isDestroyed(): boolean {
    return this._isDestroyed;
  }
  get opacity(): number {
    return this._opacity;
  }
  get backgroundColor(): RGBA | null {
    return this._backgroundColor;
  }
  get borderStyle(): BorderStyleKind {
    return this._borderStyle;
  }
  get border(): boolean | BorderSide[] {
    return this._border;
  }
  get renderer(): CliRenderer {
    return this._renderer;
  }

  /** Computed layout width (from options; not the engine-resolved value). */
  get width(): number | string | undefined {
    return this._options.width;
  }
  /** Computed layout height (from options; not the engine-resolved value). */
  get height(): number | string | undefined {
    return this._options.height;
  }
  /** Screen X position (approximate; engine is the source of truth). */
  get x(): number {
    return typeof this._options.left === "number" ? this._options.left : 0;
  }
  /** Screen Y position (approximate; engine is the source of truth). */
  get y(): number {
    return typeof this._options.top === "number" ? this._options.top : 0;
  }
  get screenX(): number {
    return this.x;
  }
  get screenY(): number {
    return this.y;
  }

  set visible(value: boolean) {
    if (this._visible !== value) {
      this._visible = value;
      this._renderer.setNodeLayout(this._nodeId, {
        display: value ? "flex" : "none",
      });
    }
  }

  set opacity(value: number) {
    this._opacity = Math.max(0, Math.min(1, value));
    this._applyStyle();
  }

  set backgroundColor(color: ColorInput) {
    this._backgroundColor = parseColor(color);
    this._applyStyle();
  }

  set borderColor(color: ColorInput) {
    this._borderColor = parseColor(color);
    this._applyStyle();
  }

  set borderStyle(style: BorderStyleKind) {
    this._borderStyle = style;
    this._applyStyle();
  }

  set border(value: boolean | BorderSide[]) {
    this._border = value;
    this._applyStyle();
  }

  set title(value: string | undefined) {
    this._title = value;
    this._applyStyle();
  }

  set focusedBorderColor(color: ColorInput) {
    this._focusedBorderColor = parseColor(color);
    this._applyStyle();
  }

  set width(value: number | string) {
    this._options.width = value;
    this._renderer.setNodeLayout(this._nodeId, { width: value });
  }

  set height(value: number | string) {
    this._options.height = value;
    this._renderer.setNodeLayout(this._nodeId, { height: value });
  }

  set flexDirection(value: BoxOptions["flexDirection"]) {
    this._options.flexDirection = value;
    if (value) this._renderer.setNodeLayout(this._nodeId, { flexDirection: value });
  }

  set flexGrow(value: number) {
    this._options.flexGrow = value;
    this._renderer.setNodeLayout(this._nodeId, { flexGrow: value });
  }

  set flexBasis(value: number | string) {
    this._options.flexBasis = value;
    this._renderer.setNodeLayout(this._nodeId, { flexBasis: value });
  }

  set marginBottom(value: number) {
    this._options.marginBottom = value;
    this._renderer.setNodeLayout(this._nodeId, { marginBottom: value });
  }

  set marginTop(value: number) {
    this._options.marginTop = value;
    this._renderer.setNodeLayout(this._nodeId, { marginTop: value });
  }

  set marginLeft(value: number) {
    this._options.marginLeft = value;
    this._renderer.setNodeLayout(this._nodeId, { marginLeft: value });
  }

  set marginRight(value: number) {
    this._options.marginRight = value;
    this._renderer.setNodeLayout(this._nodeId, { marginRight: value });
  }

  set zIndex(value: number) {
    this._options.zIndex = value;
    this._renderer.setNodeLayout(this._nodeId, { zIndex: value });
  }

  add(child: Box, index?: number): void {
    if (this._isDestroyed) return;
    this._children.set(child.id, child);
    if (index !== undefined) {
      this._childList.splice(index, 0, child);
    } else {
      this._childList.push(child);
    }
    this._renderer.appendChild(this._nodeId, child._nodeId);
  }

  remove(child: Box): void {
    if (this._isDestroyed) return;
    this._children.delete(child.id);
    const idx = this._childList.indexOf(child);
    if (idx !== -1) this._childList.splice(idx, 1);
    this._renderer.removeNode(child._nodeId);
  }

  getRenderable(id: string): Box | undefined {
    if (this._id === id) return this;
    for (const child of this._childList) {
      const found = child.getRenderable(id);
      if (found) return found;
    }
    return undefined;
  }

  getChildren(): Box[] {
    return [...this._childList];
  }

  // ── Focus management ──────────────────────────────────────────────────────────

  focus(): void {
    if (this._isDestroyed) return;
    this._focused = true;
    this.emit(RenderableEvents.FOCUSED, this);
    this._applyStyle();
  }

  blur(): void {
    if (this._isDestroyed) return;
    this._focused = false;
    this.emit(RenderableEvents.BLURRED, this);
    this._applyStyle();
  }

  // ── Lifecycle ─────────────────────────────────────────────────────────────────

  destroy(): void {
    if (this._isDestroyed) return;
    this._isDestroyed = true;
    this.emit(RenderableEvents.DESTROYED, this);
    this.removeAllListeners();
    if (this._renderAfterCallback) {
      this._renderer.removeFrameCallback(this._renderAfterCallback);
      this._renderAfterCallback = null;
    }
    try {
      this._renderer.removeNode(this._nodeId);
    } catch {
      // ignore - may already be removed
    }
    this._children.clear();
    this._childList = [];
  }

  destroyRecursively(): void {
    for (const child of [...this._childList]) {
      child.destroyRecursively();
    }
    this.destroy();
  }

  // ── Layout setters ───────────────────────────────────────────────────────────

  setLayout(layout: Partial<BoxOptions>): void {
    this._applyLayout(layout);
  }

  setPosition(pos: {
    top?: number | string;
    left?: number | string;
    right?: number | string;
    bottom?: number | string;
  }): void {
    const layout: LayoutConstraints = {};
    if (pos.top !== undefined) layout.top = pos.top as number;
    if (pos.left !== undefined) layout.left = pos.left as number;
    if (pos.right !== undefined) layout.right = pos.right as number;
    if (pos.bottom !== undefined) layout.bottom = pos.bottom as number;
    this._renderer.setNodeLayout(this._nodeId, layout);
  }

  // ── Internal helpers ──────────────────────────────────────────────────────────

  protected _applyLayout(options: Partial<BoxOptions>): void {
    const layout: LayoutConstraints = {};

    if (options.width !== undefined) layout.width = options.width;
    if (options.height !== undefined) layout.height = options.height;
    if (options.minWidth !== undefined) layout.minWidth = options.minWidth;
    if (options.maxWidth !== undefined) layout.maxWidth = options.maxWidth;
    if (options.minHeight !== undefined) layout.minHeight = options.minHeight;
    if (options.maxHeight !== undefined) layout.maxHeight = options.maxHeight;
    if (options.position !== undefined) layout.position = options.position;
    if (options.top !== undefined) layout.top = options.top as number;
    if (options.right !== undefined) layout.right = options.right as number;
    if (options.bottom !== undefined) layout.bottom = options.bottom as number;
    if (options.left !== undefined) layout.left = options.left as number;
    if (options.zIndex !== undefined) layout.zIndex = options.zIndex;
    if (options.flexDirection !== undefined) layout.flexDirection = options.flexDirection;
    if (options.flexGrow !== undefined) layout.flexGrow = options.flexGrow;
    if (options.flexShrink !== undefined) layout.flexShrink = options.flexShrink;
    if (options.flexWrap !== undefined) layout.flexWrap = options.flexWrap;
    if (options.alignItems !== undefined) layout.alignItems = options.alignItems;
    if (options.alignSelf !== undefined) layout.alignSelf = options.alignSelf;
    if (options.justifyContent !== undefined) layout.justifyContent = options.justifyContent;
    if (options.overflow !== undefined) layout.overflow = options.overflow;

    // Gap
    if (
      options.gap !== undefined &&
      options.rowGap === undefined &&
      options.columnGap === undefined
    ) {
      layout.gap = options.gap;
    } else if (options.rowGap !== undefined || options.columnGap !== undefined) {
      layout.gap = { row: options.rowGap, column: options.columnGap };
    }

    // Padding (resolve shorthand)
    const pt = options.paddingTop ?? options.paddingY ?? options.padding;
    const pr = options.paddingRight ?? options.paddingX ?? options.padding;
    const pb = options.paddingBottom ?? options.paddingY ?? options.padding;
    const pl = options.paddingLeft ?? options.paddingX ?? options.padding;
    if (pt !== undefined) layout.paddingTop = pt;
    if (pr !== undefined) layout.paddingRight = pr;
    if (pb !== undefined) layout.paddingBottom = pb;
    if (pl !== undefined) layout.paddingLeft = pl;

    // Margin
    const mt = options.marginTop ?? options.marginY ?? options.margin;
    const mr = options.marginRight ?? options.marginX ?? options.margin;
    const mb = options.marginBottom ?? options.marginY ?? options.margin;
    const ml = options.marginLeft ?? options.marginX ?? options.margin;
    if (mt !== undefined) layout.marginTop = mt;
    if (mr !== undefined) layout.marginRight = mr;
    if (mb !== undefined) layout.marginBottom = mb;
    if (ml !== undefined) layout.marginLeft = ml;

    if (options.visible === false) layout.display = "none";

    // Border layout contribution: reserve space for border cells so the
    // engine's box-sizing accounts for the border width.
    const borderVal = this._options.border;
    if (borderVal === true) {
      layout.borderTop = 1;
      layout.borderRight = 1;
      layout.borderBottom = 1;
      layout.borderLeft = 1;
    } else if (Array.isArray(borderVal) && borderVal.length > 0) {
      layout.borderTop = borderVal.includes("top") ? 1 : 0;
      layout.borderRight = borderVal.includes("right") ? 1 : 0;
      layout.borderBottom = borderVal.includes("bottom") ? 1 : 0;
      layout.borderLeft = borderVal.includes("left") ? 1 : 0;
    }

    this._renderer.setNodeLayout(this._nodeId, layout);
  }

  protected _applyStyle(): void {
    const bgColor = this._focused && this._border ? this._backgroundColor : this._backgroundColor;

    const styleJson: Record<string, unknown> = {};
    if (bgColor && bgColor.a > 0) {
      styleJson.bg = rgbaToEngineColor(bgColor);
    }

    // Border via engine style (extra fields beyond TypeScript Style type)
    const hasBorder =
      this._border === true || (Array.isArray(this._border) && this._border.length > 0);
    if (hasBorder) {
      const borderColor = this._focused ? this._focusedBorderColor : this._borderColor;
      styleJson.border = this._borderStyle;
      styleJson.border_color = rgbaToEngineColor(borderColor);
      if (this._title) {
        styleJson.title = this._title;
        styleJson.title_align = this._titleAlignment ?? "left";
        if (this._titleColor) {
          styleJson.title_color = rgbaToEngineColor(this._titleColor);
        }
      }
    }

    // Opacity
    if (this._opacity < 1) {
      styleJson.opacity = this._opacity;
    }

    // biome-ignore lint/suspicious/noExplicitAny: engine accepts extended style fields beyond the TypeScript Style interface
    this._renderer.setNodeStyle(this._nodeId, styleJson as any);
  }
}

/**
 * Root — the scene root, wrapping the engine's root node.
 * Created automatically by CliRenderer and exposed as `renderer.root`.
 */
export class Root extends Box {
  constructor(renderer: CliRenderer) {
    super(
      renderer,
      { flexDirection: "column", width: "100%", height: "100%" },
      renderer.rootNodeId,
    );
  }

  destroy(): void {
    // Root can never be destroyed
  }
}

export { BORDER_CHARS };
