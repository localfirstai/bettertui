import type { ScrollBoxOptions } from "@bettertui/shared";
import type { KeyEvent } from "@bettertui/shared";
import type { Command } from "../command/types";
import { Renderable } from "../renderable";

export type { ScrollBoxOptions };

export class ScrollBox extends Renderable<ScrollBoxOptions> {
  private _offsetX = 0;
  private _offsetY = 0;
  private _contentWidth = 0;
  private _contentHeight = 0;
  private _viewWidth = 0;
  private _viewHeight = 0;

  constructor(options: ScrollBoxOptions = {}) {
    super(options);
  }

  get offsetX(): number {
    return this._offsetX;
  }

  get offsetY(): number {
    return this._offsetY;
  }

  /** Set content dimensions for proper scroll clamping. */
  setContentSize(width: number, height: number): void {
    this._contentWidth = width;
    this._contentHeight = height;
  }

  /** Set viewport dimensions. Called by renderer on layout. */
  setViewSize(width: number, height: number): void {
    this._viewWidth = width;
    this._viewHeight = height;
  }

  scrollTo(x: number, y: number): void {
    const maxX = Math.max(0, this._contentWidth - this._viewWidth);
    const maxY = Math.max(0, this._contentHeight - this._viewHeight);
    this._offsetX = Math.max(0, Math.min(x, maxX));
    this._offsetY = Math.max(0, Math.min(y, maxY));
    this.opts.onScroll?.(this._offsetX, this._offsetY);
  }

  scrollBy(dx: number, dy: number): void {
    this.scrollTo(this._offsetX + dx, this._offsetY + dy);
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "ScrollBox" }];
    if (this.opts.width !== undefined) {
      cmds.push({ type: "SetWidth", id, value: this.opts.width as never });
    }
    if (this.opts.height !== undefined) {
      cmds.push({ type: "SetHeight", id, value: this.opts.height as never });
    }
    cmds.push({ type: "SetOverflow", id, value: "hidden" as never });
    // Pass scroll offsets as attributes for the engine to consume
    cmds.push({ type: "SetAttribute", id, key: "scrollOffsetX", value: String(this._offsetX) });
    cmds.push({ type: "SetAttribute", id, key: "scrollOffsetY", value: String(this._offsetY) });
    return cmds;
  }

  override handleKey(key: KeyEvent): boolean {
    const scrollY = this.opts.scrollY !== false; // default enabled
    const scrollX = this.opts.scrollX === true;

    if (scrollY) {
      if (key.key === "up") {
        this.scrollBy(0, -1);
        return true;
      }
      if (key.key === "down") {
        this.scrollBy(0, 1);
        return true;
      }
      if (key.key === "pageup") {
        const pageSize = Math.max(1, this._viewHeight - 1);
        this.scrollBy(0, -pageSize);
        return true;
      }
      if (key.key === "pagedown") {
        const pageSize = Math.max(1, this._viewHeight - 1);
        this.scrollBy(0, pageSize);
        return true;
      }
      if (key.key === "home") {
        this.scrollTo(this._offsetX, 0);
        return true;
      }
      if (key.key === "end") {
        this.scrollTo(this._offsetX, this._contentHeight);
        return true;
      }
    }

    if (scrollX) {
      if (key.key === "left") {
        this.scrollBy(-1, 0);
        return true;
      }
      if (key.key === "right") {
        this.scrollBy(1, 0);
        return true;
      }
    }

    return false;
  }
}
