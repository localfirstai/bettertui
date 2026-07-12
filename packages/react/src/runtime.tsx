import { CommandBuffer, Runtime } from "@bettertui/core";
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
  s.engine.processCommands(JSON.stringify(converted));
  s.engine.beginFrame();
  const frame = s.engine.render();
  s.engine.commitFrame();
  if (isNode && frame && (frame as { outputData?: Uint8Array }).outputData) {
    writeStdout((frame as { outputData: Uint8Array }).outputData);
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
  runtime: Runtime;
  dispose: () => void;
}

function renderToBuffer(element: ReactNode): RenderHandle {
  const runtime = new Runtime();
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
      updateContainer(session.reconciler, element, session.root);
      const runtime = new Runtime();
      return {
        root: session.root,
        runtime,
        dispose: () => {
          if (nativeSession) {
            nativeSession.buffer.push({ type: "Shutdown" });
            flushAndRender(nativeSession);
            nativeSession = null;
          }
        },
      };
    }
  }
  return renderToBuffer(element);
}

export { renderToBuffer };

interface RuntimeContextValue {
  runtime: Runtime;
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
  runtime: Runtime;
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
