/**
 * Select — a keyboard-navigable list selector.
 *
 * Renders a scrollable option list with selection highlighting, optional
 * descriptions and a scroll indicator. Navigation is driven by a
 * configurable keybinding map (defaults + user overrides + aliases), mirroring
 * the OpenTUI Select pattern: `handleKeyPress` resolves a key event to an
 * action and returns whether it was consumed.
 */

import type { KeyEvent } from "../lib/keyHandler";
import { RenderableEvents, SelectEvents } from "../lib/renderableEvents";
import {
  type KeyAliasMap,
  type KeyBinding,
  buildKeyBindingsMap,
  defaultKeyAliases,
  getKeyBindingAction,
  mergeKeyAliases,
  mergeKeyBindings,
} from "../lib/renderableKeyBindings";
import { type ColorInput, type RGBA, parseColor } from "../lib/rgba";
import type { CliRenderer } from "../platform/cliRenderer";
import { Box, type BoxOptions } from "./Box";

export interface SelectOption {
  name: string;
  description: string;
  value?: unknown;
}

/** Selectable actions resolved from key events. */
export type SelectAction =
  | "move-up"
  | "move-down"
  | "move-up-fast"
  | "move-down-fast"
  | "move-up-page"
  | "move-down-page"
  | "move-to-start"
  | "move-to-end"
  | "select-current";

export type SelectKeyBinding = KeyBinding<SelectAction>;

const defaultSelectKeybindings: SelectKeyBinding[] = [
  { name: "up", action: "move-up" },
  { name: "k", action: "move-up" },
  { name: "down", action: "move-down" },
  { name: "j", action: "move-down" },
  { name: "up", shift: true, action: "move-up-fast" },
  { name: "down", shift: true, action: "move-down-fast" },
  { name: "pageup", action: "move-up-page" },
  { name: "pagedown", action: "move-down-page" },
  { name: "home", action: "move-to-start" },
  { name: "end", action: "move-to-end" },
  { name: "return", action: "select-current" },
  { name: "linefeed", action: "select-current" },
  { name: "enter", action: "select-current" },
];

export interface SelectOptions extends BoxOptions {
  options?: SelectOption[];
  selectedIndex?: number;
  backgroundColor?: ColorInput;
  textColor?: ColorInput;
  focusedBackgroundColor?: ColorInput;
  focusedTextColor?: ColorInput;
  selectedBackgroundColor?: ColorInput;
  selectedTextColor?: ColorInput;
  descriptionColor?: ColorInput;
  selectedDescriptionColor?: ColorInput;
  showScrollIndicator?: boolean;
  showDescription?: boolean;
  showSelectionIndicator?: boolean;
  selectionIndicator?: string;
  unselectedIndicator?: string;
  wrapSelection?: boolean;
  fastScrollStep?: number;
  itemSpacing?: number;
  keyBindings?: SelectKeyBinding[];
  keyAliasMap?: KeyAliasMap;
}

export type SelectRenderableOptions = SelectOptions;

let _selectCounter = 0;

export class Select extends Box {
  private _selectOptions: SelectOption[];
  private _selectedIndex: number;
  private _scrollOffset: number;
  private _textColor: RGBA;
  private _focusedTextColor: RGBA;
  private _selectedBgColor: RGBA;
  private _selectedTextColor: RGBA;
  private _descriptionColor: RGBA;
  private _selectedDescriptionColor: RGBA;
  private _focusedBgColor: RGBA | null = null;
  private _showScrollIndicator: boolean;
  private _showDescription: boolean;
  private _showSelectionIndicator: boolean;
  private _selectionIndicator: string;
  private _unselectedIndicator: string;
  private _wrapSelection: boolean;
  private _fastScrollStep: number;
  private _itemSpacing: number;
  private _keyBindings: SelectKeyBinding[];
  private _keyAliasMap: KeyAliasMap;
  private _keyBindingsMap: Map<string, SelectAction>;
  private _contentNodeId: number;
  private readonly _keyHandler: (key: KeyEvent) => void;

  protected _defaultOptions = {
    textColor: "#e2e8f0",
    focusedTextColor: "#f7fafc",
    selectedBackgroundColor: "#3b82f6",
    selectedTextColor: "#ffffff",
    descriptionColor: "#94a3b8",
    selectedDescriptionColor: "#cbd5e1",
    showScrollIndicator: false,
    showDescription: true,
    showSelectionIndicator: true,
    selectionIndicator: "❯ ",
    unselectedIndicator: "  ",
    wrapSelection: false,
    fastScrollStep: 5,
    itemSpacing: 0,
  } satisfies Partial<SelectOptions>;

