/**
 * TextNodeRenderable — building block for styled text composition.
 */

import { EventEmitter } from "node:events";
import { type ColorInput, type RGBA, parseColor } from "../lib/rgba";
import { type StyledText, type TextChunk, styledTextToAnsi } from "../lib/styledText";
import { TextAttributes } from "../lib/styledText";

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
  attributes?: number;
}

let _textNodeCounter = 0;

/**
 * TextNodeRenderable — a lightweight styled-text composition node.
 * Can be used standalone or nested in TextRenderable.
 */
export class TextNodeRenderable extends EventEmitter {
  private static _counter = 0;
  public readonly id: string;
  public _fg: RGBA | undefined;
  public _bg: RGBA | undefined;
  public _attributes: number;
  public isDirty = false;
  public parent: TextNodeRenderable | null = null;
  protected _children: TextNodeRenderable[] = [];
  private _text: string;

  constructor(options: TextNodeOptions = {}) {
    super();
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
    this._text = "";
  }

  static fromString(text: string, style?: StyleAttrs): TextNodeRenderable {
    const node = new TextNodeRenderable({
      fg: style?.fg,
      bg: style?.bg,
    });
    node._text = text;
    if (style?.attributes) node._attributes = style.attributes;
    return node;
  }

  static fromNodes(...nodes: TextNodeRenderable[]): TextNodeRenderable {
    const root = new TextNodeRenderable();
    for (const node of nodes) root.add(node);
    return root;
  }

  get fg(): RGBA | undefined {
    return this._fg;
  }

  set fg(color: ColorInput) {
    this._fg = parseColor(color);
    this.isDirty = true;
  }

  get bg(): RGBA | undefined {
    return this._bg;
  }

  set bg(color: ColorInput) {
    this._bg = parseColor(color);
    this.isDirty = true;
  }

  get attributes(): number {
    return this._attributes;
  }

  set attributes(v: number) {
    this._attributes = v;
    this.isDirty = true;
  }

  get children(): TextNodeRenderable[] {
    return this._children;
  }

  add(child: TextNodeRenderable | StyledText | string, index?: number): number {
    let node: TextNodeRenderable;
    if (typeof child === "string") {
      node = TextNodeRenderable.fromString(child);
    } else if (child instanceof TextNodeRenderable) {
      node = child;
    } else {
      // StyledText
      node = new TextNodeRenderable();
      node._text = styledTextToAnsi(child);
    }
    node.parent = this;
    if (index !== undefined) {
      this._children.splice(index, 0, node);
    } else {
      this._children.push(node);
    }
    this.isDirty = true;
    return index ?? this._children.length - 1;
  }

  remove(child: TextNodeRenderable): void {
    const idx = this._children.indexOf(child);
    if (idx !== -1) {
      this._children.splice(idx, 1);
      child.parent = null;
      this.isDirty = true;
    }
  }

  insertBefore(child: TextNodeRenderable | string, anchor?: TextNodeRenderable): void {
    let node: TextNodeRenderable;
    if (typeof child === "string") {
      node = TextNodeRenderable.fromString(child);
    } else {
      node = child;
    }
    if (!anchor) {
      this._children.unshift(node);
    } else {
      const idx = this._children.indexOf(anchor);
      if (idx !== -1) {
        this._children.splice(idx, 0, node);
      } else {
        this._children.push(node);
      }
    }
    node.parent = this;
    this.isDirty = true;
  }

  clear(): void {
    for (const child of this._children) {
      child.parent = null;
    }
    this._children = [];
    this.isDirty = true;
  }

  getChildren(): TextNodeRenderable[] {
    return this._children;
  }

  /** Collect all text from this node and descendants as styled chunks. */
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

    if (this._text) {
      chunks.push({
        __isChunk: true,
        text: this._text,
        fg,
        bg,
        attributes: attrs,
      });
    }

    for (const child of this._children) {
      chunks.push(
        ...child.gatherWithInheritedStyle({ fg, bg, attributes: attrs, link: inherited.link }),
      );
    }

    return chunks;
  }

  /** Convert this node tree to a string. */
  toString(): string {
    const chunks = this.gatherWithInheritedStyle({});
    const st = new (require("../lib/styledText").StyledText)(chunks);
    return styledTextToAnsi(st);
  }
}

/**
 * RootTextNodeRenderable — the root text node for a TextRenderable.
 */
export class RootTextNodeRenderable extends TextNodeRenderable {
  constructor(
    _ctx: unknown,
    options: TextNodeOptions = {},
    public readonly owner?: unknown,
  ) {
    super(options);
  }
}
