// Standalone build for the example suite. Mirrors OpenTUI's scripts/build.ts,
// which compiles a runnable artifact; here we use tsdown (the package's existing
// bundler) to produce a single ESM executable at dist/index.mjs.
//
//   node scripts/build.ts

import { chmodSync, mkdirSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { build } from "tsdown";

const here = dirname(fileURLToPath(import.meta.url));
const packageRoot = resolve(here, "..");
const outfile = resolve(packageRoot, "dist", "index.mjs");

mkdirSync(dirname(outfile), { recursive: true });

console.log("Building examples standalone executable...");

await build({
  entry: [resolve(packageRoot, "src", "index.tsx")],
  format: ["esm"],
  outDir: resolve(packageRoot, "dist"),
  outExtension: () => ({ js: ".mjs" }),
  platform: "node",
  target: "node20",
  banner: { js: "#!/usr/bin/env node" },
  clean: true,
  shims: true,
});

try {
  chmodSync(outfile, 0o755);
} catch {
  // chmod is best-effort; on some filesystems it is a no-op.
}

console.log(`✅ Built standalone executable: ${outfile}`);
console.log("   Run with: node dist/index.mjs --list   (or a slug)");