  constructor(renderer: CliRenderer, options: SelectOptions = {}) {
    _selectCounter++;
    super(renderer, {
      ...options,
      id: options.id ?? `select-${_selectCounter}`,
      focusable: true,
      overflow: "hidden",
    });

    this._selectOptions = options.options ?? [];
    this._selectedIndex = this._resolveInitialIndex(options.selectedIndex ?? 0);
    this._scrollOffset = 0;
    this._textColor = parseColor(options.textColor ?? this._defaultOptions.textColor);
    this._focusedTextColor = parseColor(
      options.focusedTextColor ?? this._defaultOptions.focusedTextColor,
    );
    this._selectedBgColor = parseColor(
      options.selectedBackgroundColor ?? this._defaultOptions.selectedBackgroundColor,
    );
    this._selectedTextColor = parseColor(
      options.selectedTextColor ?? this._defaultOptions.selectedTextColor,
    );
    this._descriptionColor = parseColor(
      options.descriptionColor ?? this._defaultOptions.descriptionColor,
    );
    this._selectedDescriptionColor = parseColor(
      options.selectedDescriptionColor ?? this._defaultOptions.selectedDescriptionColor,
    );
    this._showScrollIndicator =
      options.showScrollIndicator ?? this._defaultOptions.showScrollIndicator;
    this._showDescription = options.showDescription ?? this._defaultOptions.showDescription;
    this._showSelectionIndicator =
      options.showSelectionIndicator ?? this._defaultOptions.showSelectionIndicator;
    this._selectionIndicator =
      options.selectionIndicator ?? this._defaultOptions.selectionIndicator;
    this._unselectedIndicator =
      options.unselectedIndicator ?? this._defaultOptions.unselectedIndicator;
    this._wrapSelection = options.wrapSelection ?? this._defaultOptions.wrapSelection;
    this._fastScrollStep = options.fastScrollStep ?? this._defaultOptions.fastScrollStep;
    this._itemSpacing = options.itemSpacing ?? this._defaultOptions.itemSpacing;

    if (options.focusedBackgroundColor) {
      this._focusedBgColor = parseColor(options.focusedBackgroundColor);
    }

    this._keyAliasMap = mergeKeyAliases(defaultKeyAliases, options.keyAliasMap ?? {});
    this._keyBindings = options.keyBindings ?? [];
    this._keyBindingsMap = buildKeyBindingsMap(
      mergeKeyBindings(defaultSelectKeybindings, this._keyBindings),
      this._keyAliasMap,
    );

    this._contentNodeId = renderer.createNode("Text");
    renderer.appendChild(this._nodeId, this._contentNodeId);

    this._keyHandler = (key: KeyEvent) => {
      if (!this._focused || this._isDestroyed) return;
      if (this.handleKeyPress(key)) {
        key.stopPropagation();
      }
    };
    this._render();
  }

  // ── Getters/Setters ───────────────────────────────────────────────────────────

  get options(): SelectOption[] {
    return this._selectOptions;
  }

  set options(opts: SelectOption[]) {
    this._selectOptions = opts;
    if (opts.length === 0) {
      this._selectedIndex = 0;
    } else {
      const clamped = Math.min(this._selectedIndex, opts.length - 1);
      this._selectedIndex = this._skipNonSelectable(clamped, 1);
    }
    this._scrollOffset = 0;
    this._updateScroll();
    this._render();
  }

  get selectedIndex(): number {
    return this._selectedIndex;
  }

  set selectedIndex(idx: number) {
    if (this._selectOptions.length === 0) return;
    const clamped = this._clampIndex(idx);
    const validIndex = this._skipNonSelectable(clamped, idx >= this._selectedIndex ? 1 : -1);
    if (validIndex !== this._selectedIndex) {
      this._selectedIndex = validIndex;
      this._updateScroll();
      this._render();
    }
  }

  get showScrollIndicator(): boolean {
    return this._showScrollIndicator;
  }

  set showScrollIndicator(v: boolean) {
    if (this._showScrollIndicator !== v) {
      this._showScrollIndicator = v;
      this._render();
    }
  }

  get showDescription(): boolean {
    return this._showDescription;
  }

  set showDescription(v: boolean) {
    if (this._showDescription !== v) {
      this._showDescription = v;
      this._updateScroll();
      this._render();
    }
  }

