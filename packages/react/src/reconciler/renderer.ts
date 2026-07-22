import type { CliRenderer } from "@bettertui/core";
import React, { type ReactNode } from "react";
import ReactReconciler from "react-reconciler";
import { ConcurrentRoot } from "react-reconciler/constants";
import { RuntimeContext } from "../context/runtimeContext";
import type { BetterTUIContainer } from "../types/host.types";
import { makeHostConfig } from "./hostConfig";

export interface Root {
  /** Mount or update a React tree in this root. */
  render(node: ReactNode): void;
  /** Unmount the React tree and clean up. */
  unmount(): void;
}

/**
 * Create a BetterTUI React root bound to the given {@link CliRenderer}.
 *
 * ```ts
 * import { createRoot } from "@bettertui/react";
 *
 * const renderer = new CliRenderer();
 * const root = createRoot(renderer);
 *
 * renderer.start();
 * root.render(<App />);
 * ```
 */
export function createRoot(renderer: CliRenderer): Root {
  // Each root gets its own reconciler so the renderer reference is captured
  // in closure without cross-root contamination.
  const hostConfig = makeHostConfig(renderer);
  // biome-ignore lint/suspicious/noExplicitAny: react-reconciler types don't align with our custom host config
  const reconciler = ReactReconciler(hostConfig as any);

  const container: BetterTUIContainer = {
    renderer,
    rootNativeId: renderer.rootNodeId,
  };

  const reactContainer = reconciler.createContainer(
    container,
    ConcurrentRoot,
    null, // hydrationCallbacks
    false, // isStrictMode
    null, // concurrentUpdatesByDefaultOverride
    "", // identifierPrefix
    console.error, // onUncaughtError
    console.error, // onCaughtError
    console.error, // onRecoverableError
    () => {}, // transitionCallbacks
  );

  try {
    reconciler.injectIntoDevTools({
      bundleType: process.env.NODE_ENV === "production" ? 0 : 1,
      version: "19.0.0",
      rendererPackageName: "@bettertui/react",
    });
  } catch {
    // DevTools not available — safe to ignore
  }

  function render(node: ReactNode): void {
    reconciler.updateContainer(
      React.createElement(RuntimeContext.Provider, { value: { renderer } }, node),
      reactContainer,
      null,
      () => {},
    );
  }

  function unmount(): void {
    reconciler.updateContainer(null, reactContainer, null, () => {});
    const r = reconciler as unknown as Record<string, unknown>;
    if (typeof r["flushSyncWork"] === "function") {
      (r["flushSyncWork"] as () => void)();
    }
  }

  return { render, unmount };
}
