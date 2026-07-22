import type { KeyEvent } from "@bettertui/shared";
import type { TableBorderStyle, TableColumn, TableColumnAlign, TableOptions } from "./widget.types";
import type { Command } from "../command/command.types";
import { Renderable } from "../renderable";

export type { TableBorderStyle, TableColumn, TableColumnAlign, TableOptions };

// ─── Border character sets ────────────────────────────────────────────────────

interface BorderChars {
  topLeft: string;
  topRight: string;
  bottomLeft: string;
  bottomRight: string;
  horizontal: string;
  vertical: string;
  topT: string;
  bottomT: string;
  leftT: string;
  rightT: string;
  cross: string;
  headerLeft: string;
  headerRight: string;
  headerT: string;
  headerCross: string;
  headerH: string;
}

const BORDER_SETS: Record<TableBorderStyle, BorderChars> = {
  none: {
    topLeft: "",
    topRight: "",
    bottomLeft: "",
    bottomRight: "",
    horizontal: " ",
    vertical: " ",
    topT: "",
    bottomT: "",
    leftT: "",
    rightT: "",
    cross: " ",
    headerLeft: "",
    headerRight: "",
    headerT: "",
    headerCross: "",
    headerH: " ",
  },
  single: {
    topLeft: "┌",
    topRight: "┐",
    bottomLeft: "└",
    bottomRight: "┘",
    horizontal: "─",
    vertical: "│",
    topT: "┬",
    bottomT: "┴",
    leftT: "├",
    rightT: "┤",
    cross: "┼",
    headerLeft: "├",
    headerRight: "┤",
    headerT: "┼",
    headerCross: "┼",
    headerH: "─",
  },
  double: {
    topLeft: "╔",
    topRight: "╗",
    bottomLeft: "╚",
    bottomRight: "╝",
    horizontal: "═",
    vertical: "║",
    topT: "╦",
    bottomT: "╩",
    leftT: "╠",
    rightT: "╣",
    cross: "╬",
    headerLeft: "╠",
    headerRight: "╣",
    headerT: "╬",
    headerCross: "╬",
    headerH: "═",
  },
  rounded: {
    topLeft: "╭",
    topRight: "╮",
    bottomLeft: "╰",
    bottomRight: "╯",
    horizontal: "─",
    vertical: "│",
    topT: "┬",
    bottomT: "┴",
    leftT: "├",
    rightT: "┤",
    cross: "┼",
    headerLeft: "├",
    headerRight: "┤",
    headerT: "┼",
    headerCross: "┼",
    headerH: "─",
  },
  bold: {
    topLeft: "┏",
    topRight: "┓",
    bottomLeft: "┗",
    bottomRight: "┛",
    horizontal: "━",
    vertical: "┃",
    topT: "┳",
    bottomT: "┻",
    leftT: "┣",
    rightT: "┫",
    cross: "╋",
    headerLeft: "┣",
    headerRight: "┫",
    headerT: "╋",
    headerCross: "╋",
    headerH: "━",
  },
};

// ─── Alignment helper ─────────────────────────────────────────────────────────

function pad(text: string, width: number, align: TableColumnAlign = "left"): string {
  const len = text.length;
  if (len >= width) return text.slice(0, width);
  const spaces = width - len;
  switch (align) {
    case "right":
      return " ".repeat(spaces) + text;
    case "center": {
      const leftPad = Math.floor(spaces / 2);
      const rightPad = spaces - leftPad;
      return " ".repeat(leftPad) + text + " ".repeat(rightPad);
    }
    default:
      return text + " ".repeat(spaces);
  }
}

// ─── Column width computation ─────────────────────────────────────────────────

function computeColWidths(columns: TableColumn[], rows: string[][]): number[] {
  return columns.map((col, ci) => {
    let w = col.width ?? col.header.length;
    if (col.minWidth !== undefined) w = Math.max(w, col.minWidth);
    for (const row of rows) {
      const cell = row[ci] ?? "";
      w = Math.max(w, cell.length);
    }
    if (col.maxWidth !== undefined) w = Math.min(w, col.maxWidth);
    return w;
  });
}

// ─── Widget ───────────────────────────────────────────────────────────────────

export class Table extends Renderable<TableOptions> {
  private _columns: TableColumn[];
  private _rows: string[][];
  private _selectedRow: number;

  constructor(options: TableOptions = {}) {
    super(options);
    this._columns = options.columns ?? [];
    this._rows = options.rows ?? [];
    this._selectedRow = options.selectedRow ?? -1;
  }

  get selectedRow(): number {
    return this._selectedRow;
  }

