import { Runtime } from "@bettertui/core";
import { createContext, useCallback, useContext, useRef } from "react";
import type { ReactNode } from "react";
import { createBetterTUIReconciler, createContainer, updateContainer } from "./renderer";
import type { OpaqueRoot } from "./renderer";

export function render(element: ReactNode): {
  root: OpaqueRoot;
  runtime: Runtime;
  dispose: () => void;
} {
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
