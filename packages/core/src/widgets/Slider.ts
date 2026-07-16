import type { KeyEvent, SliderOptions } from "@bettertui/shared";
import type { Command } from "../command/types";
import { Renderable } from "../renderable";

export type { SliderOptions };

export class Slider extends Renderable<SliderOptions> {
  constructor(options: SliderOptions = {}) {
    super(options);
  }

  get percent(): number {
    const min = this.opts.min ?? 0;
    const max = this.opts.max ?? 100;
    const value = this.opts.value ?? 0;
    const range = max - min;
    return range > 0 ? ((value - min) / range) * 100 : 0;
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "Box" }];
    cmds.push({ type: "SetWidth", id, value: "100%" as never });
    const pct = this.percent;
    const barWidth = 20;
    const filled = Math.round((pct / 100) * barWidth);
    const bar = `${"█".repeat(filled)}${"░".repeat(Math.max(0, barWidth - filled))}`;
    cmds.push({ type: "SetText", id, text: `${bar} ${this.opts.value ?? 0}` });
    return cmds;
  }

  override handleKey(key: KeyEvent): boolean {
    if (this.opts.disabled) return false;
    const min = this.opts.min ?? 0;
    const max = this.opts.max ?? 100;
    const step = this.opts.step ?? 1;
    let value = this.opts.value ?? 0;
    if (key.key === "left" || key.key === "down") {
      value = Math.max(min, value - step);
      this.opts = { ...this.opts, value };
      this.opts.onChange?.(value);
      return true;
    }
    if (key.key === "right" || key.key === "up") {
      value = Math.min(max, value + step);
      this.opts = { ...this.opts, value };
      this.opts.onChange?.(value);
      return true;
    }
    return false;
  }
}
