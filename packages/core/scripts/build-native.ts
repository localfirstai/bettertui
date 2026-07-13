#!/usr/bin/env node

/**
 * Build script for cross-compiling BetterTUI native bindings.
 *
 * This script:
 * 1. Cross-compiles the Rust napi-rs bindings for all target platforms
 * 2. Creates platform-specific npm packages (e.g., @bettertui/core-darwin-arm64)
 * 3. Each package contains the compiled .node binary and exports its path
 *
 * Usage:
 *   node scripts/build-native.ts --all          # Build for all platforms
 *   node scripts/build-native.ts --target darwin-arm64  # Build for specific target
 */

import { type SpawnSyncReturns, spawnSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

interface Variant {
  platform: string;
  arch: string;
  abi?: string;
  rustTarget: string;
  binaryName: string;
  extension: string;
}

interface PackageJson {
  name: string;
  version: string;
  license?: string;
  repository?: { type: string; url: string; directory?: string };
  description?: string;
  homepage?: string;
  author?: string;
  bugs?: { url: string };
  keywords?: string[];
}

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const rootDir = resolve(__dirname, "..");
const cratesDir = join(rootDir, "crates");
const bindingsDir = join(cratesDir, "bindings");
const licensePath = resolve(__dirname, "../../../LICENSE");

const packageJson: PackageJson = JSON.parse(readFileSync(join(rootDir, "package.json"), "utf8"));

// All supported target variants
const variants: Variant[] = [
  {
    platform: "darwin",
    arch: "x64",
    rustTarget: "x86_64-apple-darwin",
    binaryName: "bettertui_bindings.darwin-x64.node",
    extension: ".node",
  },
  {
    platform: "darwin",
    arch: "arm64",
    rustTarget: "aarch64-apple-darwin",
    binaryName: "bettertui_bindings.darwin-arm64.node",
    extension: ".node",
  },
  {
    platform: "linux",
    arch: "x64",
    abi: "gnu",
    rustTarget: "x86_64-unknown-linux-gnu",
    binaryName: "bettertui_bindings.linux-x64-gnu.node",
    extension: ".node",
  },
  {
    platform: "linux",
    arch: "arm64",
    abi: "gnu",
    rustTarget: "aarch64-unknown-linux-gnu",
    binaryName: "bettertui_bindings.linux-arm64-gnu.node",
    extension: ".node",
  },
  {
    platform: "linux",
    arch: "x64",
    abi: "musl",
    rustTarget: "x86_64-unknown-linux-musl",
    binaryName: "bettertui_bindings.linux-x64-musl.node",
    extension: ".node",
  },
  {
    platform: "linux",
    arch: "arm64",
    abi: "musl",
    rustTarget: "aarch64-unknown-linux-musl",
    binaryName: "bettertui_bindings.linux-arm64-musl.node",
    extension: ".node",
  },
  {
    platform: "win32",
    arch: "x64",
    rustTarget: "x86_64-pc-windows-msvc",
    binaryName: "bettertui_bindings.win32-x64.node",
    extension: ".node",
  },
  {
    platform: "win32",
    arch: "arm64",
    rustTarget: "aarch64-pc-windows-msvc",
    binaryName: "bettertui_bindings.win32-arm64.node",
    extension: ".node",
  },
];

// Parse CLI arguments
const args = process.argv.slice(2);
const buildAll = args.includes("--all");
const targetArg = args.find((arg) => arg.startsWith("--target="))?.split("=")[1];
const isDev = args.includes("--dev");

if (!buildAll && !targetArg) {
  console.error("Error: Specify --all or --target=<platform-arch> (e.g., --target=darwin-arm64)");
  process.exit(1);
}

const getHostVariant = (): Variant => {
  const hostVariant = variants.find(
    (variant) => variant.platform === process.platform && variant.arch === process.arch,
  );
  if (!hostVariant) {
    console.error(`Error: Unsupported host platform: ${process.platform}-${process.arch}`);
    process.exit(1);
  }
  return hostVariant;
};

const getVariantByTarget = (target: string): Variant | undefined => {
  return variants.find((v) => `${v.platform}-${v.arch}` === target);
};

const runCommand = (
  command: string,
  commandArgs: string[],
  cwd: string,
  errorMessage: string,
): void => {
  console.log(`  Running: ${command} ${commandArgs.join(" ")}`);
  const result: SpawnSyncReturns<Buffer> = spawnSync(command, commandArgs, {
    cwd,
    stdio: "inherit",
    env: {
      ...process.env,
      RUSTFLAGS: process.env.RUSTFLAGS || "",
    },
  });

  if (result.error) {
    console.error(`${errorMessage}: ${result.error.message}`);
    process.exit(1);
  }

  if (result.status !== 0) {
    console.error(errorMessage);
    process.exit(1);
  }
};

const variantsToBuild = buildAll
  ? variants
  : targetArg
    ? (() => {
        const v = getVariantByTarget(targetArg);
        if (!v) {
          console.error(`Error: Unknown target: ${targetArg}`);
          console.error(
            `Available targets: ${variants.map((v) => `${v.platform}-${v.arch}`).join(", ")}`,
          );
          process.exit(1);
        }
        return [v];
      })()
    : [getHostVariant()];

console.log(`\nBuilding BetterTUI native bindings for ${variantsToBuild.length} target(s)...\n`);

// Step 1: Cross-compile for each target
for (const variant of variantsToBuild) {
  console.log(
    `\n--- Building for ${variant.platform}-${variant.arch}${variant.abi ? ` (${variant.abi})` : ""} ---`,
  );
  console.log(`  Rust target: ${variant.rustTarget}`);

  const profile = isDev ? "debug" : "release";
  const cargoArgs = [
    "build",
    "--manifest-path",
    join(bindingsDir, "Cargo.toml"),
    "--target",
    variant.rustTarget,
    "--lib",
  ];

  if (!isDev) {
    cargoArgs.push("--release");
  }

  runCommand("cargo", cargoArgs, rootDir, `Error: Cargo build failed for ${variant.rustTarget}`);

  // Find the compiled binary
  const targetDir = join(rootDir, "target", variant.rustTarget, profile);
  const possibleNames = [
    "bettertui_bindings.node",
    "libbettertui_bindings.node",
    "libbettertui_bindings.dylib",
    "libbettertui_bindings.so",
    "bettertui_bindings.dll",
  ];

  let sourceBinary: string | null = null;
  for (const name of possibleNames) {
    const path = join(targetDir, name);
    if (existsSync(path)) {
      sourceBinary = path;
      break;
    }
  }

  if (!sourceBinary) {
    console.error(`Error: Compiled binary not found in ${targetDir}`);
    console.error(`Expected one of: ${possibleNames.join(", ")}`);
    process.exit(1);
  }

  console.log(`  Found binary: ${sourceBinary}`);

  // Step 2: Create platform-specific npm package
  const nativePackageName = `${packageJson.name}-${variant.platform}-${variant.arch}${variant.abi ? `-${variant.abi}` : ""}`;
  const nativeDir = join(rootDir, "node_modules", nativePackageName);

  rmSync(nativeDir, { recursive: true, force: true });
  mkdirSync(nativeDir, { recursive: true });

  // Copy the compiled binary
  const destBinary = join(nativeDir, variant.binaryName);
  copyFileSync(sourceBinary, destBinary);
  console.log(`  Copied binary to: ${destBinary}`);

  // Create index.js - CommonJS exports the path to the native binary
  const indexJsContent = `"use strict";
const path = require("node:path");
module.exports = path.join(__dirname || ".", "${variant.binaryName}");
`;
  writeFileSync(join(nativeDir, "index.js"), indexJsContent);

  // Create index.d.ts - TypeScript declaration
  writeFileSync(join(nativeDir, "index.d.ts"), "export = path;\ndeclare const path: string;\n");

  // Create package.json for the native package (CommonJS for sync require support)
  writeFileSync(
    join(nativeDir, "package.json"),
    JSON.stringify(
      {
        name: nativePackageName,
        version: packageJson.version,
        description: `Prebuilt ${variant.platform}-${variant.arch}${variant.abi ? `-${variant.abi}` : ""} binary for ${packageJson.name}`,
        main: "index.js",
        types: "index.d.ts",
        license: packageJson.license,
        author: packageJson.author,
        homepage: packageJson.homepage,
        repository: packageJson.repository,
        bugs: packageJson.bugs,
        keywords: [...(packageJson.keywords ?? []), "prebuild", "prebuilt", "native"],
        exports: {
          ".": {
            require: "./index.js",
            types: "./index.d.ts",
          },
        },
        os: [
          variant.platform === "win32"
            ? "win32"
            : variant.platform === "darwin"
              ? "darwin"
              : "linux",
        ],
        cpu: [variant.arch],
        ...(variant.abi ? { libc: [variant.abi] } : {}),
      },
      null,
      2,
    ),
  );

  // Copy LICENSE
  if (existsSync(licensePath)) {
    copyFileSync(licensePath, join(nativeDir, "LICENSE"));
  }

  // Create README.md
  writeFileSync(
    join(nativeDir, "README.md"),
    `## ${nativePackageName}

> Prebuilt ${variant.platform}-${variant.arch}${variant.abi ? `-${variant.abi}` : ""} binary for \`${packageJson.name}\`.
`,
  );

  console.log(`  ✅ Built: ${nativePackageName}`);
}

console.log(`\n✅ Successfully built ${variantsToBuild.length} native package(s)\n`);