  get rowCount(): number {
    return this._rows.length;
  }

  override update(options: Partial<TableOptions>): void {
    if (options.columns !== undefined) this._columns = options.columns;
    if (options.rows !== undefined) this._rows = options.rows;
    if (options.selectedRow !== undefined) this._selectedRow = options.selectedRow;
    super.update(options);
  }

  override handleKey(key: KeyEvent): boolean {
    if (this._rows.length === 0) return false;

    if (key.key === "up") {
      const next = Math.max(0, this._selectedRow - 1);
      if (next !== this._selectedRow) {
        this._selectedRow = next;
        const row = this._rows[next];
        if (row) this.opts.onSelect?.(row, next);
        return true;
      }
    }

    if (key.key === "down") {
      const next = Math.min(this._rows.length - 1, this._selectedRow + 1);
      if (next !== this._selectedRow) {
        this._selectedRow = next;
        const row = this._rows[next];
        if (row) this.opts.onSelect?.(row, next);
        return true;
      }
    }

    if (key.key === "home") {
      this._selectedRow = 0;
      const row = this._rows[0];
      if (row) this.opts.onSelect?.(row, 0);
      return true;
    }

    if (key.key === "end") {
      const last = Math.max(0, this._rows.length - 1);
      this._selectedRow = last;
      const row = this._rows[last];
      if (row) this.opts.onSelect?.(row, last);
      return true;
    }

    if (key.key === "return" && this._selectedRow >= 0) {
      const row = this._rows[this._selectedRow];
      if (row) this.opts.onSelect?.(row, this._selectedRow);
      return true;
    }

    return false;
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "Box" }];
    cmds.push({ type: "SetFlexDirection", id, direction: "column" as never });

    // If no columns defined, render cells individually in rows
    if (this._columns.length === 0) {
      for (let ri = 0; ri < this._rows.length; ri++) {
        const row = this._rows[ri];
        if (!row) continue;
        const rowId = `${id}-r${ri}`;
        cmds.push({ type: "CreateNode", id: rowId, kind: "Box" });
        cmds.push({ type: "SetFlexDirection", id: rowId, direction: "row" as never });
        for (let ci = 0; ci < row.length; ci++) {
          const cellId = `${rowId}-c${ci}`;
          cmds.push({ type: "CreateNode", id: cellId, kind: "Text" });
          cmds.push({ type: "SetText", id: cellId, text: row[ci] ?? "" });
          cmds.push({ type: "AppendChild", parent: rowId, child: cellId });
        }
        cmds.push({ type: "AppendChild", parent: id, child: rowId });
      }
      return cmds;
    }

    const showBorder = this.opts.showBorder !== false;
    const borderStyle: TableBorderStyle = this.opts.borderStyle ?? "single";
    const b = BORDER_SETS[borderStyle];
    const showHeader = this.opts.showHeader !== false;
    const compact = this.opts.compact === true;
    const striped = this.opts.striped === true;
    const pad_ = compact ? 0 : 1;

    const colWidths = computeColWidths(this._columns, this._rows);

    // Build horizontal separator strings
    const buildHLine = (left: string, mid: string, right: string, fill: string): string => {
      if (!showBorder) return "";
      let line = left;
      for (let ci = 0; ci < colWidths.length; ci++) {
        const w = (colWidths[ci] ?? 0) + pad_ * 2;
        line += fill.repeat(w);
        if (ci < colWidths.length - 1) line += mid;
      }
      line += right;
      return line;
    };

    let _lineIdx = 0;

    // Top border
    if (showBorder) {
      const topLine = buildHLine(b.topLeft, b.topT, b.topRight, b.horizontal);
      const topId = `${id}-tl`;
      cmds.push({ type: "CreateNode", id: topId, kind: "Text" });
      cmds.push({ type: "SetText", id: topId, text: topLine });
      cmds.push({ type: "SetDim", id: topId, value: true });
      cmds.push({ type: "AppendChild", parent: id, child: topId });
      _lineIdx++;
    }

    // Header row
    if (showHeader && this._columns.length > 0) {
      const hRowId = `${id}-hr`;
      cmds.push({ type: "CreateNode", id: hRowId, kind: "Box" });
      cmds.push({ type: "SetFlexDirection", id: hRowId, direction: "row" as never });

      for (let ci = 0; ci < this._columns.length; ci++) {
        const col = this._columns[ci];
        if (!col) continue;
        const w = colWidths[ci] ?? col.header.length;
        const cellText = pad(col.header, w, col.align ?? "left");
        const cellContent = compact ? cellText : ` ${cellText} `;

        if (showBorder && ci === 0) {
          const sepId = `${hRowId}-ls`;
          cmds.push({ type: "CreateNode", id: sepId, kind: "Text" });
          cmds.push({ type: "SetText", id: sepId, text: b.vertical });
          cmds.push({ type: "SetDim", id: sepId, value: true });
          cmds.push({ type: "AppendChild", parent: hRowId, child: sepId });
        }

        const cellId = `${hRowId}-c${ci}`;
        cmds.push({ type: "CreateNode", id: cellId, kind: "Text" });
        cmds.push({ type: "SetText", id: cellId, text: cellContent });
        cmds.push({ type: "SetBold", id: cellId, value: true });
        cmds.push({ type: "AppendChild", parent: hRowId, child: cellId });

        if (showBorder) {
          const sepId = `${hRowId}-rs${ci}`;
          cmds.push({ type: "CreateNode", id: sepId, kind: "Text" });
          cmds.push({ type: "SetText", id: sepId, text: b.vertical });
          cmds.push({ type: "SetDim", id: sepId, value: true });
          cmds.push({ type: "AppendChild", parent: hRowId, child: sepId });
        }
      }

      cmds.push({ type: "AppendChild", parent: id, child: hRowId });
      _lineIdx++;

      // Header separator
      if (showBorder) {
        const sepLine = buildHLine(b.headerLeft, b.headerT, b.headerRight, b.headerH);
        const sepId = `${id}-hs`;
        cmds.push({ type: "CreateNode", id: sepId, kind: "Text" });
        cmds.push({ type: "SetText", id: sepId, text: sepLine });
        cmds.push({ type: "SetDim", id: sepId, value: true });
        cmds.push({ type: "AppendChild", parent: id, child: sepId });
        _lineIdx++;
      }
    }

    // Data rows
    for (let ri = 0; ri < this._rows.length; ri++) {
      const row = this._rows[ri];
      if (!row) continue;
      const isSelected = ri === this._selectedRow;
      const isEven = ri % 2 === 0;

      const dataRowId = `${id}-dr${ri}`;
      cmds.push({ type: "CreateNode", id: dataRowId, kind: "Box" });
      cmds.push({ type: "SetFlexDirection", id: dataRowId, direction: "row" as never });

      if (isSelected) {
        cmds.push({ type: "SetInverse", id: dataRowId, value: true });
      } else if (striped && isEven) {
        cmds.push({ type: "SetDim", id: dataRowId, value: true });
      }

      for (let ci = 0; ci < this._columns.length; ci++) {
        const col = this._columns[ci];
        if (!col) continue;
        const w = colWidths[ci] ?? 0;
        const cellText = pad(row[ci] ?? "", w, col.align ?? "left");
        const cellContent = compact ? cellText : ` ${cellText} `;

        if (showBorder && ci === 0) {
          const sepId = `${dataRowId}-ls`;
          cmds.push({ type: "CreateNode", id: sepId, kind: "Text" });
          cmds.push({ type: "SetText", id: sepId, text: b.vertical });
          cmds.push({ type: "SetDim", id: sepId, value: !isSelected });
          cmds.push({ type: "AppendChild", parent: dataRowId, child: sepId });
        }

        const cellId = `${dataRowId}-c${ci}`;
        cmds.push({ type: "CreateNode", id: cellId, kind: "Text" });
        cmds.push({ type: "SetText", id: cellId, text: cellContent });
        cmds.push({ type: "AppendChild", parent: dataRowId, child: cellId });

        if (showBorder) {
          const sepId = `${dataRowId}-rs${ci}`;
          cmds.push({ type: "CreateNode", id: sepId, kind: "Text" });
          cmds.push({ type: "SetText", id: sepId, text: b.vertical });
          cmds.push({ type: "SetDim", id: sepId, value: !isSelected });
          cmds.push({ type: "AppendChild", parent: dataRowId, child: sepId });
        }
      }

      cmds.push({ type: "AppendChild", parent: id, child: dataRowId });
      _lineIdx++;
    }

    // Bottom border
    if (showBorder) {
      const bottomLine = buildHLine(b.bottomLeft, b.bottomT, b.bottomRight, b.horizontal);
      const bottomId = `${id}-bl`;
      cmds.push({ type: "CreateNode", id: bottomId, kind: "Text" });
      cmds.push({ type: "SetText", id: bottomId, text: bottomLine });
      cmds.push({ type: "SetDim", id: bottomId, value: true });
      cmds.push({ type: "AppendChild", parent: id, child: bottomId });
    }

    return cmds;
  }
}
