import type {
  NapiEngine,
  NapiEventBus,
  NapiFocusManager,
  NapiScheduler,
  NapiTextEngine,
  ProcessResult,
  SchedulerStats,
  TerminalCapabilities,
} from "./types.js";

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

export { createRuntime } from "./runtime.js";
export type { Runtime, RuntimeOptions } from "./runtime.js";

export { createEventLoop } from "./events.js";
export type { EventLoop, EventCallback } from "./events.js";

// Re-export shared event types for consumer convenience
export type { KeyEvent, MouseEvent } from "./events.js";

let nativeAddon: Record<string, unknown> | null = null;

function loadNativeAddon(): Record<string, unknown> {
  if (nativeAddon) return nativeAddon;

  try {
    nativeAddon = require("bettertui_bindings");
  } catch {
    throw new Error(
      "Failed to load native bindings. Run `cargo build -p bettertui-bindings` first.",
    );
  }
  return nativeAddon as Record<string, unknown>;
}

export function createEngine(width?: number, height?: number): NapiEngine {
  const addon = loadNativeAddon();
  const Engine = addon.NapiEngine as new (width?: number, height?: number) => NapiEngine;
  return new Engine(width, height);
}

export function createEventBus(): NapiEventBus {
  const addon = loadNativeAddon();
  const EventBus = addon.NapiEventBus as new () => NapiEventBus;
  return new EventBus();
}

export function createFocusManager(): NapiFocusManager {
  const addon = loadNativeAddon();
  const FocusManager = addon.NapiFocusManager as new () => NapiFocusManager;
  return new FocusManager();
}

export function createTextEngine(): NapiTextEngine {
  const addon = loadNativeAddon();
  const TextEngine = addon.NapiTextEngine as new () => NapiTextEngine;
  return new TextEngine();
}

export function createScheduler(): NapiScheduler {
  const addon = loadNativeAddon();
  const Scheduler = addon.NapiScheduler as new () => NapiScheduler;
  return new Scheduler();
}

export function detectCapabilities(): TerminalCapabilities {
  const addon = loadNativeAddon();
  const detect = addon.detectCapabilities as () => string;
  return JSON.parse(detect()) as TerminalCapabilities;
}

export function getVersion(): string {
  const addon = loadNativeAddon();
  const getVersionFn = addon.getVersion as () => string;
  return getVersionFn();
}
