#!/usr/bin/env node

/**
 * Publish Script
 *
 * Publishes all BetterTUI packages to npm.
 * Must run pre-publish validation first.
 *
 * Usage:
 *   node scripts/publish.ts
 */

import { type SpawnSyncReturns, spawnSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

interface PackageJson {
  name: string;
  version: string;
  optionalDependencies?: Record<string, string>;
}

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const rootDir = resolve(__dirname, "..");

const packageJson: PackageJson = JSON.parse(
  readFileSync(join(rootDir, "packages", "core", "package.json"), "utf8"),
);

console.log(`Publishing @bettertui/core@${packageJson.version}...`);
console.log("Make sure you've run the pre-publish validation script first!");

const coreDir = join(rootDir, "packages", "core");
const distDir = join(coreDir, "dist");
const packageJsons: Record<string, PackageJson> = {
  [distDir]: JSON.parse(readFileSync(join(distDir, "package.json"), "utf8")),
};

// Load all native package.json files
const optionalDeps = packageJsons[distDir].optionalDependencies;
if (optionalDeps) {
  for (const pkgName of Object.keys(optionalDeps).filter((x) => x.startsWith(packageJson.name))) {
    const nativeDir = join(coreDir, "node_modules", pkgName);
    if (existsSync(nativeDir)) {
      packageJsons[nativeDir] = JSON.parse(readFileSync(join(nativeDir, "package.json"), "utf8"));
    } else {
      console.warn(`WARNING: Native package not found: ${nativeDir}`);
    }
  }
}

// Also publish @bettertui/shared and @bettertui/react
const sharedDir = join(rootDir, "packages", "shared", "dist");
const reactDir = join(rootDir, "packages", "react", "dist");

if (existsSync(join(sharedDir, "package.json"))) {
  packageJsons[sharedDir] = JSON.parse(readFileSync(join(sharedDir, "package.json"), "utf8"));
}

if (existsSync(join(reactDir, "package.json"))) {
  packageJsons[reactDir] = JSON.parse(readFileSync(join(reactDir, "package.json"), "utf8"));
}

// Publish all packages (main + native packages)
for (const [dir, { name, version }] of Object.entries(packageJsons)) {
  console.log(`\nPublishing ${name}@${version}...`);

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

  console.log(`Successfully published '${name}@${version}'`);
}

console.log("\nAll @bettertui packages published successfully!");
