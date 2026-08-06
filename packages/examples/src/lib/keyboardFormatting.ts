import { formatScalar } from "./formatUtils";
import { truncate } from "./textUtils";

/** A normalized snapshot of a parsed key event. */
export interface KeySnapshot {
  name: string;
  ctrl: boolean;
  meta: boolean;
  shift: boolean;
  option: boolean;
  sequence: string;
  raw: string;
  eventType: string;
  source: string;
  number: boolean;
  code?: string;
  super?: boolean;
  hyper?: boolean;
  capsLock?: boolean;
  numLock?: boolean;
  baseCode?: number;
  repeated?: boolean;
}

/** A normalized snapshot of a paste event. */
export interface PasteSnapshot {
  byteLength: number;
  bytes: number[];
  text: string;
  metadata?: unknown;
}

/** Format a key name for display, mapping character names to human labels. */
export function formatCharName(name: string): string {
  if (name === " ") return "Space";

  const codePoint = name.codePointAt(0);
  if (codePoint !== undefined && (codePoint < 32 || codePoint === 127)) {
    return `U+${codePoint.toString(16).toUpperCase().padStart(4, "0")}`;
  }

  switch (name) {
    case "escape":
      return "Escape";
    case "return":
      return "Return";
    case "linefeed":
      return "Linefeed";
    case "backspace":
      return "Backspace";
    case "space":
      return "Space";
    case "tab":
      return "Tab";
    default:
      return name;
  }
}

/** Join active key modifiers into a display string. */
export function formatModifiers(snapshot: KeySnapshot): string {
  const modifiers: string[] = [];
  if (snapshot.ctrl) modifiers.push("Ctrl");
  if (snapshot.meta) modifiers.push("Meta");
  if (snapshot.shift) modifiers.push("Shift");
  if (snapshot.option) modifiers.push("Option");
  if (snapshot.super) modifiers.push("Super");
  if (snapshot.hyper) modifiers.push("Hyper");
  return modifiers.length > 0 ? modifiers.join("+") : "-";
}

/** Format a full modifier + key combo for display. */
export function formatCombo(snapshot: KeySnapshot): string {
  const modifiers = formatModifiers(snapshot);
  const name = snapshot.name ? formatCharName(snapshot.name) : "-";
  if (modifiers === "-") return name;
  if (name === "-") return modifiers;
  return `${modifiers}+${name}`;
}

/** Format a base key code with its printable character when available. */
export function formatBaseCode(baseCode: number | undefined): string {
  if (baseCode === undefined) return "-";

  let rendered = `U+${baseCode.toString(16).toUpperCase().padStart(4, "0")}`;
  if (baseCode >= 32 && baseCode !== 127) {
    try {
      rendered = JSON.stringify(String.fromCodePoint(baseCode));
    } catch {
      rendered = `U+${baseCode.toString(16).toUpperCase().padStart(4, "0")}`;
    }
  }

  return `${baseCode} (${rendered})`;
}

/** Format a base key code concisely as a codepoint or printable character. */
export function formatBaseCodeBrief(baseCode: number | undefined): string {
  if (baseCode === undefined) return "-";

  if (baseCode >= 32 && baseCode !== 127) {
    try {
      return JSON.stringify(String.fromCodePoint(baseCode));
    } catch {
      // Fall back to the codepoint form below.
    }
  }

  return `U+${baseCode.toString(16).toUpperCase().padStart(4, "0")}`;
}

/** Truncate a scalar-formatted value to a max length. */
export function formatInline(text: string | undefined, maxLength: number): string {
  return truncate(formatScalar(text), maxLength);
}

/** Format a key sequence for display. */
export function formatKeySequence(sequence: unknown, _opts?: unknown): string {
  if (!sequence) return "";
  if (Array.isArray(sequence))
    return sequence
      .map((p: unknown) => String((p as Record<string, unknown>)?.match ?? p))
      .join("");
  return String(sequence);
}
