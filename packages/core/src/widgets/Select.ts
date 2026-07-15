import type { KeyEvent, SelectOptions } from "@bettertui/shared";
import type { Command } from "../command/types";
import { Renderable } from "../renderable";

export type { SelectOptions };

export class Select extends Renderable<SelectOptions> {
  private _items: Array<{ label: string; value: string }> = [];
  private _selectedIndex = -1;

  constructor(options: SelectOptions = {}) {
    super(options);
    this._items = options.options ?? [];
    if (options.value) {
      this._selectedIndex = this._items.findIndex((o) => o.value === options.value);
    }
  }

  override update(options: Partial<SelectOptions>): void {
    if (options.options !== undefined) this._items = options.options;
    if (options.value !== undefined)
      this._selectedIndex = this._items.findIndex((o) => o.value === options.value);
    super.update(options);
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "List" }];
    for (let i = 0; i < this._items.length; i++) {
      const item = this._items[i];
      if (!item) continue;
      const itemId = `${id}-${i}`;
      cmds.push({ type: "CreateNode", id: itemId, kind: "Text" });
      cmds.push({ type: "SetText", id: itemId, text: item.label });
      cmds.push({ type: "AppendChild", parent: id, child: itemId });
      if (i === this._selectedIndex) cmds.push({ type: "SetInverse", id: itemId, value: true });
    }
    return cmds;
  }

  override handleKey(key: KeyEvent): boolean {
    if (this.opts.disabled || this._items.length === 0) return false;
    if (key.key === "up") {
      this._selectedIndex = Math.max(0, this._selectedIndex - 1);
      const item = this._items[this._selectedIndex];
      if (item) this.opts.onChange?.(item.value);
      return true;
    }
    if (key.key === "down") {
      this._selectedIndex = Math.min(this._items.length - 1, this._selectedIndex + 1);
      const item = this._items[this._selectedIndex];
      if (item) this.opts.onChange?.(item.value);
      return true;
    }
    return false;
  }
}
