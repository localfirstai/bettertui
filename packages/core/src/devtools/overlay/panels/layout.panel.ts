import { displayWidth } from "../ansi.utils";
import { DebugPanel, type Panel, type PanelContext } from "../panel.types";

function labelled(label: string, value: string, width: number): string {
  const gap = Math.max(1, width - displayWidth(label) - displayWidth(value));
  return label + " ".repeat(gap) + value;
}

/**
 * Panel 3 — Layout Inspector.
 *
 * Shows the box model (position + computed dims) for the highlighted node, or
 * a summary of the recorded tree when nothing is highlighted. Data comes from
 * the TreeInspector's recorded layout JSON (`getNode().layout`).
 */
export const layoutPanel: Panel = {
  id: DebugPanel.Layout,
  title: "Layout",

  render(ctx: PanelContext): string[] {
    const { devtools } = ctx;
    const w = ctx.maxWidth;
    const target = devtools.highlightedNodeId;

    if (!target) {
      return [
        "No node highlighted.",
        "",
        `Nodes: ${devtools.tree.countNodes()}`,
        "Use highlight(id) to inspect.",
      ];
    }

    const node = devtools.inspect(target);
    if (!node) {
      return [`Node ${target} not found.`];
    }

    const lines: string[] = [];
    lines.push(labelled("id", node.id, w));
    lines.push(labelled("type", node.type, w));

    const layout = node.layout;
    if (layout) {
      lines.push("─".repeat(w));
      lines.push(labelled("x, y", `${layout.x}, ${layout.y}`, w));
      lines.push(labelled("size", `${layout.width}×${layout.height}`, w));
    } else {
      lines.push("(no layout recorded)");
    }

    const style = node.style;
    if (style && Object.keys(style).length > 0) {
      lines.push("─".repeat(w));
      for (const [key, value] of Object.entries(style)) {
        lines.push(labelled(key, formatValue(value), w));
      }
    }

    return lines;
  },
};

function formatValue(value: unknown): string {
  if (value === null || value === undefined) return "—";
  if (typeof value === "object") return JSON.stringify(value);
  return String(value);
}
