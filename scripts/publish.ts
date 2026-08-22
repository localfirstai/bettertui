#!/usr/bin/env node

/**
 * Publish Script
 *
 * Publishes BetterTUI packages to npm in dependency order:
 *   1. @bettertui/shared (pure types, no deps)
 *   2. Native platform packages (@bettertui/core-<triple>)
 *   3. @bettertui/core (depends on shared + native)
 *
 * Skips any package with "private": true.
 *
 * Usage:
 *   node scripts/publish.ts
 *   pnpm release
 */

import { type SpawnSyncReturns, spawnSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

interface PackageJson {
  name: string;
  version: string;
  private?: boolean;
  optionalDependencies?: Record<string, string>;
}

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const rootDir = resolve(__dirname, "..");

/** Ordered list of [directory, packageJson] to publish. */
const publishQueue: Array<[string, PackageJson]> = [];

function loadPkg(dir: string): PackageJson | null {
  const p = join(dir, "package.json");
  if (!existsSync(p)) return null;
  return JSON.parse(readFileSync(p, "utf8"));
}

// 1. Shared (must go first — core depends on it)
const sharedDir = join(rootDir, "packages", "shared", "dist");
const sharedPkg = loadPkg(sharedDir);
if (sharedPkg && !sharedPkg.private) {
  publishQueue.push([sharedDir, sharedPkg]);
}

// 2. Native platform packages
const coreDir = join(rootDir, "packages", "core");
const distDir = join(coreDir, "dist");
const corePkg = loadPkg(distDir);

if (corePkg?.optionalDependencies) {
  for (const pkgName of Object.keys(corePkg.optionalDependencies).filter((x) =>
    x.startsWith(corePkg.name),
  )) {
    const nativeDir = join(coreDir, "node_modules", pkgName);
    const nativePkg = loadPkg(nativeDir);
    if (nativePkg && !nativePkg.private) {
      publishQueue.push([nativeDir, nativePkg]);
    } else {
      console.warn(`WARNING: Native package not found: ${nativeDir}`);
    }
  }
}

// 3. Core (depends on shared + native)
if (corePkg && !corePkg.private) {
  publishQueue.push([distDir, corePkg]);
}

if (publishQueue.length === 0) {
  console.error("No packages to publish. Did you run `pnpm build` first?");
  process.exit(1);
}

console.log(`Publishing ${publishQueue.length} package(s)...\n`);

for (const [dir, { name, version }] of publishQueue) {
  console.log(`Publishing ${name}@${version}...`);

  const isSnapshot = version.includes("-snapshot") || /^0\.0\.0-\d{8}-[a-f0-9]{8}$/.test(version);
  const publishArgs = ["publish", "--access=public"];

  if (isSnapshot) {
    publishArgs.push("--tag", "snapshot");
    console.log("  Publishing as snapshot (--tag snapshot)");
  }

  const publish: SpawnSyncReturns<Buffer> = spawnSync("npm", publishArgs, {
    cwd: dir,
    stdio: "inherit",
  });

  if (publish.status !== 0) {
    console.error(`Failed to publish '${name}@${version}'.`);
    process.exit(1);
  }

  console.log(`  ✅ ${name}@${version} published\n`);
}

console.log("All @bettertui packages published successfully!");
