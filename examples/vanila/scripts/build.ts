#!/usr/bin/env node
/**
 * Build standalone executable for BetterTUI examples
 * Following OpenTUI's build pattern
 */

import { execSync } from "node:child_process";
import { chmodSync, existsSync, mkdirSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { build } from "tsdown";

const here = dirname(fileURLToPath(import.meta.url));
const packageRoot = resolve(here, "..");
const distDir = resolve(packageRoot, "dist");

console.log("Building BetterTUI examples...\n");

mkdirSync(distDir, { recursive: true });

const coreDist = resolve(packageRoot, "../../packages/core/dist");
if (!existsSync(resolve(coreDist, "bettertui_engine.node"))) {
  console.log("Building native engine first...");
  execSync("pnpm build:native", {
    cwd: resolve(packageRoot, "../../packages/core"),
    stdio: "inherit",
  });
}

console.log("Bundling TypeScript...");
await build({
  entry: [resolve(packageRoot, "src", "index.ts")],
  format: ["esm"],
  outDir: distDir,
  outExtensions: () => ({ js: ".mjs" }),
  platform: "node",
  target: "node22",
  clean: true,
  shims: true,
  deps: {
    neverBundle: ["@bettertui/core", "@bettertui/shared"],
  },
});

const outfile = resolve(distDir, "index.mjs");
try {
  chmodSync(outfile, 0o755);
} catch {
  // chmod is best-effort
}

console.log(`\n✅ Built standalone executable: ${outfile}`);
console.log("   Run with: node dist/index.mjs --list   (or a slug)");
console.log("   Or run directly: ./dist/index.mjs");
