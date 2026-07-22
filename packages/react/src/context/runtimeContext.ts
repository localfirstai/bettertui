import type { CliRenderer } from "@bettertui/core";
import { createContext, useContext } from "react";

export interface RuntimeContextValue {
  /** The CliRenderer this React tree is mounted in. */
  renderer: CliRenderer | null;
}

export const RuntimeContext = createContext<RuntimeContextValue>({
  renderer: null,
});

/**
 * Access the current RuntimeContext value.
 * Throws if accessed outside of a <RuntimeContext.Provider>.
 */
export function useRuntimeContext(): RuntimeContextValue {
  return useContext(RuntimeContext);
}
