import type { Command } from "../command/command.types";
import type { HighlightedLine } from "../platform/binding";
import { highlightCode } from "../platform/binding";
import { Renderable } from "../renderable";
import type { CodeOptions } from "./widget.types";

export type { CodeOptions };

export class Code extends Renderable<CodeOptions> {
  private _content: string;

  constructor(content = "", options: CodeOptions = {}) {
    super(options);
    this._content = content;
  }

  get content(): string {
    return this._content;
  }

  set content(value: string) {
    this._content = value;
  }

  override update(options: Partial<CodeOptions & { content?: string }>): void {
    if (options.content !== undefined) {
      this._content = options.content;
    }
    super.update(options as Partial<CodeOptions>);
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "Code" }];
    cmds.push({ type: "SetBackground", id, color: "bright_black" as never });

    if (!this._content) return cmds;

    const language = this.opts.language;

    // Use native tree-sitter syntax highlighting when language is provided
    if (language) {
      const highlighted = highlightCode(this._content, language);
      if (highlighted.length > 0) {
        this._renderHighlighted(id, highlighted, cmds);
        return cmds;
      }
    }

    // Fallback: plain text
    cmds.push({ type: "SetText", id, text: this._content });
    return cmds;
  }

  private _renderHighlighted(id: string, lines: HighlightedLine[], cmds: Command[]): void {
    cmds.push({ type: "SetFlexDirection", id, direction: "column" as never });

    for (let lineIdx = 0; lineIdx < lines.length; lineIdx++) {
      const line = lines[lineIdx];
      if (!line) continue;

      const lineId = `${id}-ln-${lineIdx}`;
      cmds.push({ type: "CreateNode", id: lineId, kind: "Box" });
      cmds.push({ type: "SetFlexDirection", id: lineId, direction: "row" as never });
      cmds.push({ type: "AppendChild", parent: id, child: lineId });

      for (let segIdx = 0; segIdx < line.segments.length; segIdx++) {
        const seg = line.segments[segIdx];
        if (!seg) continue;

        const segId = `${id}-ln-${lineIdx}-seg-${segIdx}`;
        cmds.push({ type: "CreateNode", id: segId, kind: "Text" });
        cmds.push({ type: "SetText", id: segId, text: seg.text });

        if (seg.fg && seg.fg !== "default") {
          cmds.push({ type: "SetForeground", id: segId, color: seg.fg as never });
        }
        if (seg.bg && seg.bg !== "default") {
          cmds.push({ type: "SetBackground", id: segId, color: seg.bg as never });
        }
        if (seg.bold === true) {
          cmds.push({ type: "SetBold", id: segId, value: true });
        }
        if (seg.italic === true) {
          cmds.push({ type: "SetItalic", id: segId, value: true });
        }
        if (seg.underline === true) {
          cmds.push({ type: "SetUnderline", id: segId, value: true });
        }
        if (seg.dim === true) {
          cmds.push({ type: "SetDim", id: segId, value: true });
        }
        if (seg.strikethrough === true) {
          cmds.push({ type: "SetStrikethrough", id: segId, value: true });
        }

        cmds.push({ type: "AppendChild", parent: lineId, child: segId });
      }
    }
  }
}