  get wrapSelection(): boolean {
    return this._wrapSelection;
  }

  set wrapSelection(v: boolean) {
    if (this._wrapSelection !== v) {
      this._wrapSelection = v;
      this._render();
    }
  }

  get showSelectionIndicator(): boolean {
    return this._showSelectionIndicator;
  }

  set showSelectionIndicator(v: boolean) {
    if (this._showSelectionIndicator !== v) {
      this._showSelectionIndicator = v;
      this._render();
    }
  }

  get selectionIndicator(): string {
    return this._selectionIndicator;
  }

  set selectionIndicator(v: string) {
    if (this._selectionIndicator !== v) {
      this._selectionIndicator = v;
      this._render();
    }
  }

  get unselectedIndicator(): string {
    return this._unselectedIndicator;
  }

  set unselectedIndicator(v: string) {
    if (this._unselectedIndicator !== v) {
      this._unselectedIndicator = v;
      this._render();
    }
  }

  get fastScrollStep(): number {
    return this._fastScrollStep;
  }

  set fastScrollStep(v: number) {
    this._fastScrollStep = Math.max(1, Math.floor(v));
  }

  get focusedBackgroundColor(): RGBA | null {
    return this._focusedBgColor;
  }

  set focusedBackgroundColor(color: ColorInput) {
    this._focusedBgColor = parseColor(color);
    this._render();
  }

  set selectedBackgroundColor(color: ColorInput) {
    this._selectedBgColor = parseColor(color);
    this._render();
  }

  set textColor(color: ColorInput) {
    this._textColor = parseColor(color);
    this._render();
  }

  set selectedTextColor(color: ColorInput) {
    this._selectedTextColor = parseColor(color);
    this._render();
  }

  set focusedTextColor(color: ColorInput) {
    this._focusedTextColor = parseColor(color);
    this._render();
  }

  set descriptionColor(color: ColorInput) {
    this._descriptionColor = parseColor(color);
    this._render();
  }

  set selectedDescriptionColor(color: ColorInput) {
    this._selectedDescriptionColor = parseColor(color);
    this._render();
  }

  set keyBindings(bindings: SelectKeyBinding[]) {
    this._keyBindings = bindings;
    this._keyBindingsMap = buildKeyBindingsMap(
      mergeKeyBindings(defaultSelectKeybindings, bindings),
      this._keyAliasMap,
    );
  }

  set keyAliasMap(aliases: KeyAliasMap) {
    this._keyAliasMap = mergeKeyAliases(defaultKeyAliases, aliases);
    this._keyBindingsMap = buildKeyBindingsMap(
      mergeKeyBindings(defaultSelectKeybindings, this._keyBindings),
      this._keyAliasMap,
    );
  }

  // ── Methods ───────────────────────────────────────────────────────────────────

  getSelectedOption(): SelectOption | undefined {
    return this._selectOptions[this._selectedIndex];
  }

  getSelectedIndex(): number {
    return this._selectedIndex;
  }

  /** Programmatically move the selection; emits SELECTION_CHANGED on change. */
  setSelectedIndex(index: number): void {
    if (this._selectOptions.length === 0) return;
    const clamped = this._clampIndex(index);
    const validIndex = this._skipNonSelectable(clamped, index >= this._selectedIndex ? 1 : -1);
    if (validIndex !== this._selectedIndex) {
      this._selectedIndex = validIndex;
      this._updateScroll();
      this._render();
      const opt = this.getSelectedOption();
      if (opt) this.emit(SelectEvents.SELECTION_CHANGED, this._selectedIndex, opt);
    }
  }

  selectCurrent(): void {
    const opt = this.getSelectedOption();
    if (opt) {
      this.emit(SelectEvents.ITEM_SELECTED, this._selectedIndex, opt);
    }
  }

  moveUp(steps = 1): void {
    const prev = this._selectedIndex;
    let next = this._selectedIndex - steps;
    if (this._selectOptions.length === 0) return;
    if (this._wrapSelection) {
      next = this._wrapIndex(next);
    } else {
      next = Math.max(0, next);
    }
    next = this._skipNonSelectable(next, -1);
    if (next !== prev) {
      this._selectedIndex = next;
      this._updateScroll();
      this._render();
      const opt = this.getSelectedOption();
      if (opt) this.emit(SelectEvents.SELECTION_CHANGED, next, opt);
    }
  }

