import type { Command } from "../command/command.types";
import { Renderable } from "../renderable";
import type { MarkdownOptions, MarkdownTheme } from "./widget.types";

export type { MarkdownOptions, MarkdownTheme };

// ─── Token types ─────────────────────────────────────────────────────────────

type InlineToken =
  | { kind: "text"; text: string }
  | { kind: "bold"; children: InlineToken[] }
  | { kind: "italic"; children: InlineToken[] }
  | { kind: "bold_italic"; children: InlineToken[] }
  | { kind: "code"; text: string }
  | { kind: "strikethrough"; children: InlineToken[] }
  | { kind: "link"; text: string; href: string };

type BlockToken =
  | { kind: "heading"; level: 1 | 2 | 3 | 4 | 5 | 6; inline: InlineToken[] }
  | { kind: "paragraph"; inline: InlineToken[] }
  | { kind: "code_block"; text: string; lang: string }
  | { kind: "blockquote"; lines: string[] }
  | { kind: "hr" }
  | { kind: "blank" }
  | { kind: "unordered_list"; items: InlineToken[][] }
  | { kind: "ordered_list"; items: InlineToken[][]; start: number };

// ─── Default theme ────────────────────────────────────────────────────────────

const DEFAULT_THEME: Required<MarkdownTheme> = {
  h1Color: "white",
  h2Color: "cyan",
  h3Color: "blue",
  codeColor: "yellow",
  codeBg: "bright_black",
  blockquoteColor: "bright_black",
  bulletColor: "bright_cyan",
  linkColor: "blue",
  hrColor: "bright_black",
};

// ─── Inline parser ────────────────────────────────────────────────────────────

/**
 * Parse an inline markdown string into a flat list of inline tokens.
 * Handles: **bold**, *italic*, ***bold-italic***, `code`, ~~strikethrough~~, [link](url).
 */
function parseInline(text: string): InlineToken[] {
  const tokens: InlineToken[] = [];
  let i = 0;

  while (i < text.length) {
    // Link: [text](url)
    if (text[i] === "[") {
      const closeLabel = text.indexOf("]", i);
      if (closeLabel !== -1 && text[closeLabel + 1] === "(") {
        const closeHref = text.indexOf(")", closeLabel + 2);
        if (closeHref !== -1) {
          const linkText = text.slice(i + 1, closeLabel);
          const href = text.slice(closeLabel + 2, closeHref);
          tokens.push({ kind: "link", text: linkText, href });
          i = closeHref + 1;
          continue;
        }
      }
    }

    // Inline code: `code`
    if (text[i] === "`") {
      const end = text.indexOf("`", i + 1);
      if (end !== -1) {
        tokens.push({ kind: "code", text: text.slice(i + 1, end) });
        i = end + 1;
        continue;
      }
    }

    // Bold-italic: ***text***
    if (text.slice(i, i + 3) === "***") {
      const end = text.indexOf("***", i + 3);
      if (end !== -1) {
        tokens.push({ kind: "bold_italic", children: parseInline(text.slice(i + 3, end)) });
        i = end + 3;
        continue;
      }
    }

    // Bold: **text**
    if (text.slice(i, i + 2) === "**") {
      const end = text.indexOf("**", i + 2);
      if (end !== -1) {
        tokens.push({ kind: "bold", children: parseInline(text.slice(i + 2, end)) });
        i = end + 2;
        continue;
      }
    }

    // Strikethrough: ~~text~~
    if (text.slice(i, i + 2) === "~~") {
      const end = text.indexOf("~~", i + 2);
      if (end !== -1) {
        tokens.push({ kind: "strikethrough", children: parseInline(text.slice(i + 2, end)) });
        i = end + 2;
        continue;
      }
    }

    // Italic: *text* or _text_
    if ((text[i] === "*" || text[i] === "_") && text[i + 1] !== text[i]) {
      const marker = text[i] as string;
      const end = text.indexOf(marker, i + 1);
      if (end !== -1) {
        tokens.push({ kind: "italic", children: parseInline(text.slice(i + 1, end)) });
        i = end + 1;
        continue;
      }
    }

    // Accumulate plain text
    const start = i;
    while (
      i < text.length &&
      text[i] !== "[" &&
      text[i] !== "`" &&
      !(text.slice(i, i + 3) === "***") &&
      !(text.slice(i, i + 2) === "**") &&
      !(text.slice(i, i + 2) === "~~") &&
      !(text[i] === "*" && text[i + 1] !== "*") &&
      !(text[i] === "_" && text[i + 1] !== "_")
    ) {
      i++;
    }
    if (i > start) {
      tokens.push({ kind: "text", text: text.slice(start, i) });
    } else {
      // Avoid infinite loops on unmatched markers — consume one char as text
      tokens.push({ kind: "text", text: text[i] as string });
      i++;
    }
  }

  return tokens;
}

