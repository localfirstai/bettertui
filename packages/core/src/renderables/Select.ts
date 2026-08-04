/**
 * Select — a keyboard-navigable list selector.
 */

import type { KeyEvent } from "../lib/keyHandler";
import { RenderableEvents, SelectEvents } from "../lib/renderableEvents";
import { type ColorInput, type RGBA, parseColor } from "../lib/rgba";
import type { CliRenderer } from "../platform/cliRenderer";
import { Box, type BoxOptions } from "./Box";

export interface SelectOption {
  name: string;
  description: string;
  value?: unknown;
}

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
  private _contentNodeId: number;
  private readonly _keyHandler: (key: KeyEvent) => void;

  constructor(renderer: CliRenderer, options: SelectOptions = {}) {
    _selectCounter++;
    super(renderer, {
      ...options,
      id: options.id ?? `select-${_selectCounter}`,
      focusable: true,
      overflow: "hidden",
    });

    this._selectOptions = options.options ?? [];
    this._selectedIndex = this._skipNonSelectable(options.selectedIndex ?? 0, 1);
    this._scrollOffset = 0;
    this._textColor = parseColor(options.textColor ?? "#e2e8f0");
    this._focusedTextColor = parseColor(options.focusedTextColor ?? "#f7fafc");
    this._selectedBgColor = parseColor(options.selectedBackgroundColor ?? "#3b82f6");
    this._selectedTextColor = parseColor(options.selectedTextColor ?? "#ffffff");
    this._descriptionColor = parseColor(options.descriptionColor ?? "#94a3b8");
    this._selectedDescriptionColor = parseColor(options.selectedDescriptionColor ?? "#cbd5e1");
    this._showScrollIndicator = options.showScrollIndicator ?? false;
    this._showDescription = options.showDescription !== false;
    this._showSelectionIndicator = options.showSelectionIndicator !== false;
    this._selectionIndicator = options.selectionIndicator ?? "❯ ";
    this._unselectedIndicator = options.unselectedIndicator ?? "  ";
    this._wrapSelection = options.wrapSelection ?? false;
    this._fastScrollStep = options.fastScrollStep ?? 5;
    this._itemSpacing = options.itemSpacing ?? 0;

    if (options.focusedBackgroundColor) {
      this._focusedBgColor = parseColor(options.focusedBackgroundColor);
    }

    this._contentNodeId = renderer.createNode("Text");
    renderer.appendChild(this._nodeId, this._contentNodeId);

    this._keyHandler = this._handleKey.bind(this);
    this._render();
  }

  // ── Getters/Setters ───────────────────────────────────────────────────────────

  get options(): SelectOption[] {
    return this._selectOptions;
  }

  set options(opts: SelectOption[]) {
    this._selectOptions = opts;
    const clamped = Math.min(this._selectedIndex, Math.max(0, opts.length - 1));
    this._selectedIndex = this._skipNonSelectable(clamped, 1);
    this._updateScroll();
    this._render();
  }

  get selectedIndex(): number {
    return this._selectedIndex;
  }

  set selectedIndex(idx: number) {
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
    this._showScrollIndicator = v;
    this._render();
  }

  get showDescription(): boolean {
    return this._showDescription;
  }

  set showDescription(v: boolean) {
    this._showDescription = v;
    this._render();
  }

  get wrapSelection(): boolean {
    return this._wrapSelection;
  }

  set wrapSelection(v: boolean) {
    this._wrapSelection = v;
  }

  get showSelectionIndicator(): boolean {
    return this._showSelectionIndicator;
  }

  set showSelectionIndicator(v: boolean) {
    this._showSelectionIndicator = v;
    this._render();
  }

  get selectionIndicator(): string {
    return this._selectionIndicator;
  }

  set selectionIndicator(v: string) {
    this._selectionIndicator = v;
    this._render();
  }

  get unselectedIndicator(): string {
    return this._unselectedIndicator;
  }

  set unselectedIndicator(v: string) {
    this._unselectedIndicator = v;
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
    if (this._focused) this._render();
  }

  set descriptionColor(color: ColorInput) {
    this._descriptionColor = parseColor(color);
    this._render();
  }

  set selectedDescriptionColor(color: ColorInput) {
    this._selectedDescriptionColor = parseColor(color);
    this._render();
  }

  // ── Methods ───────────────────────────────────────────────────────────────────

  getSelectedOption(): SelectOption | undefined {
    return this._selectOptions[this._selectedIndex];
  }

  getSelectedIndex(): number {
    return this._selectedIndex;
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
    if (this._wrapSelection) {
      next =
        ((next % this._selectOptions.length) + this._selectOptions.length) %
        this._selectOptions.length;
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
    if (this._wrapSelection) {
      next = next % this._selectOptions.length;
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

  private _isNonSelectable(index: number): boolean {
    const opt = this._selectOptions[index];
    if (!opt) return true;
    const kind = (opt.value as { kind?: string } | undefined)?.kind;
    return kind === "spacer" || kind === "category";
  }

  private _skipNonSelectable(index: number, direction: 1 | -1): number {
    const len = this._selectOptions.length;
    let i = index;
    let attempts = 0;
    while (this._isNonSelectable(i) && attempts < len) {
      if (this._wrapSelection) {
        i = (((i + direction) % len) + len) % len;
      } else {
        i += direction;
        if (i < 0 || i >= len) {
          return this._selectedIndex; // stay put if no selectable item found
        }
      }
      attempts++;
    }
    return attempts >= len ? this._selectedIndex : i;
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

  // ── Key handling ──────────────────────────────────────────────────────────────

  private _handleKey(key: KeyEvent): void {
    if (!this._focused || this._isDestroyed) return;

    if (key.name === "up" || key.name === "k") {
      this.moveUp(key.shift ? this._fastScrollStep : 1);
    } else if (key.name === "down" || key.name === "j") {
      this.moveDown(key.shift ? this._fastScrollStep : 1);
    } else if (key.name === "return" || key.name === "linefeed" || key.name === "enter") {
      this.selectCurrent();
    } else if (key.name === "pageup") {
      this.moveUp(this._fastScrollStep * 2);
    } else if (key.name === "pagedown") {
      this.moveDown(this._fastScrollStep * 2);
    } else if (key.name === "home") {
      this.selectedIndex = 0;
    } else if (key.name === "end") {
      this.selectedIndex = this._selectOptions.length - 1;
    }
  }

  // ── Rendering ─────────────────────────────────────────────────────────────────

  private _clampIndex(idx: number): number {
    return Math.max(0, Math.min(this._selectOptions.length - 1, idx));
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
      const opt = this._selectOptions[i];
      const kind = (opt?.value as { kind?: string } | undefined)?.kind;
      const h =
        kind === "spacer" || kind === "category" || !this._showDescription
          ? 1
          : 2 + this._itemSpacing;
      linesUsed += h;
    }

    while (linesUsed > viewHeight && this._scrollOffset < this._selectedIndex) {
      const firstOpt = this._selectOptions[this._scrollOffset];
      const kind = (firstOpt?.value as { kind?: string } | undefined)?.kind;
      const firstH =
        kind === "spacer" || kind === "category" || !this._showDescription
          ? 1
          : 2 + this._itemSpacing;
      linesUsed -= firstH;
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
      const opt = this._selectOptions[i];
      const kind = (opt?.value as { kind?: string } | undefined)?.kind;
      totalContentLines +=
        kind === "spacer" || kind === "category" || !this._showDescription
          ? 1
          : 2 + this._itemSpacing;
    }

    this._scrollOffset = Math.max(0, Math.min(this._scrollOffset, totalItems - 1));

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
        rawLines.push({ text: "", fg: "0;0;0" });
        currIdx++;
        continue;
      }

      if (kind === "category") {
        const catColor = `${this._textColor.r};${this._textColor.g};${this._textColor.b}`;
        rawLines.push({
          text: `  ${opt.name}`,
          fg: catColor,
          isCategory: true,
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
        ? `${this._selectedBgColor.r};${this._selectedBgColor.g};${this._selectedBgColor.b}`
        : undefined;

      rawLines.push({ text: indicator + opt.name, bg, fg: tc });

      if (this._showDescription && opt.description && rawLines.length < viewHeight) {
        const descColor = isSelected ? this._selectedDescriptionColor : this._descriptionColor;
        const dc = `${descColor.r};${descColor.g};${descColor.b}`;
        const trimmedDesc = opt.description.trim();
        if (trimmedDesc) {
          // Indent matches the indicator width so description text starts at
          // the same column as the name. When showSelectionIndicator is false
          // the indent is "" — fully left-aligned with no leading spaces.
          const indent = this._showSelectionIndicator
            ? " ".repeat(this._selectionIndicator.length)
            : "";
          rawLines.push({ text: `${indent}${trimmedDesc}`, bg, fg: dc });
        }
      }

      for (let s = 0; s < this._itemSpacing && rawLines.length < viewHeight; s++) {
        rawLines.push({ text: "", fg: "0;0;0" });
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
      const opt = this._selectOptions[i];
      const kind = (opt?.value as { kind?: string } | undefined)?.kind;
      linesBeforeScrollOffset +=
        kind === "spacer" || kind === "category" || !this._showDescription
          ? 1
          : 2 + this._itemSpacing;
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
          lineText = `\x1b[1;38;2;${item.fg}m${item.text.padEnd(contentWidth)}\x1b[0m`;
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
