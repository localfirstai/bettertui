import type { KeyEvent } from "@bettertui/shared";
import type { ListItem, ListOptions } from "./widget.types";
import type { Command } from "../command/command.types";
import { Renderable } from "../renderable";

export type { ListItem, ListOptions };

export class List extends Renderable<ListOptions> {
  private _items: ListItem[];
  private _selectedIndex: number;
  private _selectedIds: Set<string>;
  private _scrollOffset = 0;
  private _query = "";

  constructor(options: ListOptions = {}) {
    super(options);
    this._items = options.items ?? [];
    this._selectedIndex = this._findIndexById(options.selectedId) ?? 0;
    this._selectedIds = options.selectedId ? new Set([options.selectedId]) : new Set();
  }

  get selectedItem(): ListItem | undefined {
    const filtered = this._filteredItems();
    return filtered[this._selectedIndex];
  }

  get selectedIndex(): number {
    return this._selectedIndex;
  }

  get selectedItems(): ListItem[] {
    if (!this.opts.multiSelect) {
      const item = this.selectedItem;
      return item ? [item] : [];
    }
    return this._filteredItems().filter((item) => this._selectedIds.has(item.id));
  }

  override update(options: Partial<ListOptions>): void {
    if (options.items !== undefined) {
      this._items = options.items;
      this._selectedIndex = 0;
      this._scrollOffset = 0;
      this._selectedIds.clear();
    }
    if (options.selectedId !== undefined) {
      const idx = this._findIndexById(options.selectedId);
      if (idx !== null) this._selectedIndex = idx;
    }
    super.update(options);
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "List" }];

    const viewHeight = this.opts.height ?? 10;
    cmds.push({ type: "SetHeight", id, value: viewHeight });
    cmds.push({ type: "SetFlexDirection", id, direction: "column" as never });

    // Search bar
    if (this.opts.searchable) {
      const searchId = `${id}-search`;
      cmds.push({ type: "CreateNode", id: searchId, kind: "Text" });
      cmds.push({
        type: "SetText",
        id: searchId,
        text: this._query ? `/ ${this._query}` : `/ ${this.opts.placeholder ?? "Search..."}`,
      });
      cmds.push({ type: "AppendChild", parent: id, child: searchId });
    }

    const filtered = this._filteredItems();
    const visibleStart = this._scrollOffset;
    const visibleEnd = Math.min(filtered.length, visibleStart + viewHeight);

    for (let i = visibleStart; i < visibleEnd; i++) {
      const item = filtered[i];
      if (!item) continue;
      const itemId = `${id}-item-${i}`;
      cmds.push({ type: "CreateNode", id: itemId, kind: "Text" });

      const prefix = i === this._selectedIndex ? "▶ " : "  ";
      const multiMark = this.opts.multiSelect && this._selectedIds.has(item.id) ? "✓ " : "";
      const suffix = item.description ? `  ${item.description}` : "";
      cmds.push({
        type: "SetText",
        id: itemId,
        text: `${prefix}${multiMark}${item.label}${suffix}`,
      });

      if (i === this._selectedIndex) {
        cmds.push({ type: "SetInverse", id: itemId, value: true });
      }
      if (this.opts.multiSelect && this._selectedIds.has(item.id) && i !== this._selectedIndex) {
        cmds.push({ type: "SetBold", id: itemId, value: true });
      }
      if (item.disabled) {
        cmds.push({ type: "SetDim", id: itemId, value: true });
      }

      cmds.push({ type: "AppendChild", parent: id, child: itemId });
    }

    return cmds;
  }

  override handleKey(key: KeyEvent): boolean {
    const filtered = this._filteredItems();

    if (this.opts.searchable && !key.ctrl && !key.meta && !key.alt) {
      if (key.key === "backspace") {
        if (this._query.length > 0) {
          this._query = this._query.slice(0, -1);
          this._selectedIndex = 0;
          this._scrollOffset = 0;
          return true;
        }
        return false;
      }
      if (key.key && key.key.length === 1) {
        this._query += key.key;
        this._selectedIndex = 0;
        this._scrollOffset = 0;
        return true;
      }
    }

    if (filtered.length === 0) return false;

    if (key.key === "up") {
      this._selectedIndex = Math.max(0, this._selectedIndex - 1);
      this._ensureVisible();
      const item = filtered[this._selectedIndex];
      if (item) this.opts.onChange?.(item);
      return true;
    }

    if (key.key === "down") {
      this._selectedIndex = Math.min(filtered.length - 1, this._selectedIndex + 1);
      this._ensureVisible();
      const item = filtered[this._selectedIndex];
      if (item) this.opts.onChange?.(item);
      return true;
    }

    if (key.key === "return") {
      const item = filtered[this._selectedIndex];
      if (item && !item.disabled) {
        if (this.opts.multiSelect) {
          if (this._selectedIds.has(item.id)) {
            this._selectedIds.delete(item.id);
          } else {
            this._selectedIds.add(item.id);
          }
          this.opts.onSelectMulti?.(this.selectedItems);
        } else {
          this.opts.onSelect?.(item);
        }
      }
      return true;
    }

    if (key.key === "home") {
      this._selectedIndex = 0;
      this._scrollOffset = 0;
      return true;
    }

    if (key.key === "end") {
      this._selectedIndex = Math.max(0, filtered.length - 1);
      this._ensureVisible();
      return true;
    }

    if (key.key === "pageup") {
      const viewHeight = this.opts.height ?? 10;
      this._selectedIndex = Math.max(0, this._selectedIndex - viewHeight);
      this._ensureVisible();
      return true;
    }

    if (key.key === "pagedown") {
      const viewHeight = this.opts.height ?? 10;
      this._selectedIndex = Math.min(filtered.length - 1, this._selectedIndex + viewHeight);
      this._ensureVisible();
      return true;
    }

    return false;
  }

  private _filteredItems(): ListItem[] {
    if (!this._query) return this._items;
    const q = this._query.toLowerCase();
    return this._items.filter(
      (item) =>
        item.label.toLowerCase().includes(q) ||
        (item.description?.toLowerCase().includes(q) ?? false),
    );
  }

  private _findIndexById(id?: string): number | null {
    if (!id) return null;
    const idx = this._items.findIndex((item) => item.id === id);
    return idx >= 0 ? idx : null;
  }

  private _ensureVisible(): void {
    const viewHeight = this.opts.height ?? 10;
    if (this._selectedIndex < this._scrollOffset) {
      this._scrollOffset = this._selectedIndex;
    } else if (this._selectedIndex >= this._scrollOffset + viewHeight) {
      this._scrollOffset = this._selectedIndex - viewHeight + 1;
    }
  }
}
