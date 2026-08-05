/**
 * Text — displays styled text content backed by a TextNode tree.
 *
 * Architecture:
 * - Owns a `RootTextNode` that acts as the root of a composable
 *   styled-text tree.
 * - `add(node)` attaches a TextNode to the root
 *   (`demoText.add(containerNode)` API — no ANSI round-trip workaround needed).
 * - `onLifecyclePass()` is called once per frame by the `CliRenderer` lifecycle
 *   loop; it checks `rootTextNode.isDirty`, gathers chunks, serialises to ANSI,
 *   and pushes to the engine. This enables the dynamic-update pattern:
 *     ```ts
 *     counterNode.children = [`Counter: ${n}`];
 *     // ↑ marks the tree dirty; no manual re-serialisation required.
 *     ```
 * - `content` (string / StyledText) setter is kept for backward-compatibility.
 * - The previous ANSI-flatten-on-every-setter approach is replaced: mutations
 *   to the node tree are deferred to the lifecycle pass (O(1) dirty-check, not
 *   O(n) serialise-on-every-frame).
 */

import { type ColorInput, type RGBA, parseColor, rgbaToEngineColor } from "../lib/rgba";
import { StyledText, styledTextToAnsi } from "../lib/styledText";
import type { CliRenderer } from "../platform/cliRenderer";
import { Box, type BoxOptions } from "./Box";
import { RootTextNode, type TextNode } from "./TextNode";

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

export class Text extends Box {
  private _fg: RGBA | null = null;
  private _bg: RGBA | null = null;
  private _textNodeId: number;
  private _wrapMode: "none" | "char" | "word";
  private _truncate: boolean;

  /**
   * The root of the TextNode tree. All structured text attached via `add()`
   * lives here. The lifecycle pass reads `isDirty` and, when true, gathers
   * chunks from this tree and pushes them to the engine.
   */
  public readonly rootTextNode: RootTextNode;

  /**
   * Bound lifecycle pass function, registered with the renderer so it is
   * invoked once per frame. Kept as an arrow function so `unregister` works
   * correctly on cleanup.
   */
  private readonly _lifecyclePassFn: () => void;

  constructor(renderer: CliRenderer, options: TextOptions = {}) {
    _textCounter++;
    super(renderer, {
      ...options,
      id: options.id ?? `text-${_textCounter}`,
      backgroundColor: options.bg ?? options.backgroundColor,
    });

    if (options.fg) this._fg = parseColor(options.fg);
    if (options.bg) this._bg = parseColor(options.bg);
    this._wrapMode = options.wrapMode ?? "none";
    this._truncate = options.truncate ?? false;

    // Create the inner Text node in the engine
    this._textNodeId = renderer.createNode("Text");
    renderer.appendChild(this._nodeId, this._textNodeId);

    // Create the root text node — marks itself dirty on any descendant change
    this.rootTextNode = new RootTextNode({}, () => {
      // This callback fires when ANY descendant mutates; the lifecycle pass
      // will handle the actual re-push to the engine.
    });

    // Seed the root with the initial content option (if any)
    const raw = options.content ?? "";
    const initial = typeof raw === "string" ? raw : styledTextToAnsi(raw as StyledText);
    if (initial) {
      this.rootTextNode.add(initial);
    }

    this._applyTextStyle(options);
    this._syncToEngine();

    // Register per-frame lifecycle pass so dirty-tree mutations auto-sync
    this._lifecyclePassFn = () => this.onLifecyclePass();
    renderer.registerLifecyclePass(this._lifecyclePassFn);
  }

  // ── Content API ───────────────────────────────────────────────────────────

  /**
   * Attach a TextNode to the root text node.
   *
   * This is the canonical BetterTUI API (`demoText.add(containerNode)`).
   *
   * NOTE: Named `addNode` rather than `add` because `Text` extends
   * `Box` whose `add(Box)` has a different signature.
   */
  addNode(node: TextNode, index?: number): void {
    this.rootTextNode.add(node, index);
    // isDirty is propagated through the tree; lifecycle pass will sync.
  }

  /**
   * Convenience: remove a previously added TextNode from the root.
   */
  removeNode(node: TextNode): void {
    this.rootTextNode.remove(node);
  }

  /**
   * `content` setter — accepts a plain string or StyledText.
   * Replaces all children of the root text node with a single string child.
   * Kept for backward-compatibility with code that does `text.content = "..."`.
   */
  get content(): string {
    const chunks = this.rootTextNode.gatherWithInheritedStyle({});
    if (chunks.length === 0) return "";
    return styledTextToAnsi(new StyledText(chunks));
  }

  set content(value: StyledText | string | string[]) {
    const normalised = Array.isArray(value) ? value.join("\n") : value;
    const text =
      normalised instanceof StyledText ? styledTextToAnsi(normalised) : String(normalised);
    this.rootTextNode.clear();
    if (text) {
      this.rootTextNode.add(text);
    }
    // Sync immediately so single-assignment renders without waiting a frame
    this._syncToEngine();
  }

  /** Clear all text content (clears the root node tree). */
  clear(): void {
    this.rootTextNode.clear();
    this._syncToEngine();
  }

  // ── Style API ─────────────────────────────────────────────────────────────

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
    this.backgroundColor = color;
  }

  set textColor(color: ColorInput) {
    this.fg = color;
  }

  // ── Lifecycle ─────────────────────────────────────────────────────────────

  /**
   * Called once per frame by the CliRenderer lifecycle loop.
   * Checks whether the root text node is dirty; if so, re-gathers all chunks
   * from the node tree and pushes the new ANSI string to the engine.
   *
   * This is the mechanism behind BetterTUI's "mutate a node → auto-update"
   * pattern.
   */
  onLifecyclePass(): void {
    if (!this.rootTextNode.isDirty) return;
    if (this._isDestroyed) return;
    this._syncToEngine();
    this.rootTextNode.isDirty = false;
  }

  // ── Cleanup ───────────────────────────────────────────────────────────────

  override destroy(): void {
    if (this._isDestroyed) return;
    this._renderer.unregisterLifecyclePass(this._lifecyclePassFn);
    try {
      this._renderer.removeNode(this._textNodeId);
    } catch {
      // ignore
    }
    super.destroy();
  }

  // ── Internal ──────────────────────────────────────────────────────────────

  /** Push the current node-tree content to the engine as an ANSI string. */
  private _syncToEngine(): void {
    if (this._isDestroyed) return;
    const chunks = this.rootTextNode.gatherWithInheritedStyle({
      fg: this._fg ?? undefined,
      bg: this._bg ?? undefined,
    });
    const ansi = styledTextToAnsi(new StyledText(chunks));
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
}
