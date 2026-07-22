import type { KeyEvent, TreeNode, TreeOptions } from "@bettertui/shared";
import type { Command } from "../command/types";
import { Renderable } from "../renderable";

// Re-export types consumers need when using this widget
export type { TreeOptions };

export class Tree extends Renderable<TreeOptions> {
  private _nodes: TreeNode[];
  private _flatItems: FlatItem[] = [];
  private _selectedId: string | null;
  private _scrollOffset = 0;

  constructor(options: TreeOptions = {}) {
    super(options);
    this._nodes = options.nodes ?? [];
    this._selectedId = options.selectedId ?? null;
    this._rebuildFlat();
  }

  override update(options: Partial<TreeOptions>): void {
    if (options.nodes !== undefined) {
      this._nodes = options.nodes;
      this._rebuildFlat();
    }
    if (options.selectedId !== undefined) {
      this._selectedId = options.selectedId ?? null;
    }
    super.update(options);
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "Box" }];
    cmds.push({ type: "SetFlexDirection", id, direction: "column" as never });

    const viewHeight = 20; // Default view, no height option in TreeOptions
    const visibleStart = this._scrollOffset;
    const visibleEnd = Math.min(this._flatItems.length, visibleStart + viewHeight);

    for (let i = visibleStart; i < visibleEnd; i++) {
      const item = this._flatItems[i];
      if (!item) continue;

      const rowId = `${id}-row-${i}`;
      const indent = (this.opts.indentSize ?? 2) * item.depth;
      const prefix = " ".repeat(indent);
      const toggle = item.hasChildren ? (item.node.expanded ? "▼ " : "▶ ") : "  ";
      const isSelected = item.node.id === this._selectedId;

      cmds.push({ type: "CreateNode", id: rowId, kind: "Text" });
      cmds.push({ type: "SetText", id: rowId, text: `${prefix}${toggle}${item.node.label}` });

      if (isSelected) {
        cmds.push({ type: "SetInverse", id: rowId, value: true });
      }

      cmds.push({ type: "AppendChild", parent: id, child: rowId });
    }

    return cmds;
  }

  override handleKey(key: KeyEvent): boolean {
    if (this._flatItems.length === 0) return false;

    const currentIdx = this._flatItems.findIndex((fi) => fi.node.id === this._selectedId);

    if (key.key === "up") {
      const newIdx = Math.max(0, currentIdx - 1);
      const item = this._flatItems[newIdx];
      if (item) {
        this._selectedId = item.node.id;
        this.opts.onSelect?.(item.node);
        this._ensureVisible(newIdx);
      }
      return true;
    }

    if (key.key === "down") {
      const newIdx = Math.min(this._flatItems.length - 1, currentIdx + 1);
      const item = this._flatItems[newIdx];
      if (item) {
        this._selectedId = item.node.id;
        this.opts.onSelect?.(item.node);
        this._ensureVisible(newIdx);
      }
      return true;
    }

    if (key.key === "right" || key.key === "return") {
      const item = this._flatItems[currentIdx];
      if (item?.hasChildren) {
        item.node.expanded = true;
        this.opts.onToggle?.(item.node, true);
        this._rebuildFlat();
      } else if (item) {
        this.opts.onSelect?.(item.node);
      }
      return true;
    }

    if (key.key === "left") {
      const item = this._flatItems[currentIdx];
      if (item?.hasChildren && item.node.expanded) {
        item.node.expanded = false;
        this.opts.onToggle?.(item.node, false);
        this._rebuildFlat();
      } else if (item && item.depth > 0) {
        // Move to parent
        for (let i = currentIdx - 1; i >= 0; i--) {
          const candidate = this._flatItems[i];
          if (candidate && candidate.depth < item.depth) {
            this._selectedId = candidate.node.id;
            this.opts.onSelect?.(candidate.node);
            this._ensureVisible(i);
            break;
          }
        }
      }
      return true;
    }

    if (key.key === "home") {
      const first = this._flatItems[0];
      if (first) {
        this._selectedId = first.node.id;
        this._scrollOffset = 0;
        this.opts.onSelect?.(first.node);
      }
      return true;
    }

    if (key.key === "end") {
      const last = this._flatItems[this._flatItems.length - 1];
      if (last) {
        this._selectedId = last.node.id;
        this._ensureVisible(this._flatItems.length - 1);
        this.opts.onSelect?.(last.node);
      }
      return true;
    }

    return false;
  }

  private _rebuildFlat(): void {
    this._flatItems = [];
    for (const node of this._nodes) {
      this._flatten(node, 0);
    }
  }

  private _flatten(node: TreeNode, depth: number): void {
    const hasChildren = (node.children?.length ?? 0) > 0;
    this._flatItems.push({ node, depth, hasChildren });
    if (hasChildren && node.expanded) {
      for (const child of node.children ?? []) {
        this._flatten(child, depth + 1);
      }
    }
  }

  private _ensureVisible(idx: number): void {
    const viewHeight = 20;
    if (idx < this._scrollOffset) {
      this._scrollOffset = idx;
    } else if (idx >= this._scrollOffset + viewHeight) {
      this._scrollOffset = idx - viewHeight + 1;
    }
  }
}

interface FlatItem {
  node: TreeNode;
  depth: number;
  hasChildren: boolean;
}
