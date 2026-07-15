#!/usr/bin/env tsx
/**
 * BetterTUI Examples
 * Interactive example browser powered by the native Rust engine
 */

import { createCliRenderer, detectCapabilities, getVersion } from "@bettertui/core";
import { findExample } from "./examples";
import { ExampleSelector } from "./selector";

async function main(): Promise<void> {
  const arg = process.argv[2];

  if (arg === "--help" || arg === "-h") {
    printHelp();
    process.exit(0);
  }

  const caps = detectCapabilities();

  if (caps.columns === 0) {
    console.error("BetterTUI engine not properly loaded.");
    console.error("Ensure the native module is built:");
    console.error("  cd packages/core && pnpm build:native");
    process.exit(1);
  }

  const renderer = await createCliRenderer({
    exitOnCtrlC: false,
    targetFps: 60,
  });

  if (arg) {
    const example = findExample(arg);
    if (!example) {
      console.error(`Unknown example: ${arg}`);
      console.error("Run with --help to see available examples.");
      process.exit(1);
    }

    console.log(`Running: ${example.name}\n`);

    try {
      await example.run?.(renderer);
      example.destroy?.(renderer);
    } catch (error) {
      console.error("Example failed:", error);
      process.exit(1);
    }

    process.exit(0);
  }

  const selector = new ExampleSelector(renderer);
  await selector.run();
}

function printHelp(): void {
  console.log(`\n  \x1b[1;34mBetterTUI Examples v${getVersion()}\x1b[0m\n`);
  console.log("  Interactive example browser for the native Rust engine.\n");
  console.log("  Usage: pnpm dev [example-slug]\n");
  console.log("  Options:");
  console.log("    --help, -h    Show this help\n");
  console.log("  Examples:\n");
  console.log("    pnpm dev              Launch interactive browser");
  console.log("    pnpm dev hello-world  Run specific example");
  console.log("    pnpm dev keyboard     Run keyboard demo\n");
  console.log("  Keyboard Controls:\n");
  console.log("    Tab           Switch between filter and list");
  console.log("    ↑/↓ or j/k    Navigate");
  console.log("    Enter         Run selected example");
  console.log("    /             Focus filter");
  console.log("    t             Toggle theme");
  console.log("    Ctrl+C        Quit\n");
}

main();
