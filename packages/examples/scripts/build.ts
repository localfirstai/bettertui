#!/usr/bin/env node

import { execSync } from "node:child_process";
import { existsSync, mkdirSync } from "node:fs";
import { resolve } from "node:path";
import process from "node:process";

const rootDir = resolve(import.meta.dirname, "../../..");
const examplesDir = resolve(import.meta.dirname, "..");
const distDir = resolve(examplesDir, "dist");

console.log("Building @bettertui/examples...");

// Step 1: Ensure @bettertui/core is built and available via workspace dependency
try {
  console.log("  [1/2] Building @bettertui/core...");
  execSync("pnpm --filter @bettertui/core build", {
    cwd: rootDir,
    stdio: "inherit",
    env: {
      ...process.env,
      PATH: `${process.env.HOME}/.cargo/bin:${process.env.PATH}`,
    },
  });
} catch (error) {
  console.error("Error: Failed to build @bettertui/core dependency.", error);
  process.exit(1);
}

// Step 2: Ensure output dist directory exists
if (!existsSync(distDir)) {
  mkdirSync(distDir, { recursive: true });
}

// Step 3: Bundle examples
try {
  console.log("  [2/2] Transpiling @bettertui/examples...");
  execSync("pnpm exec tsdown --entry src/index.ts --format esm --out-dir dist --clean", {
    cwd: examplesDir,
    stdio: "inherit",
  });
  console.log("✅ Successfully built @bettertui/examples\n");
} catch (error) {
  console.error("Error: Failed to bundle @bettertui/examples.", error);
  process.exit(1);
}
