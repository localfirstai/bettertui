import type { SpinnerOptions, SpinnerVariant } from "@bettertui/shared";
import type { Command } from "../command/types";
import { Renderable } from "../renderable";

export type { SpinnerOptions, SpinnerVariant };

const SPINNER_FRAMES: Record<SpinnerVariant, string[]> = {
  dots: ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
  line: ["-", "\\", "|", "/"],
  arc: ["◜", "◠", "◝", "◞", "◡", "◟"],
  bounce: ["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"],
  pipe: ["┤", "┘", "┴", "└", "├", "┌", "┬", "┐"],
  clock: ["🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚", "🕛"],
  earth: ["🌍", "🌎", "🌏"],
  moon: ["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"],
  pulse: ["█", "▓", "▒", "░", "▒", "▓"],
  star: ["✶", "✸", "✹", "✺", "✹", "✷"],
};

export class Spinner extends Renderable<SpinnerOptions> {
  private _frameIndex = 0;
  private _timer: ReturnType<typeof setInterval> | null = null;

  constructor(options: SpinnerOptions = {}) {
    super(options);
  }

  get currentFrame(): string {
    const variant: SpinnerVariant = this.opts.variant ?? "dots";
    const frames = SPINNER_FRAMES[variant];
    return frames[this._frameIndex % frames.length] ?? frames[0] ?? "⠋";
  }

  /** Call each frame to advance the animation. Returns true if frame changed. */
  tick(): boolean {
    const variant: SpinnerVariant = this.opts.variant ?? "dots";
    const frames = SPINNER_FRAMES[variant];
    this._frameIndex = (this._frameIndex + 1) % frames.length;
    return true;
  }

  /** Start auto-ticking at interval (ms). Returns cleanup fn. */
  start(intervalMs?: number): () => void {
    const speed = intervalMs ?? this.opts.speed ?? 80;
    this._timer = setInterval(() => {
      this.tick();
    }, speed);
    return () => this.stop();
  }

  stop(): void {
    if (this._timer !== null) {
      clearInterval(this._timer);
      this._timer = null;
    }
  }

  override destroy(): void {
    this.stop();
    super.destroy();
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "Box" }];
    cmds.push({ type: "SetFlexDirection", id, direction: "row" as never });

    const frameId = `${id}-frame`;
    cmds.push({ type: "CreateNode", id: frameId, kind: "Text" });
    cmds.push({ type: "SetText", id: frameId, text: this.currentFrame });

    if (this.opts.color) {
      cmds.push({ type: "SetForeground", id: frameId, color: this.opts.color as never });
    }

    cmds.push({ type: "AppendChild", parent: id, child: frameId });

    if (this.opts.label) {
      const labelId = `${id}-label`;
      cmds.push({ type: "CreateNode", id: labelId, kind: "Text" });
      cmds.push({ type: "SetText", id: labelId, text: ` ${this.opts.label}` });
      cmds.push({ type: "AppendChild", parent: id, child: labelId });
    }

    return cmds;
  }
}