  moveDown(steps = 1): void {
    const prev = this._selectedIndex;
    let next = this._selectedIndex + steps;
    if (this._selectOptions.length === 0) return;
    if (this._wrapSelection) {
      next = this._wrapIndex(next);
    } else {
      next = Math.min(this._selectOptions.length - 1, next);
    }
    next = this._skipNonSelectable(next, 1);
    if (next !== prev) {
      this._selectedIndex = next;
      this._updateScroll();
      this._render();
      const opt = this.getSelectedOption();
      if (opt) this.emit(SelectEvents.SELECTION_CHANGED, next, opt);
    }
  }

  /**
   * Resolve a key event against the keybinding map and dispatch the action.
   * Returns `true` when the key was consumed.
   */
  handleKeyPress(key: KeyEvent): boolean {
    if (this._isDestroyed) return false;
    const action = getKeyBindingAction(this._keyBindingsMap, key);
    if (!action) return false;

    switch (action) {
      case "move-up":
        this.moveUp(1);
        break;
      case "move-down":
        this.moveDown(1);
        break;
      case "move-up-fast":
        this.moveUp(this._fastScrollStep);
        break;
      case "move-down-fast":
        this.moveDown(this._fastScrollStep);
        break;
      case "move-up-page":
        this.moveUp(this._fastScrollStep * 2);
        break;
      case "move-down-page":
        this.moveDown(this._fastScrollStep * 2);
        break;
      case "move-to-start":
        this.setSelectedIndex(0);
        break;
      case "move-to-end":
        this.setSelectedIndex(this._selectOptions.length - 1);
        break;
      case "select-current":
        this.selectCurrent();
        break;
    }

    return true;
  }

  // ── Focus ─────────────────────────────────────────────────────────────────────

  override focus(): void {
    if (this._isDestroyed || this._focused) return;
    this._focused = true;
    this._renderer.keyHandler.offInternal("keypress", this._keyHandler);
    this._renderer.keyHandler.onInternal("keypress", this._keyHandler);
    this._render();
    this.emit(RenderableEvents.FOCUSED, this);
  }

  override blur(): void {
    if (this._isDestroyed) return;
    this._renderer.keyHandler.offInternal("keypress", this._keyHandler);
    if (!this._focused) return;
    this._focused = false;
    this._render();
    this.emit(RenderableEvents.BLURRED, this);
  }

  // ── Rendering ─────────────────────────────────────────────────────────────────

  private _resolveInitialIndex(requested: number): number {
    if (this._selectOptions.length === 0) return 0;
    const clamped = this._clampIndex(requested);
    return this._skipNonSelectable(clamped, 1, clamped);
  }

  private _clampIndex(idx: number): number {
    return Math.max(0, Math.min(this._selectOptions.length - 1, idx));
  }

  private _wrapIndex(idx: number): number {
    const len = this._selectOptions.length;
    if (len === 0) return 0;
    return ((idx % len) + len) % len;
  }

  private _isNonSelectable(index: number): boolean {
    const opt = this._selectOptions[index];
    if (!opt) return true;
    const kind = (opt.value as { kind?: string } | undefined)?.kind;
    return kind === "spacer" || kind === "category";
  }

  private _skipNonSelectable(index: number, direction: 1 | -1, fallback?: number): number {
    const len = this._selectOptions.length;
    const stayPut = fallback ?? this._selectedIndex;
    if (len === 0) return stayPut;
    let i = index;
    let attempts = 0;
    while (this._isNonSelectable(i) && attempts < len) {
      if (this._wrapSelection) {
        i = this._wrapIndex(i + direction);
      } else {
        i += direction;
        if (i < 0 || i >= len) {
          return stayPut;
        }
      }
      attempts++;
    }
    return attempts >= len ? stayPut : i;
  }

  /** Number of rendered rows a single option occupies. */
  private _linesPerItem(index: number): number {
    const opt = this._selectOptions[index];
    if (!opt) return 1;
    const kind = (opt.value as { kind?: string } | undefined)?.kind;
    if (kind === "spacer" || kind === "category") return 1;
    return (this._showDescription && opt.description ? 2 : 1) + this._itemSpacing;
  }

