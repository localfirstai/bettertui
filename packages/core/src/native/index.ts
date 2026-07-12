import type {
  NapiEngine,
  NapiEventBus,
  NapiFocusManager,
  NapiScheduler,
  NapiTextEngine,
  ProcessResult,
  SchedulerStats,
  TerminalCapabilities,
} from "./types";

export type {
  NapiEngine,
  NapiEventBus,
  NapiFocusManager,
  NapiTextEngine,
  NapiScheduler,
  ProcessResult,
  TerminalCapabilities,
  SchedulerStats,
};

export { createRuntime } from "./runtime";
export type { Runtime, RuntimeOptions } from "./runtime";

export { createEventLoop } from "./events";
export type { EventLoop, EventCallback } from "./events";

// Re-export shared event types for consumer convenience
export type { KeyEvent, MouseEvent } from "./events";

let nativeAddon: Record<string, unknown> | null = null;

function findDevArtifact(): unknown {
  const path = require("node:path");
  const fs = require("node:fs") as typeof import("node:fs");
  const profiles = ["debug", "release"];
  const roots: string[] = [];
  const cwd = process.cwd();
  roots.push(
    cwd,
    path.join(cwd, "packages/core"),
    path.join(cwd, "../core"),
    path.join(cwd, "../../packages/core"),
    path.join(cwd, ".."),
    path.join(cwd, "../.."),
  );
  for (const root of roots) {
    for (const profile of profiles) {
      const file = path.resolve(root, "target", profile, "bettertui_bindings.node");
      if (fs.existsSync(file)) {
        try {
          return require(file);
        } catch {
          // require failed (e.g. wrong arch); keep searching
        }
      }
    }
  }
  throw new Error("dev artifact not found");
}

function loadNativeAddon(): Record<string, unknown> {
  if (nativeAddon) return nativeAddon;

  const candidates: Array<() => unknown> = [() => require("bettertui_bindings"), findDevArtifact];

  for (const attempt of candidates) {
    try {
      nativeAddon = attempt() as Record<string, unknown>;
      return nativeAddon;
    } catch {
      // try next candidate
    }
  }
  throw new Error(
    "Failed to load native bindings. Run `cargo build -p bettertui-bindings --manifest-path packages/core/Cargo.toml` first, or install the `bettertui_bindings` package.",
  );
}

export function createEngine(width?: number, height?: number): NapiEngine {
  const addon = loadNativeAddon();
  const Engine = addon["NapiEngine"] as new (width?: number, height?: number) => NapiEngine;
  return new Engine(width, height);
}

export function createEventBus(): NapiEventBus {
  const addon = loadNativeAddon();
  const EventBus = addon["NapiEventBus"] as new () => NapiEventBus;
  return new EventBus();
}

export function createFocusManager(): NapiFocusManager {
  const addon = loadNativeAddon();
  const FocusManager = addon["NapiFocusManager"] as new () => NapiFocusManager;
  return new FocusManager();
}

export function createTextEngine(): NapiTextEngine {
  const addon = loadNativeAddon();
  const TextEngine = addon["NapiTextEngine"] as new () => NapiTextEngine;
  return new TextEngine();
}

export function createScheduler(): NapiScheduler {
  const addon = loadNativeAddon();
  const Scheduler = addon["NapiScheduler"] as new () => NapiScheduler;
  return new Scheduler();
}

export function detectCapabilities(): TerminalCapabilities {
  const addon = loadNativeAddon();
  const detect = addon["detectCapabilities"] as () => string;
  return JSON.parse(detect()) as TerminalCapabilities;
}

export function getVersion(): string {
  const addon = loadNativeAddon();
  const getVersionFn = addon["getVersion"] as () => string;
  return getVersionFn();
}
