import type {
  BindingInfo,
  NapiEngine,
  NapiEventBus,
  NapiFocusManager,
  NapiKeymap,
  NapiScheduler,
  NapiTextEngine,
  NapiTheme,
  NapiThemeBorders,
  NapiThemeColors,
  NapiThemeSpacing,
  NapiWidgetHost,
  ProcessResult,
  SchedulerStats,
  TerminalCapabilities,
} from "./types";

export type {
  BindingInfo,
  NapiEngine,
  NapiEventBus,
  NapiFocusManager,
  NapiKeymap,
  NapiTextEngine,
  NapiScheduler,
  NapiTheme,
  NapiThemeBorders,
  NapiThemeColors,
  NapiThemeSpacing,
  NapiWidgetHost,
  ProcessResult,
  TerminalCapabilities,
  SchedulerStats,
};

interface NativeBinding {
  NapiCapabilities: new () => object;
  NapiEngine: new (width?: number, height?: number) => NapiEngine;
  NapiEventBus: new () => object;
  NapiFocusManager: new () => object;
  NapiKeymap: new () => object;
  NapiScheduler: new (fps?: number) => object;
  NapiTextEngine: new (text?: string) => object;
  NapiWidgetHost: new () => object;
  createDefaultTheme: () => NapiTheme;
  createDarkTheme: () => NapiTheme;
  createLightTheme: () => NapiTheme;
  detectCapabilities: () => string;
  getVersion: () => string;
  highlightCode: (code: string, language: string) => string;
}

let nativeAddon: NativeBinding | null = null;
const loadErrors: Error[] = [];

const PLATFORM_PACKAGES: Record<string, string> = {
  "darwin-x64": "@bettertui/core-darwin-x64",
  "darwin-arm64": "@bettertui/core-darwin-arm64",
  "linux-x64-gnu": "@bettertui/core-linux-x64-gnu",
  "linux-arm64-gnu": "@bettertui/core-linux-arm64-gnu",
  "linux-x64-musl": "@bettertui/core-linux-x64-musl",
  "linux-arm64-musl": "@bettertui/core-linux-arm64-musl",
  "win32-x64": "@bettertui/core-win32-x64",
  "win32-arm64": "@bettertui/core-win32-arm64",
};

const isMusl = (): boolean => {
  if (process.platform !== "linux") return false;
  const libcEnv = process.env["BETTERTUI_LIBC"];
  if (libcEnv === "musl") return true;
  if (libcEnv === "gnu") return false;
  const fromFilesystem = isMuslFromFilesystem();
  if (fromFilesystem !== null) return fromFilesystem;
  const fromReport = isMuslFromReport();
  if (fromReport !== null) return fromReport;
  const fromChildProcess = isMuslFromChildProcess();
  if (fromChildProcess !== null) return fromChildProcess;
  return false;
};

const isFileMusl = (f: string): boolean => f.includes("libc.musl-") || f.includes("ld-musl-");

const isMuslFromFilesystem = (): boolean | null => {
  try {
    const { readFileSync } = require("node:fs");
    return readFileSync("/usr/bin/ldd", "utf-8").includes("musl");
  } catch {
    return null;
  }
};

const isMuslFromReport = (): boolean | null => {
  const report = getProcessReport();
  if (!report) return null;
  if (report.header?.glibcVersionRuntime) return false;
  if (Array.isArray(report.sharedObjects) && report.sharedObjects.some(isFileMusl)) return true;
  return false;
};

const isMuslFromChildProcess = (): boolean | null => {
  try {
    const { execSync } = require("node:child_process");
    return execSync("ldd --version", { encoding: "utf8" }).includes("musl");
  } catch {
    return false;
  }
};

interface ProcessReport {
  header?: { glibcVersionRuntime?: string };
  sharedObjects?: string[];
}

const getProcessReport = (): ProcessReport | null => {
  if (!process.report || typeof process.report.getReport !== "function") return null;
  try {
    (process.report as unknown as Record<string, boolean>)["excludeNetwork"] = true;
    return process.report.getReport() as ProcessReport;
  } catch {
    return null;
  }
};

