/**
 * TabSelect — a horizontal tab navigation widget.
 */

import type { KeyEvent } from "../lib/keyHandler";
import { RenderableEvents, TabSelectEvents } from "../lib/renderableEvents";
import { type ColorInput, type RGBA, parseColor } from "../lib/rgba";
import type { CliRenderer } from "../platform/cliRenderer";
import { Box, type BoxOptions } from "./Box";

export interface TabOption {
  name: string;
  description?: string;
  value?: unknown;
}

export interface TabSelectOptions extends BoxOptions {
  options?: TabOption[];
  selectedIndex?: number;
  /** Fixed width for each tab in characters. Set to 0 for auto-width based on content. */
  tabWidth?: number;
  /** Minimum width for auto-sized tabs. Ignored when tabWidth > 0. */
  minTabWidth?: number;
  /** Padding added to each side of tab text in auto mode. Default: 2. */
  tabPadding?: number;
  /** Gap between tabs in characters. Default: 1. */
  tabGap?: number;
  showDescription?: boolean;
  showUnderline?: boolean;
  showScrollArrows?: boolean;
  scrollArrowLeft?: string;
  scrollArrowRight?: string;
  wrapSelection?: boolean;
  backgroundColor?: ColorInput;
  textColor?: ColorInput;
  selectedTextColor?: ColorInput;
  selectedBackgroundColor?: ColorInput;
  activeUnderlineColor?: ColorInput;
  inactiveUnderlineColor?: ColorInput;
  descriptionColor?: ColorInput;
}

export type TabSelectRenderableOptions = TabSelectOptions;

let _tabSelectCounter = 0;

/** Calculate display widths for each tab based on content and options. */
function calculateTabWidths(
  options: TabOption[],
  tabWidth: number,
  minTabWidth: number,
  tabPadding: number,
): number[] {
  if (tabWidth > 0) {
    // Fixed width mode
    return options.map(() => tabWidth);
  }
  // Auto-width mode: content length + padding, with minimum
  return options.map((opt) => Math.max(minTabWidth, opt.name.length + tabPadding * 2));
}

export class TabSelect extends Box {
  private _tabOptions: TabOption[];
  private _selectedIndex: number;
  private _tabWidth: number;
  private _minTabWidth: number;
  private _tabPadding: number;
  private _tabGap: number;
  private _showDescription: boolean;
  private _showUnderline: boolean;
  private _showScrollArrows: boolean;
  private _scrollArrowLeft: string;
  private _scrollArrowRight: string;
  private _wrapSelection: boolean;
  private _textColor: RGBA;
  private _selectedTextColor: RGBA;
  private _selectedBgColor: RGBA | null = null;
  private _activeUnderlineColor: RGBA;
  private _inactiveUnderlineColor: RGBA;
  private _descriptionColor: RGBA;
  private _contentNodeId: number;
  private readonly _keyHandler: (key: KeyEvent) => void;

  constructor(renderer: CliRenderer, options: TabSelectOptions = {}) {
    _tabSelectCounter++;
    super(renderer, {
      ...options,
      id: options.id ?? `tabselect-${_tabSelectCounter}`,
      focusable: true,
    });

    this._tabOptions = options.options ?? [];
    this._selectedIndex = options.selectedIndex ?? 0;
    this._tabWidth = options.tabWidth ?? 0; // Default to auto-width (0 = auto)
    this._minTabWidth = options.minTabWidth ?? 8; // Minimum width for auto mode
    this._tabPadding = options.tabPadding ?? 2; // Padding on each side
    this._tabGap = options.tabGap ?? 1; // Gap between tabs
    this._showDescription = options.showDescription !== false;
    this._showUnderline = options.showUnderline !== false;
    this._showScrollArrows = options.showScrollArrows !== false;
    this._scrollArrowLeft = options.scrollArrowLeft ?? "◀";
    this._scrollArrowRight = options.scrollArrowRight ?? "▶";
    this._wrapSelection = options.wrapSelection ?? false;
    this._textColor = parseColor(options.textColor ?? "#888888");
    this._selectedTextColor = parseColor(options.selectedTextColor ?? "#ffffff");
    this._activeUnderlineColor = parseColor(options.activeUnderlineColor ?? "#0088ff");
    this._inactiveUnderlineColor = parseColor(options.inactiveUnderlineColor ?? "#333333");
    this._descriptionColor = parseColor(options.descriptionColor ?? "#666666");

    if (options.selectedBackgroundColor) {
      this._selectedBgColor = parseColor(options.selectedBackgroundColor);
    }

    this._contentNodeId = renderer.createNode("Text");
    renderer.appendChild(this._nodeId, this._contentNodeId);

    this._keyHandler = this._handleKey.bind(this);
    this._render();
  }

  get options(): TabOption[] {
    return this._tabOptions;
  }

  set options(opts: TabOption[]) {
    this._tabOptions = opts;
    this._selectedIndex = Math.min(this._selectedIndex, Math.max(0, opts.length - 1));
    this._render();
  }

  get selectedIndex(): number {
    return this._selectedIndex;
  }

  set selectedIndex(idx: number) {
    this._selectedIndex = Math.max(0, Math.min(this._tabOptions.length - 1, idx));
    this._render();
  }

  get showDescription(): boolean {
    return this._showDescription;
  }

  set showDescription(v: boolean) {
    this._showDescription = v;
    this._render();
  }

  get showUnderline(): boolean {
    return this._showUnderline;
  }

  set showUnderline(v: boolean) {
    this._showUnderline = v;
    this._render();
  }

  get showScrollArrows(): boolean {
    return this._showScrollArrows;
  }

  set showScrollArrows(v: boolean) {
    this._showScrollArrows = v;
    this._render();
  }

  get scrollArrowLeft(): string {
    return this._scrollArrowLeft;
  }

