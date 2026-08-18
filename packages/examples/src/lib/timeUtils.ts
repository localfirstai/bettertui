/** Format a Date as "HH:MM:SS". */
export function formatTimestamp(timestamp: Date): string {
  const hh = timestamp.getHours().toString().padStart(2, "0");
  const mm = timestamp.getMinutes().toString().padStart(2, "0");
  const ss = timestamp.getSeconds().toString().padStart(2, "0");
  return `${hh}:${mm}:${ss}`;
}

/** Format an ISO timestamp string as "HH:MM:SS.mmm". */
export function formatClock(timestamp: string): string {
  const date = new Date(timestamp);
  const hours = `${date.getHours()}`.padStart(2, "0");
  const minutes = `${date.getMinutes()}`.padStart(2, "0");
  const seconds = `${date.getSeconds()}`.padStart(2, "0");
  const millis = `${date.getMilliseconds()}`.padStart(3, "0");
  return `${hours}:${minutes}:${seconds}.${millis}`;
}

/** Format the current time as "HH:MM:SS.mmm". */
export function timestamp(): string {
  const now = new Date();
  const hh = `${now.getHours()}`.padStart(2, "0");
  const mm = `${now.getMinutes()}`.padStart(2, "0");
  const ss = `${now.getSeconds()}`.padStart(2, "0");
  const ms = `${now.getMilliseconds()}`.padStart(3, "0");
  return `${hh}:${mm}:${ss}.${ms}`;
}
