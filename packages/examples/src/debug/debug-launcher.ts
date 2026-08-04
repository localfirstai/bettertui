#!/usr/bin/env tsx
// Capture the first few frames from the actual example launcher.

const captured: string[] = [];
const origWrite = process.stdout.write.bind(process.stdout);
(process.stdout as { write: (s: string) => boolean }).write = (chunk: string | Uint8Array) => {
  const str = typeof chunk === "string" ? chunk : Buffer.from(chunk).toString("utf8");
  captured.push(str);
  return true;
};

// Now import and run the real example
await import("../index.js");

// Wait for a few frames
await new Promise((r) => setTimeout(r, 1000));

// Restore stdout
(process.stdout as { write: typeof origWrite }).write = origWrite;

const allOutput = captured.join("");

// Analyze SGR codes
// biome-ignore lint/suspicious/noControlCharactersInRegex: needed for ANSI sequence regex
const sgrRegex = /\x1B\[([0-9;]*)m/g;
let match: RegExpExecArray | null = sgrRegex.exec(allOutput);
const sgrCodes: string[] = [];
while (match !== null) {
  sgrCodes.push(match[1] || "0");
  match = sgrRegex.exec(allOutput);
}

// Check for white/near-white RGB backgrounds: 48;2;R;G;B where R,G,B > 200
const whiteBgCodes: string[] = [];
for (const c of sgrCodes) {
  const parts = c.split(";").map(Number);
  // bg=47 (white) or bg=107 (bright white)
  if (parts.includes(47) || parts.includes(107)) {
    whiteBgCodes.push(c);
    continue;
  }
  // RGB bg: 48;2;R;G;B
  const bgIdx = parts.indexOf(48);
  if (bgIdx >= 0 && parts[bgIdx + 1] === 2) {
    const r = parts[bgIdx + 2] || 0;
    const g = parts[bgIdx + 3] || 0;
    const b = parts[bgIdx + 4] || 0;
    if (r > 200 && g > 200 && b > 200) {
      whiteBgCodes.push(c);
    }
  }
}

console.log("=== TOTAL OUTPUT:", allOutput.length, "bytes ===");
console.log("=== TOTAL SGR sequences:", sgrCodes.length, "===");
console.log("=== WHITE BG SGR codes found:", whiteBgCodes.length, "===");
if (whiteBgCodes.length > 0) {
  const uniqueWhite = [...new Set(whiteBgCodes)];
  console.log("Unique white bg codes:", uniqueWhite);
}

// Show unique SGR codes
const unique = [...new Set(sgrCodes)].sort();
console.log("\nAll unique SGR codes:", unique);

// Show first 3000 chars of raw output
console.log("\n=== RAW OUTPUT (first 3000 chars, escaped) ===");
console.log(allOutput.replaceAll("\x1b", "\\x1b").slice(0, 3000));

process.exit(0);
