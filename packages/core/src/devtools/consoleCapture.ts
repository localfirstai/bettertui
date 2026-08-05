import { Console } from "node:console";
import { EventEmitter } from "node:events";
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { env, registerEnvVar } from "../lib/env";
import { Capture, CapturedWritableStream } from "../lib/outputCapture";
import { singleton } from "../lib/singleton";

export enum ConsoleLogLevel {
  LOG = "LOG",
  INFO = "INFO",
  WARN = "WARN",
  ERROR = "ERROR",
  DEBUG = "DEBUG",
}

export interface CallerInfo {
  functionName: string;
  fullPath: string;
  fileName: string;
  lineNumber: number;
  columnNumber: number;
}

function getCallerInfo(): CallerInfo | null {
  const err = new Error();
  const stackLines = err.stack?.split("\n").slice(5) || [];
  if (!stackLines.length) return null;

  const callerLine = stackLines[0]?.trim();
  if (!callerLine) return null;

  const regex = /at\s+(?:([\w$.<>]+)\s+\()?((?:\/|[A-Za-z]:\\)[^:]+):(\d+):(\d+)\)?/;
  const match = callerLine.match(regex);
  if (!match) return null;

  return {
    functionName: match[1] || "<anonymous>",
    fullPath: match[2] || "",
    fileName: (match[2] || "").split(/[\\/]/).pop() || "<unknown>",
    lineNumber: Number.parseInt(match[3] || "0", 10),
    columnNumber: Number.parseInt(match[4] || "0", 10),
  };
}

export type ConsoleLogEntry = [Date, ConsoleLogLevel, unknown[], CallerInfo | null];

export const capture = singleton("ConsoleCapture", () => new Capture());

registerEnvVar({
  name: "BTUI_USE_CONSOLE",
  description: "Enable global console.* capture for the built-in terminal console overlay.",
  type: "boolean",
  default: true,
});

registerEnvVar({
  name: "SHOW_CONSOLE",
  description: "Open the built-in terminal console overlay at startup.",
  type: "boolean",
  default: false,
});

export class TerminalConsoleCache extends EventEmitter {
  private _cachedLogs: ConsoleLogEntry[] = [];
  private readonly MAX_CACHE_SIZE = 1000;
  private _collectCallerInfo = false;
  private _cachingEnabled = true;
  private _originalConsole: typeof console | null = null;
  private _active = false;

  get cachedLogs(): ConsoleLogEntry[] {
    return this._cachedLogs;
  }

  public activate(): void {
    if (this._active) return;
    if (!this._originalConsole) {
      this._originalConsole = globalThis.console;
    }
    this.setupConsoleCapture();
    this.overrideConsoleMethods();
    this._active = true;
  }

  private setupConsoleCapture(): void {
    if (!env.BTUI_USE_CONSOLE) return;

    const mockStdout = new CapturedWritableStream("stdout", capture);
    const mockStderr = new CapturedWritableStream("stderr", capture);

    globalThis.console = new Console({
      stdout: mockStdout,
      stderr: mockStderr,
      colorMode: true,
      inspectOptions: {
        compact: false,
        breakLength: 80,
        depth: 2,
      },
    }) as unknown as Console;
  }

  private overrideConsoleMethods(): void {
    console.log = (...args: unknown[]) => {
      this.appendToConsole(ConsoleLogLevel.LOG, ...args);
    };

    console.info = (...args: unknown[]) => {
      this.appendToConsole(ConsoleLogLevel.INFO, ...args);
    };

    console.warn = (...args: unknown[]) => {
      this.appendToConsole(ConsoleLogLevel.WARN, ...args);
    };

    console.error = (...args: unknown[]) => {
      this.appendToConsole(ConsoleLogLevel.ERROR, ...args);
    };

    console.debug = (...args: unknown[]) => {
      this.appendToConsole(ConsoleLogLevel.DEBUG, ...args);
    };

    // Polyfill React / devtools timeStamp if missing
    if (typeof console.timeStamp !== "function") {
      (console as unknown as Record<string, unknown>).timeStamp = () => {};
    }
  }

  public setCollectCallerInfo(enabled: boolean): void {
    this._collectCallerInfo = enabled;
  }

  public clearConsole(): void {
    this._cachedLogs = [];
  }

  public setCachingEnabled(enabled: boolean): void {
    this._cachingEnabled = enabled;
  }

  public deactivate(): void {
    if (!this._active) return;
    this.restoreOriginalConsole();
    this._active = false;
  }

  private restoreOriginalConsole(): void {
    if (this._originalConsole) {
      globalThis.console = this._originalConsole;
    }
  }

  public addLogEntry(level: ConsoleLogLevel, ...args: unknown[]): ConsoleLogEntry {
    const callerInfo = this._collectCallerInfo ? getCallerInfo() : null;
    const logEntry: ConsoleLogEntry = [new Date(), level, args, callerInfo];

    if (this._cachingEnabled) {
      if (this._cachedLogs.length >= this.MAX_CACHE_SIZE) {
        this._cachedLogs.shift();
      }
      this._cachedLogs.push(logEntry);
    }

    return logEntry;
  }

  private appendToConsole(level: ConsoleLogLevel, ...args: unknown[]): void {
    const entry = this.addLogEntry(level, ...args);
    this.emit("entry", entry);
  }

  public destroy(): void {
    this.deactivate();
  }
}

export const terminalConsoleCache = singleton("TerminalConsoleCache", () => {
  const instance = new TerminalConsoleCache();
  if (typeof process !== "undefined") {
    process.on("exit", () => {
      if (env.BTUI_DUMP_CAPTURES) {
        try {
          const timestamp = Date.now();
          const filepath = join(process.cwd(), `_btui_dump_${timestamp}.log`);
          const logs = instance.cachedLogs
            .map(
              ([date, level, args]) =>
                `[${date.toISOString()}] [${level}] ${args.map(String).join(" ")}`,
            )
            .join("\n");
          writeFileSync(filepath, logs, "utf8");
        } catch {
          // ignore write error during process exit
        }
      }
      instance.destroy();
    });
  }
  return instance;
});
