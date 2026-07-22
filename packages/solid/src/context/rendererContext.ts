/**
 * Solid context that provides the active CliRenderer to all hooks.
 *
 * Naming follows OpenTUI's convention: `RendererContext` + `useRenderer()`.
 * Components must be rendered inside a `RendererProvider` (set up by
 * `createRoot` automatically) to use hooks.
 */

import type { CliRenderer } from "@bettertui/core";
import { createContext, useContext } from "solid-js";

/** Context carrying the active `CliRenderer` instance. */
export const RendererContext = createContext<CliRenderer | undefined>(undefined);

/**
 * Returns the `CliRenderer` from the nearest `RendererProvider`.
 * Throws when called outside a rendered tree.
 */
export function useRenderer(): CliRenderer {
  const renderer = useContext(RendererContext);
  if (!renderer) {
    throw new Error(
      "@bettertui/solid: useRenderer() called outside a RendererProvider. " +
        "Ensure your component tree is wrapped by createRoot().",
    );
  }
  return renderer;
}
