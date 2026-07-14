import { CommandBuffer, CommandRuntime } from "@bettertui/core";
import { createEngine } from "@bettertui/core";
import { createContext, useCallback, useContext, useRef } from "react";
import type { ReactNode } from "react";
import { createBetterTUIReconciler, createContainer, updateContainer } from "./renderer";
import type { OpaqueRoot, ReconcilerType } from "./renderer";

interface NodeProcess {
  stdout?: { columns?: number; rows?: number; write?: (data: string | Uint8Array) => boolean };
}

interface NodeGlobal {
  process?: NodeProcess;
  Buffer?: { from(data: Uint8Array): Uint8Array };
}

const nodeGlobal: NodeGlobal =
  (typeof globalThis !== "undefined" && (globalThis as unknown as NodeGlobal)) || {};

const isNode = !!nodeGlobal.process?.stdout;

function getTerminalSize(): { width: number; height: number } {
  const s = nodeGlobal.process?.stdout;
  const width = s?.columns || 80;
  const height = s?.rows || 24;
  return { width, height };
}

function writeStdout(data: Uint8Array): void {
  const buf = nodeGlobal.Buffer?.from(data) ?? data;
  nodeGlobal.process?.stdout?.write?.(buf);
}

// ─── Terminal Lifecycle ──────────────────────────────────────────────────────
// OpenTUI's CliRenderer handles terminal setup (alternate screen, raw mode) in
// its native Zig layer. Here we emit the same ANSI sequences from TypeScript so
// the Rust-native render path produces a visible TUI.

const CSI = "\x1b[";
const ALT_SCREEN_ENTER = `${CSI}?1049h`;
const ALT_SCREEN_LEAVE = `${CSI}?1049l`;
const HIDE_CURSOR = `${CSI}?25l`;
const SHOW_CURSOR = `${CSI}?25h`;
const CLEAR_SCREEN = `${CSI}2J`;
const CURSOR_HOME = `${CSI}H`;

function encode(s: string): Uint8Array {
  return new TextEncoder().encode(s);
}

let terminalActive = false;
let stdinRawMode = false;

function setupTerminal(): void {
  if (terminalActive) return;
  terminalActive = true;
  writeStdout(encode(HIDE_CURSOR + ALT_SCREEN_ENTER + CLEAR_SCREEN + CURSOR_HOME));
  // Enable raw mode on stdin so keyboard input works in TUI context.
  try {
    const proc = (
      nodeGlobal as unknown as {
        process?: { stdin?: { setRawMode?: (m: boolean) => void; isTTY?: boolean } };
      }
    ).process;
    if (proc?.stdin?.setRawMode && proc.stdin.isTTY) {
      proc.stdin.setRawMode(true);
      stdinRawMode = true;
    }
  } catch {
    // Not a TTY; raw mode unavailable.
  }
}

function teardownTerminal(): void {
  if (!terminalActive) return;
  terminalActive = false;
  writeStdout(encode(SHOW_CURSOR + ALT_SCREEN_LEAVE));
  try {
    if (stdinRawMode) {
      const proc = (
        nodeGlobal as unknown as { process?: { stdin?: { setRawMode?: (m: boolean) => void } } }
      ).process;
      proc?.stdin?.setRawMode?.(false);
      stdinRawMode = false;
    }
  } catch {
    // Best-effort.
  }
}

// Best-effort cleanup on process exit (covers SIGINT default handler, SIGTERM,
// process.exit(), and uncaught exceptions that trigger exit).
if (typeof process !== "undefined" && isNode) {
  process.on("exit", teardownTerminal);
}

interface NativeSession {
  reconciler: ReconcilerType;
  root: OpaqueRoot;
  buffer: CommandBuffer;
  engine: ReturnType<typeof createEngine>;
  width: number;
  height: number;
  rootId: string;
}

let nativeSession: NativeSession | null = null;
let nativeUnavailable = false;

