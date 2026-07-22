import type { ScrollBarOptions } from "@bettertui/shared";
import type { KeyEvent } from "@bettertui/shared";
import type { Command } from "../command/types";
import { Renderable } from "../renderable";

export type { ScrollBarOptions };

export class ScrollBar extends Renderable<ScrollBarOptions> {
  private _position: number;

  constructor(options: ScrollBarOptions = {}) {
    super(options);
    this._position = options.position ?? 0;
  }

  get position(): number {
    return this._position;
  }

  setPosition(value: number): void {
    const trackSize = this.opts.trackSize ?? 100;
    const thumbSize = this.opts.thumbSize ?? 10;
    const maxPos = Math.max(0, trackSize - thumbSize);
    this._position = Math.max(0, Math.min(value, maxPos));
    this.opts.onChange?.(this._position);
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "ScrollBar" }];
    const isHorizontal = this.opts.orientation === "horizontal";

    if (isHorizontal) {
      cmds.push({ type: "SetHeight", id, value: 1 });
      cmds.push({ type: "SetWidth", id, value: "100%" as never });
    } else {
      cmds.push({ type: "SetWidth", id, value: 1 });
      cmds.push({ type: "SetHeight", id, value: "100%" as never });
    }

    // Render track + thumb as text representation
    const trackSize = this.opts.trackSize ?? 20;
    const thumbSize = Math.max(1, this.opts.thumbSize ?? 3);
    const maxPos = Math.max(0, trackSize - thumbSize);
    const thumbPos = maxPos > 0 ? Math.round((this._position / 100) * maxPos) : 0;

    const track = isHorizontal
      ? buildHorizontalTrack(trackSize, thumbSize, thumbPos)
      : buildVerticalTrack(trackSize, thumbSize, thumbPos);

    cmds.push({ type: "SetText", id, text: track });
    cmds.push({ type: "SetAttribute", id, key: "scrollPosition", value: String(this._position) });
    return cmds;
  }

  override handleKey(key: KeyEvent): boolean {
    const isHorizontal = this.opts.orientation === "horizontal";
    const step = 5;

    if (isHorizontal) {
      if (key.key === "left") {
        this.setPosition(this._position - step);
        return true;
      }
      if (key.key === "right") {
        this.setPosition(this._position + step);
        return true;
      }
    } else {
      if (key.key === "up") {
        this.setPosition(this._position - step);
        return true;
      }
      if (key.key === "down") {
        this.setPosition(this._position + step);
        return true;
      }
      if (key.key === "pageup") {
        this.setPosition(this._position - 20);
        return true;
      }
      if (key.key === "pagedown") {
        this.setPosition(this._position + 20);
        return true;
      }
      if (key.key === "home") {
        this.setPosition(0);
        return true;
      }
      if (key.key === "end") {
        this.setPosition(100);
        return true;
      }
    }

    return false;
  }
}

function buildHorizontalTrack(trackSize: number, thumbSize: number, thumbPos: number): string {
  const chars = Array<string>(trackSize).fill("░");
  for (let i = thumbPos; i < thumbPos + thumbSize && i < trackSize; i++) {
    chars[i] = "█";
  }
  return chars.join("");
}

function buildVerticalTrack(trackSize: number, thumbSize: number, thumbPos: number): string {
  const lines: string[] = [];
  for (let i = 0; i < trackSize; i++) {
    lines.push(i >= thumbPos && i < thumbPos + thumbSize ? "█" : "░");
  }
  return lines.join("\n");
}