function detectLibc(): string {
  const libcEnv = process.env["BETTERTUI_LIBC"];
  if (libcEnv === "musl" || libcEnv === "gnu") return libcEnv;
  try {
    const report = process.report?.getReport?.() as {
      header?: { glibcVersionRuntime?: string };
    } | null;
    if (report?.header?.glibcVersionRuntime) return "gnu";
    return "musl";
  } catch {
    return "gnu";
  }
}

function getPlatformKey(): string {
  const platform = process.platform;
  const arch = process.arch;
  if (platform === "linux") {
    const abi = detectLibc();
    return `${platform}-${arch}-${abi}`;
  }
  return `${platform}-${arch}`;
}

function findDevArtifact(): string | null {
  const path = require("node:path");
  const fs = require("node:fs") as typeof import("node:fs");
  const profiles = ["debug", "release"];
  const roots: string[] = [];
  const cwd = process.cwd();
  roots.push(
    cwd,
    path.join(cwd, "packages/core"),
    path.join(cwd, "../packages/core"),
    path.join(cwd, "../core"),
    path.join(cwd, "../../packages/core"),
    path.join(cwd, "../../packages"),
    path.join(cwd, ".."),
    path.join(cwd, "../.."),
  );
  for (const root of roots) {
    for (const profile of profiles) {
      const file = path.resolve(root, "target", profile, "bettertui_bindings.node");
      if (fs.existsSync(file)) return file;
    }
  }
  return null;
}

function resolveNativePath(): string | null {
  const platformKey = getPlatformKey();
  const packageName = PLATFORM_PACKAGES[platformKey];
  if (!packageName) return null;
  try {
    const path = require("node:path");
    const possibleLocations: string[] = [];
    try {
      possibleLocations.push(require.resolve(path.join(packageName, "index.js")));
    } catch {}
    try {
      possibleLocations.push(require.resolve(packageName));
    } catch {}
    for (const indexPath of possibleLocations) {
      const dir = path.dirname(indexPath);
      const nodeFiles = require("node:fs")
        .readdirSync(dir)
        .filter((f: string) => f.endsWith(".node"));
      if (nodeFiles.length > 0) return path.join(dir, nodeFiles[0]);
    }
  } catch {}
  return null;
}

function tryLoad(candidates: string[]): NativeBinding | null {
  for (const candidate of candidates) {
    try {
      const mod = require(candidate);
      if (mod && typeof mod === "object") {
        if (typeof mod === "string") {
          const actualMod = require(mod);
          return actualMod as NativeBinding;
        }
        return mod as NativeBinding;
      }
    } catch (e) {
      loadErrors.push(e as Error);
    }
  }
  return null;
}

function resolveNativeBinding(): NativeBinding | null {
  const platform = process.platform;
  const arch = process.arch;

  if (platform === "darwin") {
    if (arch === "x64") {
      return tryLoad([
        "@bettertui/core-darwin-x64",
        "./index.darwin-x64.node",
        "./bettertui_bindings.darwin-x64.node",
      ]);
    }
    if (arch === "arm64") {
      return tryLoad([
        "@bettertui/core-darwin-arm64",
        "./index.darwin-arm64.node",
        "./bettertui_bindings.darwin-arm64.node",
      ]);
    }
    loadErrors.push(new Error(`Unsupported architecture on macOS: ${arch}`));
    return null;
  }

  if (platform === "linux") {
    const musl = isMusl();
    const abi = musl ? "musl" : "gnu";
    if (arch === "x64") {
      return tryLoad([
        `@bettertui/core-linux-x64-${abi}`,
        `./index.linux-x64-${abi}.node`,
        `./bettertui_bindings.linux-x64-${abi}.node`,
      ]);
    }
    if (arch === "arm64") {
      return tryLoad([
        `@bettertui/core-linux-arm64-${abi}`,
        `./index.linux-arm64-${abi}.node`,
        `./bettertui_bindings.linux-arm64-${abi}.node`,
      ]);
    }
    loadErrors.push(new Error(`Unsupported architecture on Linux: ${arch}`));
    return null;
  }

  if (platform === "win32") {
    if (arch === "x64") {
      return tryLoad([
        "@bettertui/core-win32-x64",
        "./index.win32-x64.node",
        "./bettertui_bindings.win32-x64.node",
      ]);
    }
    if (arch === "arm64") {
      return tryLoad([
        "@bettertui/core-win32-arm64",
        "./index.win32-arm64.node",
        "./bettertui_bindings.win32-arm64.node",
      ]);
    }
    loadErrors.push(new Error(`Unsupported architecture on Windows: ${arch}`));
    return null;
  }

  loadErrors.push(new Error(`Unsupported platform: ${platform}-${arch}`));
  return null;
}