  private _updateScroll(): void {
    if (this._selectOptions.length === 0) return;

    const viewHeight = this._getViewHeight();
    this._selectedIndex = Math.max(
      0,
      Math.min(this._selectOptions.length - 1, this._selectedIndex),
    );
    this._selectedIndex = this._skipNonSelectable(this._selectedIndex, 1);

    if (this._selectedIndex < this._scrollOffset) {
      let targetOffset = this._selectedIndex;
      while (targetOffset > 0 && this._isNonSelectable(targetOffset - 1)) {
        targetOffset--;
      }
      this._scrollOffset = targetOffset;
      return;
    }

    let linesUsed = 0;
    for (let i = this._scrollOffset; i <= this._selectedIndex; i++) {
      linesUsed += this._linesPerItem(i);
    }

    while (linesUsed > viewHeight && this._scrollOffset < this._selectedIndex) {
      linesUsed -= this._linesPerItem(this._scrollOffset);
      this._scrollOffset++;
    }
  }

  private _getViewHeight(): number {
    const h = this._options.height;
    if (typeof h === "number") return h;

    let available = this._renderer.viewportHeight;

    if (this._options.border) {
      available -= 2;
    }
    if (typeof this._options.marginTop === "number") available -= this._options.marginTop;
    if (typeof this._options.marginBottom === "number") available -= this._options.marginBottom;

    let current: Box | null = this._parent;
    while (current) {
      const opts = current.boxOptions;
      if (typeof opts.height === "number") {
        available = Math.min(available, opts.height);
      }

      if (opts.border) {
        available -= 2;
      }

      if (typeof opts.padding === "number") {
        available -= opts.padding * 2;
      } else {
        if (typeof opts.paddingTop === "number") available -= opts.paddingTop;
        if (typeof opts.paddingBottom === "number") available -= opts.paddingBottom;
      }

      if (typeof opts.margin === "number") {
        available -= opts.margin * 2;
      } else {
        if (typeof opts.marginTop === "number") available -= opts.marginTop;
        if (typeof opts.marginBottom === "number") available -= opts.marginBottom;
      }

      const dir = opts.flexDirection ?? "column";
      if (dir === "column") {
        for (const child of current.getChildren()) {
          if (child === this || child.getRenderable(this.id)) continue;
          if (!child.boxOptions.flexGrow) {
            available -= child.getEstimatedHeight();
          }
        }
      }

      current = current.parent;
    }

    return Math.max(3, available);
  }

