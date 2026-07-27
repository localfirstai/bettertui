/**
 * TextRenderable — displays styled text content.
 * Supports StyledText, plain strings, and multi-line text.
 */

import { type ColorInput, type RGBA, parseColor, rgbaToEngineColor } from "../lib/rgba";
import {
  type StyledText,
  isStyledText,
  stringToStyledText,
  styledTextToAnsi,
} from "../lib/styledText";
import type { CliRenderer } from "../platform/cliRenderer";
import { type BoxOptions, BoxRenderable } from "./Box";
import type { TextNodeRenderable } from "./TextNode";

export interface TextOptions extends BoxOptions {
  content?: StyledText | string;
  /** Foreground (text) color. */
  fg?: ColorInput;
  /** Background color (alias for backgroundColor). */
  bg?: ColorInput;
  /** Text wrap mode. */
  wrapMode?: "none" | "char" | "word";
  /** Truncate long lines with ellipsis. */
  truncate?: boolean;
  /** Text alignment. */
  textAlign?: "left" | "center" | "right";
  margin?: number;
  /** Enable text selection. */
  selectable?: boolean;
  /** Selection background color. */
  selectionBg?: ColorInput;
  /** Selection foreground color. */
  selectionFg?: ColorInput;
}

let _textCounter = 0;

export class TextRenderable extends BoxRenderable {
  private _content: StyledText;
  private _fg: RGBA | null = null;
  private _bg: RGBA | null = null;
  private _textNodeId: number;
  private _wrapMode: "none" | "char" | "word";
  private _truncate: boolean;

  constructor(renderer: CliRenderer, options: TextOptions = {}) {
    _textCounter++;
    // Create a Box parent for layout, then a Text child for content
    super(renderer, {
      ...options,
      id: options.id ?? `text-${_textCounter}`,
      backgroundColor: options.bg ?? options.backgroundColor,
    });

    if (options.fg) this._fg = parseColor(options.fg);
    if (options.bg) this._bg = parseColor(options.bg);
    this._wrapMode = options.wrapMode ?? "none";
    this._truncate = options.truncate ?? false;

    // Create the inner Text node
    this._textNodeId = renderer.createNode("Text");
    renderer.appendChild(this._nodeId, this._textNodeId);

    // Parse initial content
    const raw = options.content ?? "";
    this._content = typeof raw === "string" ? stringToStyledText(raw) : raw;

    this._applyTextStyle(options);
    this._updateText();
  }

  get content(): StyledText {
    return this._content;
  }

  set content(value: StyledText | string | string[]) {
    // Accept string arrays (join with newline)
    const normalized = Array.isArray(value) ? value.join("\n") : value;
    this._content = isStyledText(normalized) ? normalized : stringToStyledText(String(normalized));
    this._updateText();
  }

  get wrapMode(): "none" | "char" | "word" {
    return this._wrapMode;
  }

  set wrapMode(value: "none" | "char" | "word") {
    this._wrapMode = value;
    this._applyTextStyle({ wrapMode: value });
  }

  get truncate(): boolean {
    return this._truncate;
  }

  set truncate(value: boolean) {
    this._truncate = value;
    const styleJson: Record<string, unknown> = { text_truncate: value };
    // biome-ignore lint/suspicious/noExplicitAny: engine accepts extended style JSON
    this._renderer.setNodeStyle(this._textNodeId, styleJson as any);
  }

  get fg(): RGBA | null {
    return this._fg;
  }

  set fg(color: ColorInput) {
    this._fg = parseColor(color);
    this._applyTextStyle({});
  }

  set bg(color: ColorInput) {
    this._bg = parseColor(color);
    this._applyTextStyle({});
    // Also update background on the box
    this.backgroundColor = color;
  }

  set textColor(color: ColorInput) {
    this.fg = color;
  }

  /** Clear all text content. */
  clear(): void {
    this.content = "";
  }

  /** Add a TextNodeRenderable's content (converts to ANSI and appends). */
  addNode(node: TextNodeRenderable): void {
    const ansi = node.toString();
    const existing = styledTextToAnsi(this._content);
    this.content = existing + ansi;
  }

  /** No-op lifecycle hook for compatibility. */
  onLifecyclePass(): void {
    // no-op
  }

  // ── Internal ──────────────────────────────────────────────────────────────────

  private _updateText(): void {
    if (this._isDestroyed) return;
    const ansi = styledTextToAnsi(this._content);
    this._renderer.setText(this._textNodeId, ansi);
  }

  private _applyTextStyle(options: Partial<TextOptions>): void {
    const styleJson: Record<string, unknown> = {};
    if (this._fg) styleJson.fg = rgbaToEngineColor(this._fg);
    if (this._bg) styleJson.bg = rgbaToEngineColor(this._bg);
    if (options.textAlign) styleJson.text_align = options.textAlign;
    const wm = options.wrapMode ?? this._wrapMode;
    if (wm) styleJson.text_wrap = wm !== "none";
    // biome-ignore lint/suspicious/noExplicitAny: engine accepts extended style JSON
    this._renderer.setNodeStyle(this._textNodeId, styleJson as any);
  }

  override destroy(): void {
    if (this._isDestroyed) return;
    try {
      this._renderer.removeNode(this._textNodeId);
    } catch {
      // ignore
    }
    super.destroy();
  }
}
