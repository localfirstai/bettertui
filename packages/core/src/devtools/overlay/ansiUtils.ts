/**
 * ANSI helpers for the debug overlay.
 *
 * The overlay is composited by writing absolute-positioned ANSI to stdout
 * *after* the engine's frame output (the engine returns finished base64 ANSI
 * bytes; there is no TS-accessible cell buffer). These helpers build the escape
 * sequences and lay out box-drawn panels. All coordinates are 1-based to match
 * the terminal's `CSI row;col H` convention.
 */

// ─── Cursor control ──────────────────────────────────────────────────────────

/** Save the cursor position (DEC save). */
export const SAVE_CURSOR = "\x1b7";

/** Restore the cursor position (DEC restore). */
export const RESTORE_CURSOR = "\x1b8";

/** Reset all SGR attributes. */
export const RESET = "\x1b[0m";

/** Move the cursor to an absolute (row, col), both 1-based. */
export function moveTo(row: number, col: number): string {
  const r = Math.max(1, Math.floor(row));
  const c = Math.max(1, Math.floor(col));
  return `\x1b[${r};${c}H`;
}

// ─── SGR helpers ─────────────────────────────────────────────────────────────

/** Wrap text in an SGR sequence and a reset. */
export function sgr(text: string, ...codes: number[]): string {
  if (codes.length === 0) return text;
  return `\x1b[${codes.join(";")}m${text}${RESET}`;
}

/** 24-bit foreground color. */
export function fg(r: number, g: number, b: number): string {
  return `\x1b[38;2;${r};${g};${b}m`;
}

/** 24-bit background color. */
export function bg(r: number, g: number, b: number): string {
  return `\x1b[48;2;${r};${g};${b}m`;
}

// ─── String width / truncation / padding ─────────────────────────────────────

// Matches SGR/CSI escape sequences so width math ignores styling.
// biome-ignore lint/suspicious/noControlCharactersInRegex: ANSI escapes are control chars by definition
const ANSI_PATTERN = /\x1b\[[0-9;]*[A-Za-z]|\x1b[78]/g;

/** Strip ANSI escape sequences from a string. */
export function stripAnsi(text: string): string {
  return text.replace(ANSI_PATTERN, "");
}

/** Visible width of a string (ANSI-aware, treats each code point as width 1). */
export function displayWidth(text: string): number {
  return [...stripAnsi(text)].length;
}

/**
 * Truncate a string to a visible width, appending an ellipsis when clipped.
 * ANSI-unaware truncation would corrupt escape sequences, so callers should
 * pass plain text; styling is applied by the panel afterwards.
 */
export function truncate(text: string, width: number, ellipsis = "…"): string {
  if (width <= 0) return "";
  const chars = [...text];
  if (chars.length <= width) return text;
  if (width <= ellipsis.length) return chars.slice(0, width).join("");
  return chars.slice(0, width - ellipsis.length).join("") + ellipsis;
}

/** Right-pad plain text to a fixed visible width. */
export function padEnd(text: string, width: number, fill = " "): string {
  const w = displayWidth(text);
  if (w >= width) return text;
  return text + fill.repeat(width - w);
}

/** Left-pad plain text to a fixed visible width. */
export function padStart(text: string, width: number, fill = " "): string {
  const w = displayWidth(text);
  if (w >= width) return text;
  return fill.repeat(width - w) + text;
}

// ─── Box drawing ─────────────────────────────────────────────────────────────

export interface BoxChars {
  topLeft: string;
  topRight: string;
  bottomLeft: string;
  bottomRight: string;
  horizontal: string;
  vertical: string;
}

export const ROUNDED_BOX: BoxChars = {
  topLeft: "╭",
  topRight: "╮",
  bottomLeft: "╰",
  bottomRight: "╯",
  horizontal: "─",
  vertical: "│",
};

export const SHARP_BOX: BoxChars = {
  topLeft: "┌",
  topRight: "┐",
  bottomLeft: "└",
  bottomRight: "┘",
  horizontal: "─",
  vertical: "│",
};

export interface DrawBoxOptions {
  title?: string | undefined;
  /** Inner content width (columns between the vertical borders). */
  width: number;
  chars?: BoxChars;
  /** Optional style codes applied to the border characters. */
  borderSgr?: number[];
}

/**
 * Draw a box around the given content lines. Content is truncated/padded to the
 * requested inner width. Returns the framed lines (borders included), each a
 * complete row of the panel. Lines do not include positioning; the host places
 * them with {@link moveTo}.
 */
export function drawBox(lines: string[], options: DrawBoxOptions): string[] {
  const chars = options.chars ?? ROUNDED_BOX;
  const width = Math.max(1, options.width);
  const border = (s: string): string =>
    options.borderSgr && options.borderSgr.length > 0 ? sgr(s, ...options.borderSgr) : s;

  const h = chars.horizontal;
  const out: string[] = [];

  // Top border with optional title.
  if (options.title) {
    const title = ` ${truncate(options.title, Math.max(0, width - 2))} `;
    const titleW = displayWidth(title);
    const remaining = Math.max(0, width - titleW);
    out.push(
      border(chars.topLeft) + border(title) + border(h.repeat(remaining)) + border(chars.topRight),
    );
  } else {
    out.push(border(chars.topLeft + h.repeat(width) + chars.topRight));
  }

  // Content rows.
  for (const line of lines) {
    const plain = truncate(line, width);
    const padded = padEnd(plain, width);
    out.push(border(chars.vertical) + padded + border(chars.vertical));
  }

  // Bottom border.
  out.push(border(chars.bottomLeft + h.repeat(width) + chars.bottomRight));

  return out;
}

// ─── Bar / sparkline helpers ─────────────────────────────────────────────────

const SPARK_CHARS = ["▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];

/** Render a numeric series as a unicode sparkline of the given width. */
export function sparkline(values: number[], width: number): string {
  if (width <= 0 || values.length === 0) return "";
  const sample = values.slice(-width);
  const max = Math.max(...sample, 0.0001);
  const min = Math.min(...sample, 0);
  const range = max - min || 1;
  return sample
    .map((v) => {
      const idx = Math.min(
        SPARK_CHARS.length - 1,
        Math.max(0, Math.round(((v - min) / range) * (SPARK_CHARS.length - 1))),
      );
      return SPARK_CHARS[idx];
    })
    .join("");
}

/** Render a 0..1 ratio as a horizontal bar of the given width. */
export function bar(ratio: number, width: number, filledChar = "█", emptyChar = "░"): string {
  if (width <= 0) return "";
  const clamped = Math.min(1, Math.max(0, ratio));
  const filled = Math.round(clamped * width);
  return filledChar.repeat(filled) + emptyChar.repeat(width - filled);
}