// ─── Block parser ─────────────────────────────────────────────────────────────

function parseBlocks(content: string): BlockToken[] {
  const lines = content.split(/\r?\n/);
  const blocks: BlockToken[] = [];
  let i = 0;

  while (i < lines.length) {
    const line = lines[i] as string;

    // Blank line
    if (line.trim() === "") {
      blocks.push({ kind: "blank" });
      i++;
      continue;
    }

    // ATX heading: # to ######
    const headingMatch = /^(#{1,6})\s+(.+)$/.exec(line);
    if (headingMatch) {
      const level = (headingMatch[1] as string).length as 1 | 2 | 3 | 4 | 5 | 6;
      const text = (headingMatch[2] as string).trim();
      blocks.push({ kind: "heading", level, inline: parseInline(text) });
      i++;
      continue;
    }

    // Horizontal rule: --- or *** or ___
    if (/^(\-{3,}|\*{3,}|_{3,})\s*$/.test(line)) {
      blocks.push({ kind: "hr" });
      i++;
      continue;
    }

    // Fenced code block: ``` or ~~~
    const fenceMatch = /^(`{3,}|~{3,})(\w*)/.exec(line);
    if (fenceMatch) {
      const fence = fenceMatch[1] as string;
      const lang = fenceMatch[2] ?? "";
      const codeLines: string[] = [];
      i++;
      while (i < lines.length && !(lines[i] as string).trimStart().startsWith(fence)) {
        codeLines.push(lines[i] as string);
        i++;
      }
      if (i < lines.length) i++; // consume closing fence
      blocks.push({ kind: "code_block", text: codeLines.join("\n"), lang });
      continue;
    }

    // Blockquote: > text
    if (/^>\s*/.test(line)) {
      const quoteLines: string[] = [];
      while (i < lines.length && /^>\s*/.test(lines[i] as string)) {
        quoteLines.push((lines[i] as string).replace(/^>\s*/, ""));
        i++;
      }
      blocks.push({ kind: "blockquote", lines: quoteLines });
      continue;
    }

    // Unordered list: - or * or +
    if (/^[\-\*\+]\s/.test(line)) {
      const items: InlineToken[][] = [];
      while (i < lines.length && /^[\-\*\+]\s/.test(lines[i] as string)) {
        const itemText = (lines[i] as string).replace(/^[\-\*\+]\s+/, "");
        items.push(parseInline(itemText));
        i++;
      }
      blocks.push({ kind: "unordered_list", items });
      continue;
    }

    // Ordered list: 1. 2. etc.
    const olMatch = /^(\d+)\.\s+(.+)$/.exec(line);
    if (olMatch) {
      const start = Number.parseInt(olMatch[1] as string, 10);
      const items: InlineToken[][] = [];
      while (i < lines.length && /^\d+\.\s+/.test(lines[i] as string)) {
        const itemText = (lines[i] as string).replace(/^\d+\.\s+/, "");
        items.push(parseInline(itemText));
        i++;
      }
      blocks.push({ kind: "ordered_list", items, start });
      continue;
    }

    // Paragraph — accumulate until blank or special block
    const paraLines: string[] = [];
    while (
      i < lines.length &&
      (lines[i] as string).trim() !== "" &&
      !/^#{1,6}\s/.test(lines[i] as string) &&
      !/^(`{3,}|~{3,})/.test(lines[i] as string) &&
      !/^>\s/.test(lines[i] as string) &&
      !/^[\-\*\+]\s/.test(lines[i] as string) &&
      !/^\d+\.\s/.test(lines[i] as string) &&
      !/^(\-{3,}|\*{3,}|_{3,})\s*$/.test(lines[i] as string)
    ) {
      paraLines.push(lines[i] as string);
      i++;
    }
    if (paraLines.length > 0) {
      blocks.push({ kind: "paragraph", inline: parseInline(paraLines.join(" ")) });
    }
  }

  return blocks;
}

