import type { BadgeOptions, BadgeVariant } from "./widget.types";
import type { Command } from "../command/command.types";
import { Renderable } from "../renderable";

export type { BadgeOptions, BadgeVariant };

const VARIANT_COLORS: Record<BadgeVariant, { fg: string; bg: string }> = {
  default: { fg: "white", bg: "bright_black" },
  primary: { fg: "white", bg: "blue" },
  success: { fg: "black", bg: "green" },
  warning: { fg: "black", bg: "yellow" },
  error: { fg: "white", bg: "red" },
  info: { fg: "white", bg: "cyan" },
};

export class Badge extends Renderable<BadgeOptions> {
  renderCommands(id: string): Command[] {
    const variant: BadgeVariant = this.opts.variant ?? "default";
    const colors = VARIANT_COLORS[variant];

    const cmds: Command[] = [{ type: "CreateNode", id, kind: "Text" }];
    cmds.push({ type: "SetText", id, text: ` ${this.opts.label} ` });

    const fg = this.opts.color ?? colors.fg;
    const bg = this.opts.bgColor ?? colors.bg;

    cmds.push({ type: "SetForeground", id, color: fg as never });
    cmds.push({ type: "SetBackground", id, color: bg as never });
    cmds.push({ type: "SetBold", id, value: true });

    return cmds;
  }
}
