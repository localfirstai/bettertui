/**
 * TextNode — building block for styled text composition.
 *
 * Design:
 * - Children are `string | TextNode` (heterogeneous).
 * - Leaf text is stored as a string child, NOT a separate `_text` field.
 *   This means `clear()` always wipes all content and `children` setter
 *   works correctly for dynamic updates.
 * - `fromNodes(nodes[], options?)` takes array + options signature.
 * - `children` setter re-parents the new children and marks the node dirty.
 */

import { type ColorInput, type RGBA, parseColor } from "../lib/rgba";
import { type StyledText, type TextChunk, styledTextToAnsi } from "../lib/styledText";
import { TextAttributes } from "../lib/styledText";
import { StyledText as StyledTextClass } from "../lib/styledText";

export interface TextNodeOptions {
  id?: string;
  fg?: ColorInput;
  bg?: ColorInput;
  bold?: boolean;
  italic?: boolean;
  underline?: boolean;
  dim?: boolean;
  strikethrough?: boolean;
  blink?: boolean;
}

export interface StyleAttrs {
  fg?: ColorInput;
  bg?: ColorInput;
  /** Pre-computed attribute bitmask (TextAttributes flags). */
  attributes?: number;
}

/** A child can be either a string (leaf text) or a nested node. */
export type TextNodeChild = string | TextNode;

let _textNodeCounter = 0;

/**
 * TextNode — a lightweight styled-text composition node.
 * Can be used standalone or nested in Text.
 */
export class TextNode {
  private static _counter = 0;
  public readonly id: string;
  public _fg: RGBA | undefined;
  public _bg: RGBA | undefined;
  public _attributes: number;
  public isDirty = false;
  public parent: TextNode | null = null;

  /**
   * Children are heterogeneous: strings are leaf text, TextNodeRenderables are
   * nested nodes. Stored as a union array.
   *
   * NOTE: Do NOT store leaf text in a separate field — always use children.
   * This ensures `clear()` wipes everything and the `children` setter works.
   */
  protected _children: TextNodeChild[] = [];

  constructor(options: TextNodeOptions = {}) {
    _textNodeCounter++;
    this.id = options.id ?? `textnode-${_textNodeCounter}`;
    this._fg = options.fg ? parseColor(options.fg) : undefined;
    this._bg = options.bg ? parseColor(options.bg) : undefined;
    this._attributes = 0;
    if (options.bold) this._attributes |= TextAttributes.BOLD;
    if (options.italic) this._attributes |= TextAttributes.ITALIC;
    if (options.underline) this._attributes |= TextAttributes.UNDERLINE;
    if (options.dim) this._attributes |= TextAttributes.DIM;
    if (options.strikethrough) this._attributes |= TextAttributes.STRIKETHROUGH;
    if (options.blink) this._attributes |= TextAttributes.BLINK;
  }

  // ── Factory methods ────────────────────────────────────────────────────────

  /**
   * Create a leaf node from a plain string with optional style.
   * Signature: `TextNode.fromString(text, options?)`.
   */
  static fromString(text: string, style?: StyleAttrs): TextNode {
    const node = new TextNode({
      fg: style?.fg,
      bg: style?.bg,
    });
    if (style?.attributes) node._attributes = style.attributes;
    // Store as a string child — this is the canonical storage location.
    node._children = [text];
    return node;
  }

  /**
   * Create a container node from an array of child nodes with optional root style.
   * Signature: `fromNodes(nodes: TextNode[], options?)`.
   *
   * Previous implementation used variadic rest params and no options,
   * which broke call sites that pass `([a,b,c], { fg: "..." })`.
   */
  static fromNodes(nodes: TextNode[], options: StyleAttrs = {}): TextNode {
    const root = new TextNode({
      fg: options.fg,
      bg: options.bg,
    });
    if (options.attributes) root._attributes = options.attributes;
    for (const node of nodes) root.add(node);
    return root;
  }

  // ── Style getters / setters ────────────────────────────────────────────────

  get fg(): RGBA | undefined {
    return this._fg;
  }

  set fg(color: ColorInput) {
    this._fg = parseColor(color);
    this.isDirty = true;
    this._bubbleDirty();
  }

  get bg(): RGBA | undefined {
    return this._bg;
  }

  set bg(color: ColorInput) {
    this._bg = parseColor(color);
    this.isDirty = true;
    this._bubbleDirty();
  }

  get attributes(): number {
    return this._attributes;
  }

  set attributes(v: number) {
    this._attributes = v;
    this.isDirty = true;
    this._bubbleDirty();
  }

  // ── Children ──────────────────────────────────────────────────────────────

  /**
   * Read-only view of this node's children (strings + sub-nodes).
   * For mutation use the setter or `add`/`remove`/`clear`.
   */
  get children(): readonly TextNodeChild[] {
    return this._children;
  }

  /**
   * Replace all children with a new array of strings and/or nodes.
   * The `children` setter: detaches old node children, adopts
   * new ones, and marks the node dirty so the owner Text resyncs.
   *
   * Usage (dynamic update pattern):
   * ```ts
   * counterNode.children = [`\n\nCounter: ${n}`];
   * ```
   */
  set children(newChildren: TextNodeChild[]) {
    // Detach old node children
    for (const child of this._children) {
      if (child instanceof TextNode) {
        child.parent = null;
      }
    }
    // Adopt new node children
    for (const child of newChildren) {
      if (child instanceof TextNode) {
        child.parent = this;
      }
    }
    this._children = [...newChildren];
    this.isDirty = true;
    this._bubbleDirty();
  }