// ─── Command emitter ──────────────────────────────────────────────────────────

function emitInline(
  tokens: InlineToken[],
  parentId: string,
  prefix: string,
  cmds: Command[],
  theme: Required<MarkdownTheme>,
  opts: { bold?: boolean; italic?: boolean; fg?: string },
): void {
  for (let ti = 0; ti < tokens.length; ti++) {
    const tok = tokens[ti];
    if (!tok) continue;
    const segId = `${parentId}-seg-${ti}`;

    switch (tok.kind) {
      case "text": {
        cmds.push({ type: "CreateNode", id: segId, kind: "Text" });
        cmds.push({ type: "SetText", id: segId, text: tok.text });
        if (opts.bold) cmds.push({ type: "SetBold", id: segId, value: true });
        if (opts.italic) cmds.push({ type: "SetItalic", id: segId, value: true });
        if (opts.fg) cmds.push({ type: "SetForeground", id: segId, color: opts.fg as never });
        cmds.push({ type: "AppendChild", parent: parentId, child: segId });
        break;
      }
      case "bold": {
        const boldWrap = `${segId}-bw`;
        cmds.push({ type: "CreateNode", id: boldWrap, kind: "Box" });
        cmds.push({ type: "SetFlexDirection", id: boldWrap, direction: "row" as never });
        emitInline(tok.children, boldWrap, prefix, cmds, theme, {
          ...opts,
          bold: true,
        });
        cmds.push({ type: "AppendChild", parent: parentId, child: boldWrap });
        break;
      }
      case "italic": {
        const italicWrap = `${segId}-iw`;
        cmds.push({ type: "CreateNode", id: italicWrap, kind: "Box" });
        cmds.push({ type: "SetFlexDirection", id: italicWrap, direction: "row" as never });
        emitInline(tok.children, italicWrap, prefix, cmds, theme, {
          ...opts,
          italic: true,
        });
        cmds.push({ type: "AppendChild", parent: parentId, child: italicWrap });
        break;
      }
      case "bold_italic": {
        const biWrap = `${segId}-biw`;
        cmds.push({ type: "CreateNode", id: biWrap, kind: "Box" });
        cmds.push({ type: "SetFlexDirection", id: biWrap, direction: "row" as never });
        emitInline(tok.children, biWrap, prefix, cmds, theme, {
          ...opts,
          bold: true,
          italic: true,
        });
        cmds.push({ type: "AppendChild", parent: parentId, child: biWrap });
        break;
      }
      case "strikethrough": {
        const stWrap = `${segId}-stw`;
        cmds.push({ type: "CreateNode", id: stWrap, kind: "Box" });
        cmds.push({ type: "SetFlexDirection", id: stWrap, direction: "row" as never });
        emitInline(tok.children, stWrap, prefix, cmds, theme, opts);
        // Strikethrough applied to all text children inside
        cmds.push({ type: "SetStrikethrough", id: stWrap, value: true });
        cmds.push({ type: "AppendChild", parent: parentId, child: stWrap });
        break;
      }
      case "code": {
        cmds.push({ type: "CreateNode", id: segId, kind: "Text" });
        cmds.push({ type: "SetText", id: segId, text: ` ${tok.text} ` });
        cmds.push({ type: "SetForeground", id: segId, color: theme.codeColor as never });
        if (theme.codeBg)
          cmds.push({ type: "SetBackground", id: segId, color: theme.codeBg as never });
        cmds.push({ type: "AppendChild", parent: parentId, child: segId });
        break;
      }
      case "link": {
        cmds.push({ type: "CreateNode", id: segId, kind: "Text" });
        cmds.push({ type: "SetText", id: segId, text: tok.text });
        cmds.push({ type: "SetForeground", id: segId, color: theme.linkColor as never });
        cmds.push({ type: "SetUnderline", id: segId, value: true });
        cmds.push({ type: "AppendChild", parent: parentId, child: segId });
        break;
      }
    }
  }
}

