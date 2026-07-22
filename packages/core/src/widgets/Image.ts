import type { ImageFormat, ImageOptions, ImageProtocol } from "./widget.types";
import type { Command } from "../command/command.types";
import {
  graphicsItermWrite,
  graphicsKittyDelete,
  graphicsKittyDeleteAll,
  graphicsKittyWrite,
  graphicsQuery,
  graphicsSixelWrite,
} from "../platform/binding";
import { Renderable } from "../renderable";

export type { ImageFormat, ImageOptions, ImageProtocol };
export {
  graphicsQuery,
  graphicsKittyWrite,
  graphicsKittyDelete,
  graphicsKittyDeleteAll,
  graphicsItermWrite,
  graphicsSixelWrite,
};

/**
 * Image widget — renders inline images using the terminal's graphics protocol.
 *
 * Protocol selection:
 * - `"kitty"` — Kitty graphics protocol (best quality, requires id)
 * - `"iterm2"` — iTerm2 inline-image protocol
 * - `"sixel"` — Sixel graphics (widely supported)
 * - `"auto"` (default) — tries Kitty, falls back to iTerm2, then Sixel
 *
 * The widget emits a `WriteRaw` command carrying the graphics escape sequence.
 * It is the caller's responsibility to ensure the terminal supports the chosen
 * protocol (use `graphicsQuery()` to probe capabilities).
 */
export class Image extends Renderable<ImageOptions> {
  private static _nextId = 1;

  /** The resolved image id (auto-assigned if not provided). */
  get imageId(): number {
    return this.opts.id ?? 0;
  }

  /**
   * Build and return the raw escape sequence bytes for the current image data.
   * Write these bytes to stdout to display the image.
   */
  buildSequence(protocol?: ImageProtocol): Buffer {
    const proto = protocol ?? this.opts.protocol ?? "auto";
    const { data, width, height, format = "png", name } = this.opts;
    const id = this.opts.id ?? Image._nextId++;

    switch (proto) {
      case "kitty":
        return graphicsKittyWrite(format, width, height, data, id);
      case "iterm2":
        return graphicsItermWrite(data, name, width, height);
      case "sixel":
        return graphicsSixelWrite(format, width, height, data);
      case "auto": {
        // Try Kitty first (best quality), then iTerm2, then Sixel
        const kitty = graphicsKittyWrite(format, width, height, data, id);
        if (kitty.length > 0) return kitty;
        const iterm = graphicsItermWrite(data, name, width, height);
        if (iterm.length > 0) return iterm;
        return graphicsSixelWrite(format, width, height, data);
      }
    }
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "Box" }];
    cmds.push({ type: "SetWidth", id, value: this.opts.width });
    cmds.push({ type: "SetHeight", id, value: this.opts.height });

    // Emit the raw graphics escape sequence via WriteRaw attribute
    const seq = this.buildSequence();
    if (seq.length > 0) {
      cmds.push({
        type: "SetAttribute",
        id,
        key: "rawSequence",
        value: seq.toString("base64"),
      });
    }

    return cmds;
  }
}
