import type { ProgressBarOptions } from "./widget.types";
import type { Command } from "../command/command.types";
import { Renderable } from "../renderable";

export type { ProgressBarOptions };

export class ProgressBar extends Renderable<ProgressBarOptions> {
  constructor(options: ProgressBarOptions = {}) {
    super(options);
  }

  get percent(): number {
    const min = this.opts.min ?? 0;
    const max = this.opts.max ?? 100;
    const value = this.opts.value ?? 0;
    const range = max - min;
    if (range <= 0) return 0;
    return Math.max(0, Math.min(100, ((value - min) / range) * 100));
  }

  override update(options: Partial<ProgressBarOptions>): void {
    super.update(options);
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "Box" }];

    const pct = this.percent;
    const barWidth = typeof this.opts.width === "number" ? this.opts.width : 30;

    if (this.opts.width !== undefined) {
      cmds.push({ type: "SetWidth", id, value: this.opts.width as never });
    }

    cmds.push({ type: "SetFlexDirection", id, direction: "row" as never });

    // Bar track
    const trackId = `${id}-track`;
    const filledWidth = Math.round((pct / 100) * barWidth);
    const filled = "█".repeat(filledWidth);
    const empty = "░".repeat(Math.max(0, barWidth - filledWidth));

    cmds.push({ type: "CreateNode", id: trackId, kind: "Text" });
    cmds.push({ type: "SetText", id: trackId, text: `${filled}${empty}` });

    if (this.opts.color) {
      cmds.push({ type: "SetForeground", id: trackId, color: this.opts.color as never });
    }

    cmds.push({ type: "AppendChild", parent: id, child: trackId });

    // Optional percentage label
    if (this.opts.showPercent !== false) {
      const labelId = `${id}-label`;
      const displayLabel = this.opts.label
        ? ` ${this.opts.label} ${pct.toFixed(0)}%`
        : ` ${pct.toFixed(0)}%`;
      cmds.push({ type: "CreateNode", id: labelId, kind: "Text" });
      cmds.push({ type: "SetText", id: labelId, text: displayLabel });
      cmds.push({ type: "AppendChild", parent: id, child: labelId });
    } else if (this.opts.label) {
      const labelId = `${id}-label`;
      cmds.push({ type: "CreateNode", id: labelId, kind: "Text" });
      cmds.push({ type: "SetText", id: labelId, text: ` ${this.opts.label}` });
      cmds.push({ type: "AppendChild", parent: id, child: labelId });
    }

    return cmds;
  }
}
