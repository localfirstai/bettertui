/**
 * Styled text system for BetterTUI.
 * Provides a template literal tag `t` and style helpers for building
 * rich text with colors, bold, italic, and other attributes.
 */

import { type ColorInput, type RGBA, parseColor } from "./rgba";

/** TextAttributes bitmask constants. */
export const TextAttributes = {
  NONE: 0,
  BOLD: 1,
  DIM: 2,
  ITALIC: 4,
  UNDERLINE: 8,
  BLINK: 16,
  INVERSE: 32,
  HIDDEN: 64,
  STRIKETHROUGH: 128,
} as const;

export type TextAttributeFlag = (typeof TextAttributes)[keyof typeof TextAttributes];

/** A single styled text chunk. */
export interface TextChunk {
  __isChunk: true;
  text: string;
  fg?: RGBA;
  bg?: RGBA;
  attributes?: number;
  link?: { url: string };
}

const BrandedStyledText: unique symbol = Symbol.for("@bettertui/core/StyledText");

/** A rich text object made of styled chunks. */
export class StyledText {
  [BrandedStyledText] = true;
  public chunks: TextChunk[];

  constructor(chunks: TextChunk[]) {
    this.chunks = chunks;
  }
}

/** Type guard for StyledText. */
export function isStyledText(obj: unknown): obj is StyledText {
  return !!(obj as Record<symbol, unknown>)?.[BrandedStyledText];
}

/** Convert a plain string to a StyledText with one chunk. */
export function stringToStyledText(content: string): StyledText {
  return new StyledText([{ __isChunk: true, text: content }]);
}

/** A value that can be used in styled text. */
export type StylableInput = string | number | boolean | TextChunk;

interface StyleAttrs {
  fg?: ColorInput;
  bg?: ColorInput;
  attributes?: number;
}

function applyStyle(input: StylableInput, attrs: StyleAttrs): TextChunk {
  const fg = attrs.fg !== undefined ? parseColor(attrs.fg) : undefined;
  const bg = attrs.bg !== undefined ? parseColor(attrs.bg) : undefined;
  const newAttrs = attrs.attributes ?? 0;

  if (typeof input === "object" && "__isChunk" in (input as object)) {
    const existing = input as TextChunk;
    return {
      __isChunk: true,
      text: existing.text,
      fg: fg !== undefined ? fg : existing.fg,
      bg: bg !== undefined ? bg : existing.bg,
      attributes: newAttrs ? (existing.attributes ?? 0) | newAttrs : existing.attributes,
      link: existing.link,
    };
  }

  return {
    __isChunk: true,
    text: String(input),
    fg,
    bg,
    attributes: newAttrs || undefined,
  };
}

/**
 * Template literal tag for building styled text.
 *
 * @example
 * t`${bold(red("Error:"))} Connection failed`
 */
export function t(strings: TemplateStringsArray, ...values: StylableInput[]): StyledText {
  const chunks: TextChunk[] = [];
  for (let i = 0; i < strings.length; i++) {
    const raw = strings[i];
    if (raw) chunks.push({ __isChunk: true, text: raw });
    const val = values[i];
    if (val !== undefined) {
      if (typeof val === "object" && "__isChunk" in (val as object)) {
        chunks.push(val as TextChunk);
      } else {
        chunks.push({ __isChunk: true, text: String(val) });
      }
    }
  }
  return new StyledText(chunks);
}

// ── Style attribute helpers ───────────────────────────────────────────────────

export const bold = (input: StylableInput): TextChunk =>
  applyStyle(input, { attributes: TextAttributes.BOLD });
export const italic = (input: StylableInput): TextChunk =>
  applyStyle(input, { attributes: TextAttributes.ITALIC });
export const underline = (input: StylableInput): TextChunk =>
  applyStyle(input, { attributes: TextAttributes.UNDERLINE });
export const strikethrough = (input: StylableInput): TextChunk =>
  applyStyle(input, { attributes: TextAttributes.STRIKETHROUGH });
export const dim = (input: StylableInput): TextChunk =>
  applyStyle(input, { attributes: TextAttributes.DIM });
export const reverse = (input: StylableInput): TextChunk =>
  applyStyle(input, { attributes: TextAttributes.INVERSE });
export const blink = (input: StylableInput): TextChunk =>
  applyStyle(input, { attributes: TextAttributes.BLINK });

// ── Named foreground color helpers ───────────────────────────────────────────

export const black = (input: StylableInput): TextChunk => applyStyle(input, { fg: "black" });
export const red = (input: StylableInput): TextChunk => applyStyle(input, { fg: "red" });
export const green = (input: StylableInput): TextChunk => applyStyle(input, { fg: "green" });
export const yellow = (input: StylableInput): TextChunk => applyStyle(input, { fg: "yellow" });
export const blue = (input: StylableInput): TextChunk => applyStyle(input, { fg: "blue" });
export const magenta = (input: StylableInput): TextChunk => applyStyle(input, { fg: "magenta" });
export const cyan = (input: StylableInput): TextChunk => applyStyle(input, { fg: "cyan" });
export const white = (input: StylableInput): TextChunk => applyStyle(input, { fg: "white" });

// Bright variants
export const brightBlack = (input: StylableInput): TextChunk =>
  applyStyle(input, { fg: "brightblack" });
export const brightRed = (input: StylableInput): TextChunk =>
  applyStyle(input, { fg: "brightred" });
export const brightGreen = (input: StylableInput): TextChunk =>
  applyStyle(input, { fg: "brightgreen" });
export const brightYellow = (input: StylableInput): TextChunk =>
  applyStyle(input, { fg: "brightyellow" });
