import type { DevToolsNode } from "../../devtools.types";
import { truncate } from "../ansiUtils";
import { DebugPanel, type Panel, type PanelContext } from "../panel.types";

/** Flatten a tree to indented display lines (depth-first). */
function walk(
  node: DevToolsNode,
  depth: number,
  out: string[],
  maxRows: number,
  highlight: string | null,
): void {
  if (out.length >= maxRows) return;
  const indent = "  ".repeat(depth);
  const marker = node.id === highlight ? "▸ " : "";
  const dims = node.layout ? ` ${node.layout.width}×${node.layout.height}` : "";
  out.push(`${indent}${marker}${node.type}#${node.id}${dims}`);
  for (const child of node.children) {
    walk(child, depth + 1, out, maxRows, highlight);
  }
}

/**
 * Panel 2 — Node Tree Viewer (display-only in the all-TS pass).
 *
 * Renders the recorded render tree with id/type/dims. Click-to-inspect
 * hit-routing is deferred to a later Rust phase (see the task plan); this pass
 * shows the tree and marks the highlighted node.
 */
export const treePanel: Panel = {
  id: DebugPanel.Tree,
  title: "Tree",

  render(ctx: PanelContext): string[] {
    const { devtools } = ctx;
    const w = ctx.maxWidth;
    const rows = Math.max(1, ctx.maxHeight);
    const root = devtools.tree.getRoot();

    if (!root) {
      return [`(no tree recorded — ${devtools.tree.countNodes()} nodes)`];
    }

    const lines: string[] = [];
    walk(root, 0, lines, rows, devtools.highlightedNodeId);
    return lines.map((line) => truncate(line, w));
  },
};