function loadNativeAddon(): NativeBinding {
  if (nativeAddon) return nativeAddon;

  const candidates: Array<() => NativeBinding | null> = [
    () => resolveNativeBinding(),
    () => {
      const resolvedPath = resolveNativePath();
      if (resolvedPath) {
        const mod = require(resolvedPath) as Record<string, unknown>;
        return mod as unknown as NativeBinding;
      }
      return null;
    },
    () => {
      const devPath = findDevArtifact();
      if (devPath) {
        const mod = require(devPath) as Record<string, unknown>;
        return mod as unknown as NativeBinding;
      }
      return null;
    },
    () => {
      try {
        return require("bettertui_bindings") as NativeBinding;
      } catch {
        return null;
      }
    },
  ];

  for (const attempt of candidates) {
    try {
      const result = attempt();
      if (result) {
        nativeAddon = result;
        return nativeAddon;
      }
    } catch {}
  }

  if (loadErrors.length > 0) {
    const error = new Error(
      "Cannot find native binding: " +
        "npm may have a bug with optional dependencies (https://github.com/npm/cli/issues/4828). " +
        "Please try `npm i` again after removing both package-lock.json and node_modules directory.",
    );
    error.cause = loadErrors.reduce((err, cur) => {
      cur.cause = err;
      return cur;
    });
    throw error;
  }

  const platformKey = getPlatformKey();
  const supported = Object.keys(PLATFORM_PACKAGES).join(", ");
  throw new Error(
    `Failed to load native bindings for ${platformKey}. Make sure you have the correct platform package installed (${PLATFORM_PACKAGES[platformKey] ?? "unknown"}), or run \`pnpm build:native\` for local development. Supported platforms: ${supported}`,
  );
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

export function createDefaultTheme(): NapiTheme {
  const addon = loadNativeAddon();
  const fn = addon.createDefaultTheme as () => NapiTheme;
  return fn();
}

export function createDarkTheme(): NapiTheme {
  const addon = loadNativeAddon();
  const fn = addon.createDarkTheme as () => NapiTheme;
  return fn();
}

export function createLightTheme(): NapiTheme {
  const addon = loadNativeAddon();
  const fn = addon.createLightTheme as () => NapiTheme;
  return fn();
}

export function createWidgetHost(): NapiWidgetHost {
  const addon = loadNativeAddon();
  const WidgetHost = addon.NapiWidgetHost as new () => NapiWidgetHost;
  return new WidgetHost();
}

export function createKeymap(): NapiKeymap {
  const addon = loadNativeAddon();
  const Keymap = addon.NapiKeymap as new () => NapiKeymap;
  return new Keymap();
}

export interface HighlightSegment {
  text: string;
  fg: string | null;
  bg: string | null;
  bold: boolean | null;
  italic: boolean | null;
  underline: boolean | null;
  dim: boolean | null;
  strikethrough: boolean | null;
}

export function highlightCode(code: string, language: string): HighlightSegment[][] {
  const addon = loadNativeAddon();
  const fn = addon.highlightCode as (code: string, language: string) => string;
  const raw = fn(code, language);
  try {
    return JSON.parse(raw) as HighlightSegment[][];
  } catch {
    return [];
  }
}

export function getNativePackageName(): string {
  const platformKey = getPlatformKey();
  const packageName = PLATFORM_PACKAGES[platformKey];
  if (!packageName) {
    throw new Error(`Unsupported platform: ${platformKey}`);
  }
  return packageName;
}