  // ── Mutation methods ───────────────────────────────────────────────────────

  /**
   * Append a string, TextNode, or StyledText as a child.
   * Returns the index at which the child was inserted.
   */
  add(child: TextNodeChild | StyledText, index?: number): number {
    let item: TextNodeChild;
    if (typeof child === "string") {
      item = child;
    } else if (child instanceof TextNode) {
      child.parent = this;
      item = child;
    } else {
      // StyledText — serialise to ANSI string and store as a leaf string child
      item = styledTextToAnsi(child as StyledText);
    }

    if (index !== undefined) {
      this._children.splice(index, 0, item);
    } else {
      this._children.push(item);
    }
    this.isDirty = true;
    this._bubbleDirty();
    return index ?? this._children.length - 1;
  }

  remove(child: TextNode): void {
    const idx = this._children.indexOf(child);
    if (idx !== -1) {
      this._children.splice(idx, 1);
      child.parent = null;
      this.isDirty = true;
      this._bubbleDirty();
    }
  }

  /**
   * Insert `child` before `anchor`. Throws if `anchor` is provided but not
   * found — strict contract (helps catch anchor mismatches).
   */
  insertBefore(child: TextNodeChild | StyledText, anchor?: TextNode): void {
    const item =
      typeof child === "string" || child instanceof TextNode
        ? child
        : styledTextToAnsi(child as StyledText);

    if (!anchor) {
      this._children.unshift(item);
    } else {
      const idx = this._children.indexOf(anchor);
      if (idx === -1) {
        throw new Error(
          `[TextNode] insertBefore: anchor node (id=${anchor.id}) not found among children`,
        );
      }
      this._children.splice(idx, 0, item);
    }
    if (item instanceof TextNode) item.parent = this;
    this.isDirty = true;
    this._bubbleDirty();
  }

  /**
   * Remove all children and mark the node dirty.
   * Unlike the old implementation there is NO separate `_text` field to miss.
   */
  clear(): void {
    for (const child of this._children) {
      if (child instanceof TextNode) {
        child.parent = null;
      }
    }
    this._children = [];
    this.isDirty = true;
    this._bubbleDirty();
  }

  getChildren(): readonly TextNodeChild[] {
    return this._children;
  }

  // ── Rendering helpers ─────────────────────────────────────────────────────

  /**
   * Walk this node and all descendants, accumulating {@link TextChunk}s with
   * inherited style applied. Called by `Text.onLifecyclePass` to
   * build the flat chunk array that goes to the engine.
   */
  gatherWithInheritedStyle(inherited: {
    fg?: RGBA;
    bg?: RGBA;
    attributes?: number;
    link?: { url: string } | undefined;
  }): TextChunk[] {
    const chunks: TextChunk[] = [];

    const fg = this._fg ?? inherited.fg;
    const bg = this._bg ?? inherited.bg;
    const attrs = this._attributes
      ? (inherited.attributes ?? 0) | this._attributes
      : inherited.attributes;

    for (const child of this._children) {
      if (typeof child === "string") {
        // Leaf string child — emit as a chunk with current inherited style
        if (child) {
          chunks.push({
            __isChunk: true,
            text: child,
            fg,
            bg,
            attributes: attrs,
          });
        }
      } else {
        // Nested node — recurse
        chunks.push(
          ...child.gatherWithInheritedStyle({
            fg,
            bg,
            attributes: attrs,
            link: inherited.link,
          }),
        );
      }
    }

    return chunks;
  }

  /** Serialise this node tree to an ANSI-escaped string. */
  toString(): string {
    const chunks = this.gatherWithInheritedStyle({});
    const st = new StyledTextClass(chunks);
    return styledTextToAnsi(st);
  }

  // ── Internal ──────────────────────────────────────────────────────────────

  /** Walk up the parent chain and mark all ancestors dirty. */
  private _bubbleDirty(): void {
    let p = this.parent;
    while (p) {
      p.isDirty = true;
      // If we reach a RootTextNode, notify it so it can signal its
      // owning Text to resync on the next lifecycle pass.
      if (p instanceof RootTextNode) {
        p.markDirtyFromChild();
        break;
      }
      p = p.parent;
    }
  }
}

/**
 * RootTextNode — the root text node for a Text.
 * When any descendant calls `_bubbleDirty()` and the dirty flag reaches this
 * root, the `onDirty` callback is invoked so the owning `Text` can
 * schedule a re-sync to the engine on the next lifecycle pass.
 */
export class RootTextNode extends TextNode {
  private readonly _onDirty: (() => void) | undefined;

  constructor(options: TextNodeOptions = {}, onDirty?: () => void) {
    super(options);
    this._onDirty = onDirty;
  }

  /**
   * Overrides the private `_bubbleDirty` propagation: when this root is
   * reached, fire the `onDirty` callback instead of (or in addition to)
   * walking further up (there is no parent above the root).
   */
  markDirtyFromChild(): void {
    this.isDirty = true;
    this._onDirty?.();
  }
}
