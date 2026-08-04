/**
 * Select — a keyboard-navigable list selector.
 */

import { RenderableEvents, SelectEvents } from "../lib/renderableEvents";
import { type ColorInput, type RGBA, parseColor } from "../lib/rgba";
import type { CliRenderer } from "../platform/cliRenderer";
import type { RawKeyEvent } from "../platform/cliRenderer";
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
  private _wrapSelection: boolean;
  private _fastScrollStep: number;
  private _itemSpacing: number;
  private _contentNodeId: number;
  private readonly _keyHandler: (key: RawKeyEvent) => void;

  constructor(renderer: CliRenderer, options: SelectOptions = {}) {
    _selectCounter++;
    super(renderer, {
      ...options,
      id: options.id ?? `select-${_selectCounter}`,
      focusable: true,
      overflow: "hidden",
    });

    this._selectOptions = options.options ?? [];
    this._selectedIndex = options.selectedIndex ?? 0;
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
    this._selectedIndex = Math.min(this._selectedIndex, Math.max(0, opts.length - 1));
    this._updateScroll();
    this._render();
  }

  get selectedIndex(): number {
    return this._selectedIndex;
  }

  set selectedIndex(idx: number) {
    const clamped = this._clampIndex(idx);
    if (clamped !== this._selectedIndex) {
      this._selectedIndex = clamped;
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
    if (this._isDestroyed) return;
    this._focused = true;
    this._render();
    this.emit(RenderableEvents.FOCUSED, this);
    this._renderer.keyInput.on("keypress", this._keyHandler);
  }

  override blur(): void {
    if (this._isDestroyed) return;
    this._renderer.keyInput.off("keypress", this._keyHandler);
    this._focused = false;
    this._render();
    this.emit(RenderableEvents.BLURRED, this);
  }

  // ── Key handling ──────────────────────────────────────────────────────────────

  private _handleKey(key: RawKeyEvent): void {
    if (!this._focused || this._isDestroyed) return;

    if (key.name === "up" || key.name === "k") {
      this.moveUp(key.shift ? this._fastScrollStep : 1);
    } else if (key.name === "down" || key.name === "j") {
      this.moveDown(key.shift ? this._fastScrollStep : 1);
    } else if (key.name === "return" || key.name === "linefeed") {
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
    // Simple scroll: keep selected item in view (center when possible)
    const viewHeight = this._getViewHeight();
    const halfView = Math.floor(viewHeight / 2);
    const maxOffset = Math.max(0, this._selectOptions.length - viewHeight);
    this._scrollOffset = Math.max(0, Math.min(this._selectedIndex - halfView, maxOffset));
  }

  private _getViewHeight(): number {
    // Estimate from layout options
    const h = this._options.height;
    if (typeof h === "number") return h;
    return 10; // fallback
  }

  private _render(): void {
    if (this._isDestroyed) return;

    const lines: string[] = [];
    const viewHeight = this._getViewHeight();
    const linesPerItem = (this._showDescription ? 2 : 1) + this._itemSpacing;
    const visibleCount = Math.floor(viewHeight / linesPerItem);

    const start = this._scrollOffset;
    const end = Math.min(start + visibleCount, this._selectOptions.length);
    const rowWidth = Math.max(40, this._renderer.terminalWidth - 4);

    for (let i = start; i < end; i++) {
      const opt = this._selectOptions[i];
      if (!opt) continue;

      // Handle spacer and category menu options
      const kind = (opt.value as { kind?: string } | undefined)?.kind;
      if (kind === "spacer") {
        lines.push("");
        continue;
      }

      if (kind === "category") {
        lines.push(`\x1b[1;38;2;255;255;255m${opt.name}\x1b[0m`);
        continue;
      }

      const isSelected = i === this._selectedIndex;

      const textColor = isSelected
        ? this._selectedTextColor
        : this._focused
          ? this._focusedTextColor
          : this._textColor;

      const tc = `${textColor.r};${textColor.g};${textColor.b}`;
      const indicator = this._showSelectionIndicator ? (isSelected ? "▶ " : "  ") : "";

      let line: string;
      if (isSelected) {
        const bg = `${this._selectedBgColor.r};${this._selectedBgColor.g};${this._selectedBgColor.b}`;
        const titleText = (indicator + opt.name).padEnd(rowWidth);
        line = `\x1b[48;2;${bg}m\x1b[38;2;${tc}m${titleText}\x1b[0m`;
      } else {
        line = `\x1b[38;2;${tc}m${indicator}${opt.name}\x1b[0m`;
      }
      lines.push(line);

      if (this._showDescription) {
        const descColor = isSelected ? this._selectedDescriptionColor : this._descriptionColor;
        const dc = `${descColor.r};${descColor.g};${descColor.b}`;
        if (isSelected) {
          const bg = `${this._selectedBgColor.r};${this._selectedBgColor.g};${this._selectedBgColor.b}`;
          const descText = opt.description.padEnd(rowWidth);
          const desc = `\x1b[48;2;${bg}m\x1b[38;2;${dc}m${descText}\x1b[0m`;
          lines.push(desc);
        } else {
          const desc = `\x1b[38;2;${dc}m${opt.description}\x1b[0m`;
          lines.push(desc);
        }
      }

      // Item spacing
      for (let s = 0; s < this._itemSpacing; s++) {
        lines.push("");
      }
    }

    this._renderer.setText(this._contentNodeId, lines.join("\n"));
  }

  override destroy(): void {
    if (this._isDestroyed) return;
    this._renderer.keyInput.off("keypress", this._keyHandler);
    try {
      this._renderer.removeNode(this._contentNodeId);
    } catch {
      // ignore
    }
    super.destroy();
  }
}

export { SelectEvents };