  set scrollArrowLeft(v: string) {
    this._scrollArrowLeft = v;
    this._render();
  }

  get scrollArrowRight(): string {
    return this._scrollArrowRight;
  }

  set scrollArrowRight(v: string) {
    this._scrollArrowRight = v;
    this._render();
  }

  get wrapSelection(): boolean {
    return this._wrapSelection;
  }

  set wrapSelection(v: boolean) {
    this._wrapSelection = v;
  }

  getSelectedOption(): TabOption | undefined {
    return this._tabOptions[this._selectedIndex];
  }

  getSelectedIndex(): number {
    return this._selectedIndex;
  }

  selectCurrent(): void {
    const opt = this.getSelectedOption();
    if (opt) {
      this.emit(TabSelectEvents.ITEM_SELECTED, this._selectedIndex, opt);
    }
  }

  moveLeft(steps = 1): void {
    const prev = this._selectedIndex;
    let next = this._selectedIndex - steps;
    if (this._wrapSelection) {
      next = ((next % this._tabOptions.length) + this._tabOptions.length) % this._tabOptions.length;
    } else {
      next = Math.max(0, next);
    }
    if (next !== prev) {
      this._selectedIndex = next;
      this._render();
      const opt = this.getSelectedOption();
      if (opt) this.emit(TabSelectEvents.SELECTION_CHANGED, next, opt);
    }
  }

  moveRight(steps = 1): void {
    const prev = this._selectedIndex;
    let next = this._selectedIndex + steps;
    if (this._wrapSelection) {
      next = next % this._tabOptions.length;
    } else {
      next = Math.min(this._tabOptions.length - 1, next);
    }
    if (next !== prev) {
      this._selectedIndex = next;
      this._render();
      const opt = this.getSelectedOption();
      if (opt) this.emit(TabSelectEvents.SELECTION_CHANGED, next, opt);
    }
  }

  override focus(): void {
    if (this._isDestroyed || this._focused) return;
    this._focused = true;
    this._render();
    this.emit(RenderableEvents.FOCUSED, this);
    this._renderer.keyHandler.offInternal("keypress", this._keyHandler);
    this._renderer.keyHandler.onInternal("keypress", this._keyHandler);
  }

  override blur(): void {
    if (this._isDestroyed) return;
    this._renderer.keyHandler.offInternal("keypress", this._keyHandler);
    if (!this._focused) return;
    this._focused = false;
    this._render();
    this.emit(RenderableEvents.BLURRED, this);
  }

  private _handleKey(key: KeyEvent): void {
    if (!this._focused || this._isDestroyed) return;

    if (key.name === "left" || (key.shift && key.name === "tab")) {
      this.moveLeft();
    } else if (key.name === "right" || key.name === "tab") {
      this.moveRight();
    } else if (key.name === "return" || key.name === "linefeed") {
      this.selectCurrent();
    }
  }

  private _render(): void {
    if (this._isDestroyed) return;

    const lines: string[] = [];
    const tabLine: string[] = [];

    // Calculate dynamic widths for each tab
    const tabWidths = calculateTabWidths(
      this._tabOptions,
      this._tabWidth,
      this._minTabWidth,
      this._tabPadding,
    );

    if (this._showScrollArrows) {
      tabLine.push(`\x1b[38;2;100;100;100m${this._scrollArrowLeft}\x1b[0m`);
    }

    for (let i = 0; i < this._tabOptions.length; i++) {
      const opt = this._tabOptions[i];
      if (!opt) continue;
      const isSelected = i === this._selectedIndex;
      const width = tabWidths[i] ?? this._minTabWidth;

      // Center the name within the tab width, padding on both sides
      const name = opt.name.padEnd(width).slice(0, width);
      const textColor = isSelected ? this._selectedTextColor : this._textColor;
      const tc = `${textColor.r};${textColor.g};${textColor.b}`;

      if (isSelected && this._selectedBgColor) {
        const bc = `${this._selectedBgColor.r};${this._selectedBgColor.g};${this._selectedBgColor.b}`;
        tabLine.push(`\x1b[48;2;${bc}m\x1b[38;2;${tc}m${name}\x1b[0m`);
      } else {
        tabLine.push(`\x1b[38;2;${tc}m${name}\x1b[0m`);
      }

      // Add gap between tabs (but not after the last one)
      if (i < this._tabOptions.length - 1 && this._tabGap > 0) {
        tabLine.push(" ".repeat(this._tabGap));
      }
    }

    if (this._showScrollArrows) {
      tabLine.push(`\x1b[38;2;100;100;100m${this._scrollArrowRight}\x1b[0m`);
    }

    lines.push(tabLine.join(""));

    if (this._showUnderline) {
      const underline: string[] = [];
      for (let i = 0; i < this._tabOptions.length; i++) {
        const isSelected = i === this._selectedIndex;
        const color = isSelected ? this._activeUnderlineColor : this._inactiveUnderlineColor;
        const cc = `${color.r};${color.g};${color.b}`;
        const width = tabWidths[i] ?? this._minTabWidth;
        underline.push(`\x1b[38;2;${cc}m${"─".repeat(width)}\x1b[0m`);
        // Add matching gap between underlines
        if (i < this._tabOptions.length - 1 && this._tabGap > 0) {
          underline.push(" ".repeat(this._tabGap));
        }
      }
      lines.push(underline.join(""));
    }

    if (this._showDescription) {
      const opt = this.getSelectedOption();
      if (opt?.description) {
        const dc = `${this._descriptionColor.r};${this._descriptionColor.g};${this._descriptionColor.b}`;
        lines.push(`\x1b[38;2;${dc}m${opt.description}\x1b[0m`);
      } else {
        lines.push("");
      }
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

export { TabSelectEvents };
