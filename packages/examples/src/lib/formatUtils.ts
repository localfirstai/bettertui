import { clamp01 } from "./mathUtils";

/** Serialize an unknown value as JSON, falling back to String on failure. */
export function safeJson(value: unknown, indent = 0): string {
  try {
    return JSON.stringify(value, null, indent) ?? String(value);
  } catch {
    return String(value);
  }
}

/** Render a scalar (string/number/boolean/null) as a compact, JSON-aware string. */
export function formatScalar(value: unknown): string {
  if (value === undefined) return "-";
  if (typeof value === "string") return JSON.stringify(value);
  if (typeof value === "number" || typeof value === "boolean" || value === null)
    return String(value);
  return safeJson(value);
}

/** Render a signed number with a + prefix and a small dead-zone around zero. */
export function formatSigned(value: number): string {
  const normalized = Math.abs(value) < 0.005 ? 0 : value;
  if (normalized >= 0) {
    return `+${normalized.toFixed(2)}`;
  }
  return `${normalized.toFixed(2)}`;
}

/** Render a normalized [0, 1] value as a filled/empty bar. */
export function meterBar(value: number, width = 28): string {
  const clamped = clamp01(value);
  const filled = Math.floor(clamped * width);
  return `[${"#".repeat(filled)}${"-".repeat(width - filled)}]`;
}

/** Render a normalized [0, 1] value as a rounded filled/empty bar. */
export function rangeBar(value: number, width = 6): string {
  const clamped = clamp01(value);
  const filled = Math.round(clamped * width);
  return `[${"#".repeat(filled)}${"-".repeat(width - filled)}]`;
}

/** Format a frequency in Hz as a compact string (k suffix above 1000). */
export function formatFrequency(value: number): string {
  return value >= 1000 ? `${value / 1000}k` : value.toString();
}

/** Sanitize arbitrary metadata text for display, replacing control characters with spaces. */
export function displayMetadata(value: string | undefined): string {
  // biome-ignore lint/suspicious/noControlCharactersInRegex: strip ASCII control characters
  const sanitized = value?.replace(/[\u0000-\u001f\u007f-\u009f]/g, " ").trim();
  return sanitized || "-";
}

/** Format a byte count as a human-readable size string (B, KiB, MiB). */
export function formatBytes(value: bigint): string {
  const bytes = Number(value);
  if (!Number.isFinite(bytes)) return `${value.toString()} B`;
  if (bytes < 1024) return `${bytes.toFixed(0)} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KiB`;
  return `${(bytes / (1024 * 1024)).toFixed(2)} MiB`;
}
