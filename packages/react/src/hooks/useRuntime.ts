import type { CliRenderer } from "@bettertui/core";
import { useRuntimeContext } from "../context/runtimeContext";

/**
 * Returns the {@link CliRenderer} this React tree is mounted in.
 * Throws if called outside a `createRoot` tree.
 */
export function useRuntime(): CliRenderer {
  const { renderer } = useRuntimeContext();
  if (!renderer) {
    throw new Error("useRuntime() must be called inside a component rendered via createRoot().");
  }
  return renderer;
}
