#!/usr/bin/env node

/**
 * Dist Manifest Generator
 *
 * Emits a publish-ready package.json into a package's dist/ directory and
 * copies README/LICENSE alongside it, so publishing happens from dist/ with a
 * manifest that is independent of the dev-time package.json.
 *
 * - Entry points are rewritten relative to dist ("./dist/index.mjs" -> "./index.mjs").
 * - "@bettertui/*": "workspace:*" dependencies are pinned to exact versions.
 * - @bettertui/core additionally declares every native platform package as an
 *   optionalDependency at the same version (napi-rs distribution pattern).
 *
 * Usage:
 *   From inside a package directory (postbuild lifecycle): only that package
 *   is processed.
 *   node ../../scripts/generateDistManifest.ts
 *
 *   From the repository root: all supported packages are processed; packages
 *   without an existing dist/ are skipped with a warning.
 *   node scripts/generateDistManifest.ts
 */

import { copyFileSync, existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

interface PackageJson {
  name?: string;
  version?: string;
  description?: string;
  keywords?: string[];
  license?: string;
  author?: string;
  homepage?: string;
  repository?: { type: string; url: string; directory?: string };
  bugs?: { url: string };
  engines?: Record<string, string>;
  type?: string;
  main?: string;
  module?: string;
  types?: string;
  exports?: Record<string, Record<string, string>>;
  dependencies?: Record<string, string>;
  peerDependencies?: Record<string, string>;
}

interface PackageTarget {
  name: string;
  dirName: string;
  dir: string;
  distDir: string;
  native: boolean;
}

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const rootDir = resolve(__dirname, "..");

const NATIVE_TRIPLES = [
  "darwin-x64",
  "darwin-arm64",
  "linux-x64-gnu",
  "linux-arm64-gnu",
  "linux-x64-musl",
  "linux-arm64-musl",
  "win32-x64",
  "win32-arm64",
] as const;

function createTargets(): PackageTarget[] {
  return [
    { name: "@bettertui/core", dirName: "core", native: true },
    { name: "@bettertui/react", dirName: "react", native: false },
    { name: "@bettertui/shared", dirName: "shared", native: false },
  ].map((pkg) => {
    const dir = join(rootDir, "packages", pkg.dirName);
    return { ...pkg, dir, distDir: join(dir, "dist") };
  });
}

function readJson<T>(filePath: string): T {
  return JSON.parse(readFileSync(filePath, "utf8")) as T;
}

function rewriteEntry(entry: string): string {
  if (!entry) return entry;
  return entry.replace(/^\.\/dist\//, "./");
}

function resolveWorkspaceVersion(depName: string): string {
  const shortName = depName.startsWith("@") ? depName.split("/")[1] : depName;
  const depPkgPath = join(rootDir, "packages", shortName, "package.json");
  if (!existsSync(depPkgPath)) {
    throw new Error(`Cannot resolve workspace dependency ${depName}: ${depPkgPath} not found`);
  }
  const version = readJson<PackageJson>(depPkgPath).version;
  if (!version) {
    throw new Error(`Workspace dependency ${depName} has no version`);
  }
  return version;
}

function pinWorkspaceDeps(
  deps: Record<string, string> | undefined,
): Record<string, string> | undefined {
  if (!deps) return undefined;
  const pinned: Record<string, string> = {};
  for (const [name, spec] of Object.entries(deps)) {
    pinned[name] =
      spec === "workspace:*" || spec === "workspace:^" || spec === "workspace:~"
        ? resolveWorkspaceVersion(name)
        : spec;
  }
  return pinned;
}

function buildManifest(target: PackageTarget): PackageJson & Record<string, unknown> {
  const source = readJson<PackageJson>(join(target.dir, "package.json"));
  const version = source.version ?? "0.0.0";

  const manifest: PackageJson & Record<string, unknown> = {};

  const identityFields = [
    "name",
    "version",
    "description",
    "keywords",
    "license",
    "author",
    "homepage",
    "repository",
    "bugs",
    "engines",
    "type",
  ] as const;
  for (const field of identityFields) {
    if (source[field] !== undefined) {
      manifest[field] = source[field];
    }
  }

  for (const field of ["main", "module", "types"] as const) {
    if (source[field]) {
      manifest[field] = rewriteEntry(source[field] as string);
    }
  }

  if (source.exports) {
    manifest.exports = Object.fromEntries(
      Object.entries(source.exports).map(([key, conditions]) => [
        key,
        Object.fromEntries(
          Object.entries(conditions).map(([cond, val]) => [cond, rewriteEntry(val)]),
        ),
      ]),
    );
  }

  const dependencies = pinWorkspaceDeps(source.dependencies);
  if (dependencies) {
    manifest.dependencies = dependencies;
  }

  if (source.peerDependencies) {
    manifest.peerDependencies = source.peerDependencies;
  }

  if (target.native) {
    const nativeDeps: Record<string, string> = {};
    for (const triple of NATIVE_TRIPLES) {
      nativeDeps[`${target.name}-${triple}`] = version;
    }
    manifest.optionalDependencies = nativeDeps;
  }

  return manifest;
}

function generateFor(target: PackageTarget): void {
  console.log(`\nGenerating manifest for ${target.name}...`);

  if (!existsSync(target.distDir)) {
    throw new Error(
      `dist not found for ${target.name}: ${target.distDir}. Run 'pnpm build' first.`,
    );
  }

  const manifest = buildManifest(target);

  const distManifestPath = join(target.distDir, "package.json");
  writeFileSync(distManifestPath, `${JSON.stringify(manifest, null, 2)}\n`);
  console.log(`  Wrote ${distManifestPath}`);

  const readmePath = join(target.dir, "README.md");
  if (existsSync(readmePath)) {
    copyFileSync(readmePath, join(target.distDir, "README.md"));
    console.log("  Copied README.md");
  } else {
    console.warn(`  WARNING: README.md not found at ${readmePath}`);
  }

  const licensePath = join(rootDir, "LICENSE");
  if (existsSync(licensePath)) {
    copyFileSync(licensePath, join(target.distDir, "LICENSE"));
    console.log("  Copied LICENSE");
  } else {
    console.warn(`  WARNING: LICENSE not found at ${licensePath}`);
  }

  console.log(`  ✅ ${target.name}@${manifest.version} ready to publish from dist/`);
}

console.log("BetterTUI dist manifest generation");

const targets = createTargets();
const cwd = process.cwd();
const invokedFrom = targets.find(
  (target) => cwd === target.dir || cwd.startsWith(`${target.dir}${"/"}`),
);

if (invokedFrom) {
  generateFor(invokedFrom);
} else {
  let failures = 0;
  for (const target of targets) {
    try {
      generateFor(target);
    } catch (error) {
      failures++;
      console.warn(`  SKIPPED: ${(error as Error).message}`);
    }
  }
  if (failures === targets.length) {
    console.error("\nERROR: No package dists found. Run 'pnpm build' first.");
    process.exit(1);
  }
}

console.log("\n✅ Dist manifests generated\n");