// ─── Widget ───────────────────────────────────────────────────────────────────

export class Markdown extends Renderable<MarkdownOptions> {
  private _content = "";
  private _theme: Required<MarkdownTheme>;

  constructor(options: MarkdownOptions = {}) {
    super(options);
    this._content = options.content ?? "";
    this._theme = { ...DEFAULT_THEME, ...(options.theme ?? {}) };
  }

  get content(): string {
    return this._content;
  }

  set content(value: string) {
    this._content = value;
  }

  override update(options: Partial<MarkdownOptions>): void {
    if (options.content !== undefined) this._content = options.content;
    if (options.theme !== undefined) this._theme = { ...DEFAULT_THEME, ...options.theme };
    super.update(options);
  }

  renderCommands(id: string): Command[] {
    const cmds: Command[] = [{ type: "CreateNode", id, kind: "Box" }];
    cmds.push({ type: "SetFlexDirection", id, direction: "column" as never });

    if (!this._content) return cmds;

    const blocks = parseBlocks(this._content);
    const theme = this._theme;

    for (let bi = 0; bi < blocks.length; bi++) {
      const block = blocks[bi];
      if (!block) continue;
      const blockId = `${id}-b${bi}`;

      switch (block.kind) {
        case "blank":
          // Skip rendering blank lines
          break;

        case "heading": {
          cmds.push({ type: "CreateNode", id: blockId, kind: "Box" });
          cmds.push({ type: "SetFlexDirection", id: blockId, direction: "row" as never });
          const hColor =
            block.level === 1 ? theme.h1Color : block.level === 2 ? theme.h2Color : theme.h3Color;
          const prefix = `${"#".repeat(block.level)} `;
          const prefixId = `${blockId}-hpfx`;
          cmds.push({ type: "CreateNode", id: prefixId, kind: "Text" });
          cmds.push({ type: "SetText", id: prefixId, text: prefix });
          cmds.push({ type: "SetForeground", id: prefixId, color: hColor as never });
          cmds.push({ type: "SetBold", id: prefixId, value: true });
          cmds.push({ type: "AppendChild", parent: blockId, child: prefixId });
          emitInline(block.inline, blockId, prefix, cmds, theme, {
            bold: block.level <= 2,
            fg: hColor,
          });
          cmds.push({ type: "AppendChild", parent: id, child: blockId });
          break;
        }

        case "paragraph": {
          cmds.push({ type: "CreateNode", id: blockId, kind: "Box" });
          cmds.push({ type: "SetFlexDirection", id: blockId, direction: "row" as never });
          emitInline(block.inline, blockId, "", cmds, theme, {});
          cmds.push({ type: "AppendChild", parent: id, child: blockId });
          break;
        }

        case "code_block": {
          cmds.push({ type: "CreateNode", id: blockId, kind: "Box" });
          cmds.push({ type: "SetFlexDirection", id: blockId, direction: "column" as never });
          cmds.push({ type: "SetBackground", id: blockId, color: theme.codeBg as never });
          const codeLines = block.text.split("\n");
          for (let li = 0; li < codeLines.length; li++) {
            const lineId = `${blockId}-cl${li}`;
            cmds.push({ type: "CreateNode", id: lineId, kind: "Text" });
            cmds.push({ type: "SetText", id: lineId, text: codeLines[li] as string });
            cmds.push({ type: "SetForeground", id: lineId, color: theme.codeColor as never });
            cmds.push({ type: "AppendChild", parent: blockId, child: lineId });
          }
          cmds.push({ type: "AppendChild", parent: id, child: blockId });
          break;
        }

        case "blockquote": {
          cmds.push({ type: "CreateNode", id: blockId, kind: "Box" });
          cmds.push({ type: "SetFlexDirection", id: blockId, direction: "column" as never });
          for (let qi = 0; qi < block.lines.length; qi++) {
            const qLineId = `${blockId}-ql${qi}`;
            cmds.push({ type: "CreateNode", id: qLineId, kind: "Box" });
            cmds.push({ type: "SetFlexDirection", id: qLineId, direction: "row" as never });
            const gutId = `${qLineId}-gut`;
            cmds.push({ type: "CreateNode", id: gutId, kind: "Text" });
            cmds.push({ type: "SetText", id: gutId, text: "│ " });
            cmds.push({ type: "SetForeground", id: gutId, color: theme.blockquoteColor as never });
            cmds.push({ type: "AppendChild", parent: qLineId, child: gutId });
            const textId = `${qLineId}-t`;
            cmds.push({ type: "CreateNode", id: textId, kind: "Text" });
            cmds.push({ type: "SetText", id: textId, text: block.lines[qi] as string });
            cmds.push({ type: "SetForeground", id: textId, color: theme.blockquoteColor as never });
            cmds.push({ type: "SetItalic", id: textId, value: true });
            cmds.push({ type: "AppendChild", parent: qLineId, child: textId });
            cmds.push({ type: "AppendChild", parent: blockId, child: qLineId });
          }
          cmds.push({ type: "AppendChild", parent: id, child: blockId });
          break;
        }

        case "hr": {
          const hrId = blockId;
          cmds.push({ type: "CreateNode", id: hrId, kind: "Text" });
          cmds.push({ type: "SetText", id: hrId, text: "─".repeat(40) });
          cmds.push({ type: "SetForeground", id: hrId, color: theme.hrColor as never });
          cmds.push({ type: "SetDim", id: hrId, value: true });
          cmds.push({ type: "AppendChild", parent: id, child: hrId });
          break;
        }

        case "unordered_list": {
          cmds.push({ type: "CreateNode", id: blockId, kind: "Box" });
          cmds.push({ type: "SetFlexDirection", id: blockId, direction: "column" as never });
          for (let ii = 0; ii < block.items.length; ii++) {
            const itemId = `${blockId}-li${ii}`;
            cmds.push({ type: "CreateNode", id: itemId, kind: "Box" });
            cmds.push({ type: "SetFlexDirection", id: itemId, direction: "row" as never });
            const bulletId = `${itemId}-bul`;
            cmds.push({ type: "CreateNode", id: bulletId, kind: "Text" });
            cmds.push({ type: "SetText", id: bulletId, text: "• " });
            cmds.push({ type: "SetForeground", id: bulletId, color: theme.bulletColor as never });
            cmds.push({ type: "AppendChild", parent: itemId, child: bulletId });
            emitInline(block.items[ii] ?? [], itemId, "", cmds, theme, {});
            cmds.push({ type: "AppendChild", parent: blockId, child: itemId });
          }
          cmds.push({ type: "AppendChild", parent: id, child: blockId });
          break;
        }

        case "ordered_list": {
          cmds.push({ type: "CreateNode", id: blockId, kind: "Box" });
          cmds.push({ type: "SetFlexDirection", id: blockId, direction: "column" as never });
          for (let ii = 0; ii < block.items.length; ii++) {
            const itemId = `${blockId}-li${ii}`;
            cmds.push({ type: "CreateNode", id: itemId, kind: "Box" });
            cmds.push({ type: "SetFlexDirection", id: itemId, direction: "row" as never });
            const numId = `${itemId}-num`;
            cmds.push({ type: "CreateNode", id: numId, kind: "Text" });
            cmds.push({ type: "SetText", id: numId, text: `${block.start + ii}. ` });
            cmds.push({ type: "SetForeground", id: numId, color: theme.bulletColor as never });
            cmds.push({ type: "AppendChild", parent: itemId, child: numId });
            emitInline(block.items[ii] ?? [], itemId, "", cmds, theme, {});
            cmds.push({ type: "AppendChild", parent: blockId, child: itemId });
          }
          cmds.push({ type: "AppendChild", parent: id, child: blockId });
          break;
        }
      }
    }

    return cmds;
  }
}