  private _render(): void {
    if (this._isDestroyed) return;

    const viewHeight = this._getViewHeight();
    const totalItems = this._selectOptions.length;

    let totalContentLines = 0;
    for (let i = 0; i < totalItems; i++) {
      totalContentLines += this._linesPerItem(i);
    }

    this._scrollOffset = Math.max(0, Math.min(this._scrollOffset, Math.max(0, totalItems - 1)));

    const rawLines: {
      text: string;
      bg?: string;
      fg: string;
      isCategory?: boolean;
    }[] = [];
    let currIdx = this._scrollOffset;

    while (currIdx < totalItems && rawLines.length < viewHeight) {
      const opt = this._selectOptions[currIdx];
      if (!opt) {
        currIdx++;
        continue;
      }

      const kind = (opt.value as { kind?: string } | undefined)?.kind;
      if (kind === "spacer") {
        rawLines.push({
          text: "",
          fg: "0;0;0",
          bg: this._focused && this._focusedBgColor ? this._ansi(this._focusedBgColor) : undefined,
        });
        currIdx++;
        continue;
      }

      if (kind === "category") {
        const catColor = `${this._textColor.r};${this._textColor.g};${this._textColor.b}`;
        rawLines.push({
          text: opt.name,
          fg: catColor,
          isCategory: true,
          bg: this._focused && this._focusedBgColor ? this._ansi(this._focusedBgColor) : undefined,
        });
        currIdx++;
        continue;
      }

      const isSelected = currIdx === this._selectedIndex;
      const textColor = isSelected
        ? this._selectedTextColor
        : this._focused
          ? this._focusedTextColor
          : this._textColor;

      const tc = `${textColor.r};${textColor.g};${textColor.b}`;
      const indicator = this._showSelectionIndicator
        ? isSelected
          ? this._selectionIndicator
          : this._unselectedIndicator
        : "";

      const bg = isSelected
        ? this._ansi(this._selectedBgColor)
        : this._focused && this._focusedBgColor
          ? this._ansi(this._focusedBgColor)
          : undefined;

      rawLines.push({ text: indicator + opt.name, bg, fg: tc });

      if (this._showDescription && opt.description && rawLines.length < viewHeight) {
        const descColor = isSelected ? this._selectedDescriptionColor : this._descriptionColor;
        const dc = `${descColor.r};${descColor.g};${descColor.b}`;
        const trimmedDesc = opt.description.trim();
        if (trimmedDesc) {
          const descIndent = this._showSelectionIndicator
            ? " ".repeat(Select.displayWidth(indicator))
            : "";
          rawLines.push({ text: `${descIndent}${trimmedDesc}`, bg, fg: dc });
        }
      }

      for (let s = 0; s < this._itemSpacing && rawLines.length < viewHeight; s++) {
        rawLines.push({ text: "", bg, fg: "0;0;0" });
      }

      currIdx++;
    }

    const hasScrollbar = this._showScrollIndicator && totalContentLines > viewHeight;
    const rowWidth = Math.max(40, this._renderer.terminalWidth - 4);
    const contentWidth = hasScrollbar ? rowWidth - 1 : rowWidth;

    const trackHeight = viewHeight;
    const visibleRatio = Math.min(1, viewHeight / Math.max(1, totalContentLines));
    const thumbSize = Math.max(1, Math.round(visibleRatio * trackHeight));
    const maxThumbPos = Math.max(0, trackHeight - thumbSize);

    let linesBeforeScrollOffset = 0;
    for (let i = 0; i < this._scrollOffset; i++) {
      linesBeforeScrollOffset += this._linesPerItem(i);
    }

    const maxScrollableLines = Math.max(1, totalContentLines - viewHeight);
    const scrollRatio = Math.min(1, Math.max(0, linesBeforeScrollOffset / maxScrollableLines));
    const thumbPos = Math.min(maxThumbPos, Math.round(scrollRatio * maxThumbPos));

    const lines: string[] = [];

    for (let lineIdx = 0; lineIdx < viewHeight; lineIdx++) {
      const item = rawLines[lineIdx];
      let lineText = "";

      if (item) {
        if (item.isCategory) {
          if (item.bg) {
            lineText = `\x1b[48;2;${item.bg}m\x1b[1;38;2;${item.fg}m${item.text.padEnd(contentWidth)}\x1b[0m`;
          } else {
            lineText = `\x1b[1;38;2;${item.fg}m${item.text.padEnd(contentWidth)}\x1b[0m`;
          }
        } else if (item.bg) {
          lineText = `\x1b[48;2;${item.bg}m\x1b[38;2;${item.fg}m${item.text.padEnd(contentWidth)}\x1b[0m`;
        } else {
          lineText = `\x1b[38;2;${item.fg}m${item.text.padEnd(contentWidth)}\x1b[0m`;
        }
      } else {
        lineText = "".padEnd(contentWidth);
      }

      if (hasScrollbar) {
        const isThumb = lineIdx >= thumbPos && lineIdx < thumbPos + thumbSize;
        if (isThumb) {
          const thumbFg = `${this._descriptionColor.r};${this._descriptionColor.g};${this._descriptionColor.b}`;
          lineText += `\x1b[38;2;${thumbFg}m█\x1b[0m`;
        } else {
          lineText += " ";
        }
      }

      lines.push(lineText);
    }

    this._renderer.setText(this._contentNodeId, lines.join("\n"));
  }

  private _ansi(color: RGBA): string {
    return `${color.r};${color.g};${color.b}`;
  }

  private static displayWidth(text: string): number {
    let width = 0;
    for (const char of text) {
      const cp = char.codePointAt(0);
      if (cp === undefined) continue;
      if (
        (cp >= 0x1100 && cp <= 0x115f) ||
        (cp >= 0x2e80 && cp <= 0xa4cf && cp !== 0x303f) ||
        (cp >= 0xac00 && cp <= 0xd7a3) ||
        (cp >= 0xf900 && cp <= 0xfaff) ||
        (cp >= 0xfe10 && cp <= 0xfe19) ||
        (cp >= 0xfe30 && cp <= 0xfe6f) ||
        (cp >= 0xff01 && cp <= 0xff60) ||
        (cp >= 0xffe0 && cp <= 0xffe6) ||
        (cp >= 0x20000 && cp <= 0x2fffd) ||
        (cp >= 0x30000 && cp <= 0x3fffd)
      ) {
        width += 2;
      } else {
        width += 1;
      }
    }
    return width;
  }

  override destroy(): void {
    if (this._isDestroyed) return;
    this._renderer.keyHandler.offInternal("keypress", this._keyHandler);
    try {
      this._renderer.removeNode(this._contentNodeId);
    } catch {
      // ignore
    }
    super.destroy();
  }
}

export { SelectEvents };