export const brightBlue = (input: StylableInput): TextChunk =>
  applyStyle(input, { fg: "brightblue" });
export const brightMagenta = (input: StylableInput): TextChunk =>
  applyStyle(input, { fg: "brightmagenta" });
export const brightCyan = (input: StylableInput): TextChunk =>
  applyStyle(input, { fg: "brightcyan" });
export const brightWhite = (input: StylableInput): TextChunk =>
  applyStyle(input, { fg: "brightwhite" });

// ── Named background color helpers ───────────────────────────────────────────

export const bgBlack = (input: StylableInput): TextChunk => applyStyle(input, { bg: "black" });
export const bgRed = (input: StylableInput): TextChunk => applyStyle(input, { bg: "red" });
export const bgGreen = (input: StylableInput): TextChunk => applyStyle(input, { bg: "green" });
export const bgYellow = (input: StylableInput): TextChunk => applyStyle(input, { bg: "yellow" });
export const bgBlue = (input: StylableInput): TextChunk => applyStyle(input, { bg: "blue" });
export const bgMagenta = (input: StylableInput): TextChunk => applyStyle(input, { bg: "magenta" });
export const bgCyan = (input: StylableInput): TextChunk => applyStyle(input, { bg: "cyan" });
export const bgWhite = (input: StylableInput): TextChunk => applyStyle(input, { bg: "white" });

// ── Curried custom-color helpers ──────────────────────────────────────────────

/** Set foreground color. `fg("#ff0000")("text")` */
export const fg =
  (color: ColorInput) =>
  (input: StylableInput): TextChunk =>
    applyStyle(input, { fg: color });

/** Set background color. `bg("#ff0000")("text")` */
export const bg =
  (color: ColorInput) =>
  (input: StylableInput): TextChunk =>
    applyStyle(input, { bg: color });

/** Create a hyperlink. `link("https://example.com")("click here")` */
export const link =
  (url: string) =>
  (input: StylableInput): TextChunk => {
    const base =
      typeof input === "object" && "__isChunk" in (input as object)
        ? (input as TextChunk)
        : ({ __isChunk: true, text: String(input) } as TextChunk);
    return { ...base, link: { url } };
  };

// ── ANSI conversion ───────────────────────────────────────────────────────────

/** Convert a StyledText or string to an ANSI escape-code string. */
export function styledTextToAnsi(styledText: StyledText | string): string {
  if (typeof styledText === "string") return styledText;
  let result = "";
  for (const chunk of styledText.chunks) {
    let prefix = "";
    const suffix_parts: string[] = [];

    if (chunk.fg) prefix += `\x1b[38;2;${chunk.fg.r};${chunk.fg.g};${chunk.fg.b}m`;
    if (chunk.bg) prefix += `\x1b[48;2;${chunk.bg.r};${chunk.bg.g};${chunk.bg.b}m`;

    const attrs = chunk.attributes ?? 0;
    if (attrs & TextAttributes.BOLD) prefix += "\x1b[1m";
    if (attrs & TextAttributes.DIM) prefix += "\x1b[2m";
    if (attrs & TextAttributes.ITALIC) prefix += "\x1b[3m";
    if (attrs & TextAttributes.UNDERLINE) prefix += "\x1b[4m";
    if (attrs & TextAttributes.BLINK) prefix += "\x1b[5m";
    if (attrs & TextAttributes.INVERSE) prefix += "\x1b[7m";
    if (attrs & TextAttributes.HIDDEN) prefix += "\x1b[8m";
    if (attrs & TextAttributes.STRIKETHROUGH) prefix += "\x1b[9m";

    if (chunk.link?.url) {
      prefix += `\x1b]8;;${chunk.link.url}\x1b\\`;
      suffix_parts.push("\x1b]8;;\x1b\\");
    }

    if (prefix) suffix_parts.push("\x1b[0m");

    result += prefix + chunk.text + suffix_parts.join("");
  }
  return result;
}

/** Get the visible (non-ANSI) character width of a string. */
export function visibleWidth(str: string): number {
  // Strip ANSI codes then count wide chars as 2
  // biome-ignore lint/suspicious/noControlCharactersInRegex: ANSI escape sequences require ESC character
  const stripped = str.replace(/\x1b\[[^m]*m|\x1b\][^\x07\x1b]*[\x07\x1b\\]/g, "");
  let width = 0;
  for (const ch of stripped) {
    const cp = ch.codePointAt(0) ?? 0;
    // Basic wide character detection (CJK ranges)
    if (
      (cp >= 0x1100 && cp <= 0x115f) ||
      cp === 0x2329 ||
      cp === 0x232a ||
      (cp >= 0x2e80 && cp <= 0x303e) ||
      (cp >= 0x3040 && cp <= 0xa4cf) ||
      (cp >= 0xa960 && cp <= 0xa97f) ||
      (cp >= 0xac00 && cp <= 0xd7a3) ||
      (cp >= 0xf900 && cp <= 0xfaff) ||
      (cp >= 0xfe10 && cp <= 0xfe19) ||
      (cp >= 0xfe30 && cp <= 0xfe6f) ||
      (cp >= 0xff01 && cp <= 0xff60) ||
      (cp >= 0xffe0 && cp <= 0xffe6) ||
      (cp >= 0x1b000 && cp <= 0x1b001) ||
      (cp >= 0x1f004 && cp <= 0x1f0cf) ||
      (cp >= 0x1f200 && cp <= 0x1f251) ||
      (cp >= 0x1f300 && cp <= 0x1f9ff)
    ) {
      width += 2;
    } else {
      width += 1;
    }
  }
  return width;
}
