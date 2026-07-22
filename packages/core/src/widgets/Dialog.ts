import type { DialogOptions, KeyEvent } from "@bettertui/shared";
import type { Command } from "../command/types";
import { Renderable } from "../renderable";

export type { DialogOptions };

export class Dialog extends Renderable<DialogOptions> {
  private _open: boolean;

  constructor(options: DialogOptions = {}) {
    super(options);
    this._open = options.open ?? false;
  }

  get isOpen(): boolean {
    return this._open;
  }

  open(): void {
    this._open = true;
  }

  close(): void {
    this._open = false;
    this.opts.onClose?.();
  }

  override update(options: Partial<DialogOptions>): void {
    if (options.open !== undefined) {
      this._open = options.open;
    }
    super.update(options);
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "Box" }];

    if (!this._open) {
      // Hidden — zero size
      cmds.push({ type: "SetWidth", id, value: 0 });
      cmds.push({ type: "SetHeight", id, value: 0 });
      cmds.push({ type: "SetHidden", id, value: true });
      return cmds;
    }

    // Show dialog
    const width = this.opts.width ?? "auto";
    const height = this.opts.height ?? "auto";

    cmds.push({ type: "SetWidth", id, value: width as never });
    cmds.push({ type: "SetHeight", id, value: height as never });
    cmds.push({ type: "SetZIndex", id, value: 100 });
    cmds.push({ type: "SetPosition", id, value: "absolute" as never });
    cmds.push({ type: "SetFlexDirection", id, direction: "column" as never });
    cmds.push({ type: "SetPadding", id, value: 1 as never });

    // Title bar
    if (this.opts.title) {
      const titleId = `${id}-title`;
      cmds.push({ type: "CreateNode", id: titleId, kind: "Text" });
      cmds.push({ type: "SetText", id: titleId, text: this.opts.title });
      cmds.push({ type: "SetBold", id: titleId, value: true });
      cmds.push({ type: "AppendChild", parent: id, child: titleId });
    }

    // Content slot — children are appended by caller
    const contentId = `${id}-content`;
    cmds.push({ type: "CreateNode", id: contentId, kind: "Box" });
    cmds.push({ type: "SetFlexGrow", id: contentId, value: 1 });
    cmds.push({ type: "AppendChild", parent: id, child: contentId });

    return cmds;
  }

  override handleKey(key: KeyEvent): boolean {
    if (!this._open) return false;

    if (key.key === "escape" && this.opts.closeOnEsc !== false) {
      this.close();
      return true;
    }

    return false;
  }
}
