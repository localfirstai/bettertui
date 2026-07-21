import type { RecordedEvent } from "../../devtools.types";
import { truncate } from "../ansi.utils";
import { DebugPanel, type Panel, type PanelContext } from "../panel.types";

const CATEGORY_GLYPH: Record<string, string> = {
  keyboard: "⌨",
  mouse: "🖱",
  focus: "◎",
  resize: "⤢",
  lifecycle: "◆",
  clipboard: "📋",
  animation: "✦",
  scheduler: "⏱",
};

function summarize(event: RecordedEvent): string {
  const data = event.data as Record<string, unknown> | undefined;
  switch (event.category) {
    case "keyboard": {
      const key = data && typeof data.key === "string" ? data.key : "?";
      return `key ${key}`;
    }
    case "mouse": {
      const x = data?.x ?? "?";
      const y = data?.y ?? "?";
      return `${event.type} @${x},${y}`;
    }
    case "focus":
      return `${event.type} ${event.target ?? ""}`.trim();
    case "resize": {
      const width = data?.width ?? "?";
      const height = data?.height ?? "?";
      return `${width}×${height}`;
    }
    default:
      return event.type;
  }
}

/**
 * Panel 4 — Event Tracer.
 *
 * Renders the most recent key/mouse/focus/resize events from the
 * EventInspector, newest last, one per line. Data-complete today.
 */
export const eventsPanel: Panel = {
  id: DebugPanel.Events,
  title: "Events",

  render(ctx: PanelContext): string[] {
    const { devtools } = ctx;
    const w = ctx.maxWidth;
    const rows = Math.max(1, ctx.maxHeight);
    const log = devtools.getEventLog();

    if (log.length === 0) {
      return ["(no events yet)"];
    }

    const recent = log.slice(-rows);
    return recent.map((event) => {
      const glyph = CATEGORY_GLYPH[event.category] ?? "•";
      return truncate(`${glyph} ${summarize(event)}`, w);
    });
  },
};