function flushAndRender(s: NativeSession): void {
  const commands = s.buffer.drain();
  if (commands.length === 0) {
    s.engine.beginFrame();
    s.engine.render();
    s.engine.commitFrame();
    return;
  }
  const idKeys = new Set(["id", "parent", "child", "reference", "node", "newParent", "old", "new"]);
  const converted = commands.map((cmd) => {
    const out: Record<string, unknown> = { type: cmd.type };
    for (const [key, value] of Object.entries(cmd)) {
      if (key === "type") continue;
      if (idKeys.has(key) && typeof value === "string") {
        out[key] = Number(value);
      } else {
        out[key] = value;
      }
    }
    return out;
  });
  const cmdResult = s.engine.processCommands(JSON.stringify(converted));
  if (typeof cmdResult === "string") {
    try {
      const parsed = JSON.parse(cmdResult);
      if (parsed.errors?.length > 0) {
        console.error("[flushAndRender] command errors:", parsed.errors);
      }
    } catch {
      /* ignore parse errors from result */
    }
  }
  s.engine.beginFrame();
  const frame = s.engine.render();
  s.engine.commitFrame();
  if (isNode && frame) {
    const data = (frame as { outputData?: unknown }).outputData;
    if (data) {
      const bytes =
        data instanceof Uint8Array ? data : new Uint8Array(data as ArrayBuffer | number[]);
      writeStdout(bytes);
    }
  }
}

function getOrCreateNativeSession(width: number, height: number): NativeSession | null {
  if (nativeUnavailable) return null;
  if (nativeSession) {
    if (nativeSession.width !== width || nativeSession.height !== height) {
      nativeSession.engine.resize(width, height);
      nativeSession.width = width;
      nativeSession.height = height;
    }
    return nativeSession;
  }
  let engine: ReturnType<typeof createEngine>;
  try {
    engine = createEngine(width, height);
  } catch {
    nativeUnavailable = true;
    return null;
  }
  const buffer = new CommandBuffer();
  const rootId = engine.root();
  const reconciler = createBetterTUIReconciler({
    push(command) {
      buffer.push(command);
    },
  });
  const root = createContainer(
    reconciler,
    {
      push(command) {
        buffer.push(command);
      },
    },
    {
      id: rootId,
      onCommit: () => {
        if (nativeSession) flushAndRender(nativeSession);
      },
    },
  );
  nativeSession = { reconciler, root, buffer, engine, width, height, rootId };
  return nativeSession;
}

export interface RenderHandle {
  root: OpaqueRoot;
  runtime: CommandRuntime;
  dispose: () => void;
}

function renderToBuffer(element: ReactNode): RenderHandle {
  const runtime = new CommandRuntime();
  const reconciler = createBetterTUIReconciler({
    push(command) {
      runtime.commandBuffer.push(command);
    },
  });
  const root = createContainer(reconciler, {
    push(command) {
      runtime.commandBuffer.push(command);
    },
  });
  updateContainer(reconciler, element, root);
  return {
    root,
    runtime,
    dispose: () => {
      runtime.dispose();
    },
  };
}

export function render(element: ReactNode): RenderHandle {
  if (isNode) {
    const { width, height } = getTerminalSize();
    const session = getOrCreateNativeSession(width, height);
    if (session) {
      setupTerminal();
      updateContainer(session.reconciler, element, session.root);
      const runtime = new CommandRuntime();
      return {
        root: session.root,
        runtime,
        dispose: () => {
          if (nativeSession) {
            nativeSession.buffer.push({ type: "Shutdown" });
            flushAndRender(nativeSession);
            nativeSession = null;
          }
          teardownTerminal();
        },
      };
    }
  }
  return renderToBuffer(element);
}

export { renderToBuffer };

interface RuntimeContextValue {
  runtime: CommandRuntime;
  onKey: (
    handler: (
      key: string,
      modifiers: { ctrl: boolean; shift: boolean; alt: boolean; meta: boolean },
    ) => void,
  ) => () => void;
}

const RuntimeContext = createContext<RuntimeContextValue | null>(null);

export function RuntimeProvider({
  runtime,
  children,
}: {
  runtime: CommandRuntime;
  children: ReactNode;
}) {
  const keyHandlersRef = useRef<
    Set<
      (
        key: string,
        modifiers: { ctrl: boolean; shift: boolean; alt: boolean; meta: boolean },
      ) => void
    >
  >(new Set());

  const onKey = useCallback(
    (
      handler: (
        key: string,
        modifiers: { ctrl: boolean; shift: boolean; alt: boolean; meta: boolean },
      ) => void,
    ) => {
      keyHandlersRef.current.add(handler);
      return () => {
        keyHandlersRef.current.delete(handler);
      };
    },
    [],
  );

  return <RuntimeContext.Provider value={{ runtime, onKey }}>{children}</RuntimeContext.Provider>;
}

export function useRuntime(): RuntimeContextValue | null {
  return useContext(RuntimeContext);
}
