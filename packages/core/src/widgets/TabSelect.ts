import type { KeyEvent } from "@bettertui/shared";
import type { TabSelectOptions } from "./widget.types";
import type { Command } from "../command/command.types";
import { Renderable } from "../renderable";

export type { TabSelectOptions };

export class TabSelect extends Renderable<TabSelectOptions> {
  private _items: Array<{ label: string; value: string }> = [];
  private _activeIndex = 0;

  constructor(options: TabSelectOptions = {}) {
    super(options);
    this._items = options.tabs ?? [];
    this._activeIndex = options.activeIndex ?? 0;
  }

  override update(options: Partial<TabSelectOptions>): void {
    if (options.tabs !== undefined) this._items = options.tabs;
    if (options.activeIndex !== undefined) this._activeIndex = options.activeIndex;
    super.update(options);
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "Box" }];
    cmds.push({ type: "SetFlexDirection", id, direction: "row" as never });
    for (let i = 0; i < this._items.length; i++) {
      const tab = this._items[i];
      if (!tab) continue;
      const tabId = `${id}-tab-${i}`;
      cmds.push({ type: "CreateNode", id: tabId, kind: "Text" });
      cmds.push({ type: "SetText", id: tabId, text: tab.label });
      cmds.push({ type: "AppendChild", parent: id, child: tabId });
      if (i === this._activeIndex) cmds.push({ type: "SetUnderline", id: tabId, value: true });
    }
    return cmds;
  }

  override handleKey(key: KeyEvent): boolean {
    if (this._items.length === 0) return false;
    if (key.key === "left") {
      this._activeIndex = Math.max(0, this._activeIndex - 1);
      const tab = this._items[this._activeIndex];
      if (tab) this.opts.onChange?.(tab.value);
      return true;
    }
    if (key.key === "right") {
      this._activeIndex = Math.min(this._items.length - 1, this._activeIndex + 1);
      const tab = this._items[this._activeIndex];
      if (tab) this.opts.onChange?.(tab.value);
      return true;
    }
    return false;
  }
}
