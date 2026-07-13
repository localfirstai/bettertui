#!/usr/bin/env node
// Run any example from the built dist by slug.
// Usage: node scripts/run-example.mjs hello-world
import { existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const examplesDir = resolve(here, "..", "dist", "examples");

const slug = process.argv[2];
if (!slug) {
  console.error("Usage: node scripts/run-example.mjs <slug>");
  process.exit(1);
}

// Search all category subdirectories for <slug>.mjs
import { readdirSync } from "node:fs";
const categories = readdirSync(examplesDir, { withFileTypes: true }).filter((d) => d.isDirectory());
for (const cat of categories) {
  const candidate = resolve(examplesDir, cat.name, `${slug}.mjs`);
  if (existsSync(candidate)) {
    await import(candidate);
    process.exit(0);
  }
}

console.error(`Unknown example: ${slug}`);
console.error("Run `node dist/index.mjs --list` to see all available examples.");
process.exit(1);
