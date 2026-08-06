/** Truncate text with an ellipsis, keeping at least the first three characters. */
export function truncate(text: string, maxLength: number): string {
  if (maxLength <= 3 || text.length <= maxLength) {
    return text;
  }

  return `${text.slice(0, maxLength - 3)}...`;
}

/** Truncate text to a width, using an ellipsis when space allows. */
export function truncateToWidth(text: string, width: number): string {
  if (width <= 0) {
    return "";
  }

  if (text.length <= width) {
    return text;
  }

  if (width <= 3) {
    return text.slice(0, width);
  }

  return `${text.slice(0, width - 3)}...`;
}

/** Pad a string with trailing spaces to at least the given width. */
export function pad(text: string, width: number): string {
  return text.length >= width ? text : text.padEnd(width, " ");
}

/** Split a long token into fixed-width segments. */
export function splitLongToken(token: string, width: number): string[] {
  const clampedWidth = Math.max(1, width);
  const segments: string[] = [];

  for (let offset = 0; offset < token.length; offset += clampedWidth) {
    segments.push(token.slice(offset, offset + clampedWidth));
  }

  return segments;
}

/** Word-wrap text to a width, breaking overlong tokens on character boundaries. */
export function wrapText(text: string, width: number): string[] {
  const clampedWidth = Math.max(1, width);
  const normalized = text.replace(/\r/g, "");
  const paragraphs = normalized.split("\n");
  const wrapped: string[] = [];

  for (const paragraph of paragraphs) {
    if (paragraph.length === 0) {
      wrapped.push("");
      continue;
    }

    const words = paragraph.split(/\s+/);
    let current = "";

    for (const word of words) {
      if (word.length === 0) {
        continue;
      }

      if (current.length === 0) {
        if (word.length <= clampedWidth) {
          current = word;
        } else {
          const segments = splitLongToken(word, clampedWidth);
          current = segments.pop() ?? "";
          wrapped.push(...segments);
        }
        continue;
      }

      const candidate = `${current} ${word}`;
      if (candidate.length <= clampedWidth) {
        current = candidate;
        continue;
      }

      wrapped.push(current);

      if (word.length <= clampedWidth) {
        current = word;
      } else {
        const segments = splitLongToken(word, clampedWidth);
        current = segments.pop() ?? "";
        wrapped.push(...segments);
      }
    }

    wrapped.push(current);
  }

  return wrapped.length > 0 ? wrapped : [""];
}

/** Normalize CRLF and CR line endings to LF. */
export function normalizeNewlines(value: string): string {
  return value.replace(/\r\n/g, "\n").replace(/\r/g, "\n");
}

/** Render a string as a JSON-escaped preview, truncated with an ellipsis. */
export function escapedPreview(value: string, maxLength = 64): string {
  const escaped = JSON.stringify(value);
  return escaped.length <= maxLength ? escaped : `${escaped.slice(0, maxLength - 3)}...`;
}

const textEncoder = new TextEncoder();

/** Count the UTF-8 byte length of a string. */
export function byteLength(value: string): number {
  return textEncoder.encode(value).length;
}

/** Render the leading bytes of a buffer as a lowercase hex string. */
export function hexPrefix(bytes: Uint8Array, count = 12): string {
  const slice = bytes.slice(0, count);
  const hex = Array.from(slice, (byte) => byte.toString(16).padStart(2, "0")).join("");
  return bytes.length > count ? `${hex}…` : hex;
}

/** Safely extract a non-empty trimmed string from an unknown value, or undefined. */
export function getMetadataText(value: unknown): string | undefined {
  if (typeof value !== "string") {
    return undefined;
  }

  const trimmed = value.trim();
  return trimmed || undefined;
}

/** Fit text into a fixed-width cell: pad if short, truncate with '.' if long. */
export function trimCell(value: string, width: number): string {
  if (value.length <= width) {
    return value.padEnd(width);
  }

  if (width <= 1) {
    return value.slice(0, width);
  }

  return `${value.slice(0, width - 1)}.`;
}
