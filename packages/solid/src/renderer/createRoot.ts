/**
 * `createRoot` — entry point for @bettertui/solid.
 *
 * Creates an isolated Solid reactive root bound to a `CliRenderer`. The
 * returned `Root` object mirrors the @bettertui/react API so that switching
 * between adapters is straightforward.
 *
 * ```ts
 * import { createRoot } from "@bettertui/solid";
 *
 * const root = createRoot(renderer);
 * root.render(() => <App />);
 * // later:
 * root.unmount();
 * ```
 *
 * Following OpenTUI's pattern, a top-level `render()` convenience function is
 * also exported from `index.ts`.
 */

import type { CliRenderer } from "@bettertui/core";
import { RendererContext } from "../context/rendererContext";
import { makeUniversalRenderer } from "./hostConfig";

export interface Root {
  /** Render `fn` into the renderer. Re-calling replaces the current tree. */
  render(fn: () => unknown): void;
  /** Destroy the reactive root and remove all rendered nodes. */
  unmount(): void;
}

/**
 * Create a Solid reactive root attached to `renderer`.
 *
 * The `RendererContext` is injected at the root level so every hook call
 * inside the tree can access the renderer without prop-drilling.
 */
export function createRoot(renderer: CliRenderer): Root {
  const { render: solidRender, createComponent: solidCreateComponent } =
    makeUniversalRenderer(renderer);

  const rootNativeId = renderer.rootNodeId;
  let dispose: (() => void) | null = null;

  return {
    render(fn: () => unknown): void {
      // Unmount any previous tree.
      dispose?.();
      dispose = null;

      // Wrap the user's element tree in RendererContext.Provider so every
      // hook inside the tree receives the renderer without prop-drilling.
      const wrappedFn = () =>
        solidCreateComponent(RendererContext.Provider, {
          value: renderer,
          get children() {
            return fn();
          },
        });

      dispose = solidRender(wrappedFn, rootNativeId);
      renderer.render();
    },

    unmount(): void {
      dispose?.();
      dispose = null;
    },
  };
}

/**
 * Top-level `render` convenience function.  Equivalent to:
 * ```ts
 * createRoot(renderer).render(fn);
 * ```
 * Returns an `unmount` function.
 */
export function render(fn: () => unknown, renderer: CliRenderer): () => void {
  const root = createRoot(renderer);
  root.render(fn);
  return () => root.unmount();
}
