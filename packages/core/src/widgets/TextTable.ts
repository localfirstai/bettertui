import type { TextTableOptions } from "./widget.types";
import type { Command } from "../command/command.types";
import { Renderable } from "../renderable";

export type { TextTableOptions };

export class TextTable extends Renderable<TextTableOptions> {
  private _rows: string[][] = [];

  constructor(options: TextTableOptions = {}) {
    super(options);
    this._rows = options.rows ?? [];
  }

  override update(options: Partial<TextTableOptions>): void {
    if (options.rows !== undefined) this._rows = options.rows;
    super.update(options);
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "Box" }];
    cmds.push({ type: "SetFlexDirection", id, direction: "column" as never });
    if (this.opts.headers && this.opts.showHeader !== false) {
      const headerId = `${id}-header`;
      cmds.push({ type: "CreateNode", id: headerId, kind: "Box" });
      cmds.push({ type: "SetFlexDirection", id: headerId, direction: "row" as never });
      cmds.push({ type: "AppendChild", parent: id, child: headerId });
      for (let h = 0; h < this.opts.headers.length; h++) {
        const header = this.opts.headers[h];
        if (header === undefined) continue;
        const cellId = `${headerId}-h-${h}`;
        cmds.push({ type: "CreateNode", id: cellId, kind: "Text" });
        cmds.push({ type: "SetText", id: cellId, text: header });
        cmds.push({ type: "SetBold", id: cellId, value: true });
        cmds.push({ type: "AppendChild", parent: headerId, child: cellId });
      }
    }
    for (let r = 0; r < this._rows.length; r++) {
      const row = this._rows[r];
      if (!row) continue;
      const rowId = `${id}-row-${r}`;
      cmds.push({ type: "CreateNode", id: rowId, kind: "Box" });
      cmds.push({ type: "SetFlexDirection", id: rowId, direction: "row" as never });
      cmds.push({ type: "AppendChild", parent: id, child: rowId });
      for (let c = 0; c < row.length; c++) {
        const cell = row[c];
        if (cell === undefined) continue;
        const cellId = `${rowId}-c-${c}`;
        cmds.push({ type: "CreateNode", id: cellId, kind: "Text" });
        cmds.push({ type: "SetText", id: cellId, text: cell });
        cmds.push({ type: "AppendChild", parent: rowId, child: cellId });
      }
    }
    return cmds;
  }
}
