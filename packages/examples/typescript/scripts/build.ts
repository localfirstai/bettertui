#!/usr/bin/env node

import { spawnSync, type SpawnSyncReturns } from "node:child_process"
import { chmodSync, cpSync, existsSync, mkdirSync, rmSync } from "node:fs"
import { createRequire } from "node:module"
import { dirname, join, resolve } from "node:path"
import { pathToFileURL } from "node:url"

const require = createRequire(import.meta.url)
const { build: esbuild } = require("esbuild")

const __dirname = dirname(new URL(import.meta.url).pathname)
const packageRoot = resolve(__dirname, "..")
const repoRoot = resolve(packageRoot, "../..")
const coreRoot = join(repoRoot, "packages", "core")
const coreDistDir = join(coreRoot, "dist")
const bundleDir = join(packageRoot, ".node")
const bundleEntry = join(bundleDir, "index.js")
const workerEntry = join(bundleDir, "parser.worker.js")

function requireNodeGte26(): string {
  const v = process.version
  const match = v.match(/^v(\d+)\./)
  if (!match) throw new Error(`Cannot parse Node version: ${v}`)
  const major = Number.parseInt(match[1], 10)
  if (major < 26) {
    throw new Error(`Node >= 26 required, found ${v}. Install from https://nodejs.org`)
  }
  return process.execPath
}

const nodePath = requireNodeGte26()

const args = process.argv.slice(2)
const skipBuild = args.includes("--skip-build")
const runOnly = args.includes("--run")

function run(command: string, commandArgs: string[], cwd: string): void {
  const result: SpawnSyncReturns<Buffer> = spawnSync(command, commandArgs, {
    cwd,
    stdio: "inherit",
  })

  if (result.error) {
    throw result.error
  }

  if (result.status !== 0) {
    process.exit(result.status ?? 1)
  }
}

function prepareCorePackage(): void {
  console.log("Building @bettertui/core (tsdown + native)...")

  run("pnpm", ["run", "build"], coreRoot)

  const nativePackageName = `core-${process.platform === "win32" ? "win32" : process.platform}-${process.arch}`
  const sourceNativeDir = join(coreRoot, "node_modules", "@bettertui", nativePackageName)
  const targetNativeDir = join(packageRoot, "node_modules", "@bettertui", nativePackageName)

  if (!existsSync(sourceNativeDir)) {
    console.warn(`Warning: Native package not found at ${sourceNativeDir}, skipping copy`)
    return
  }

  mkdirSync(join(packageRoot, "node_modules", "@bettertui"), { recursive: true })
  rmSync(targetNativeDir, { recursive: true, force: true })
  cpSync(sourceNativeDir, targetNativeDir, { recursive: true, dereference: true })

  console.log("Core package ready")
}

async function bundleExamples(): Promise<void> {
  console.log("Bundling examples with esbuild...")

  rmSync(bundleDir, { recursive: true, force: true })
  mkdirSync(bundleDir, { recursive: true })

  const result = await esbuild({
    entryPoints: [
      join(packageRoot, "src", "index.ts"),
      join(coreRoot, "src", "lib", "tree-sitter", "parser.worker.ts"),
    ],
    bundle: true,
    format: "esm",
    platform: "node",
    target: "esnext",
    outdir: bundleDir,
    outbase: packageRoot,
    sourcemap: true,
    splitting: true,
    alias: {
      "@bettertui/core": join(coreRoot, "src", "index.ts"),
      "@bettertui/three": join(repoRoot, "packages", "three", "src", "index.ts"),
      "@bettertui/keymap": join(repoRoot, "packages", "keymap", "src", "index.ts"),
      "@bettertui/keymap/addons/opentui": join(repoRoot, "packages", "keymap", "src", "addons", "opentui", "index.ts"),
      "@bettertui/keymap/opentui": join(repoRoot, "packages", "keymap", "src", "opentui.ts"),
      "@bettertui/qrcode": join(repoRoot, "packages", "qrcode", "src", "index.ts"),
    },
    external: [
      "bettertui_engine",
      "@bettertui/core-darwin-*",
      "@bettertui/core-linux-*",
      "@bettertui/core-win32-*",
      "@bettertui/core-darwin-*",
      "@bettertui/core-linux-*",
      "@bettertui/core-win32-*",
    ],
    logLevel: "info",
  })

  if (result.errors.length > 0) {
    console.error("Bundle errors:")
    for (const err of result.errors) {
      console.error(`  ${err.text}`)
    }
    process.exit(1)
  }

  console.log("Bundling complete")
}

function copyCoreDistPackage(): void {
  const targetCoreDir = join(bundleDir, "node_modules", "@bettertui", "core")

  mkdirSync(join(targetCoreDir, ".."), { recursive: true })
  cpSync(coreDistDir, targetCoreDir, { recursive: true })
}

if (!skipBuild && !runOnly) {
  prepareCorePackage()
  await bundleExamples()
  copyCoreDistPackage()
}

console.log(`Running with Node ${process.version}...`)

const result = spawnSync(nodePath, ["--experimental-ffi", "--no-warnings", bundleEntry], {
  cwd: packageRoot,
  stdio: "inherit",
  env: {
    ...process.env,
    OTUI_TREE_SITTER_WORKER_PATH: pathToFileURL(workerEntry).href,
  },
})

if (result.error) {
  throw result.error
}

process.exit(result.status ?? 0)
