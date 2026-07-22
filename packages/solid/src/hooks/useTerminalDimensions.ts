/**
 * useTerminalDimensions — reactive terminal width and height.
 *
 * Returns a Solid `Signal` accessor (following OpenTUI's Solid convention of
 * returning reactive signals rather than plain objects). The signal updates
 * whenever the terminal is resized (process SIGWINCH).
 */

import { createSignal, onCleanup, onMount } from "solid-js";
import { useRenderer } from "../context/rendererContext";

export interface TerminalDimensions {
  width: number;
  height: number;
}

export function useTerminalDimensions(): () => TerminalDimensions {
  const renderer = useRenderer();
  const [dims, setDims] = createSignal<TerminalDimensions>({
    width: renderer.terminalWidth,
    height: renderer.terminalHeight,
  });

  const handleResize = () => {
    setDims({ width: renderer.terminalWidth, height: renderer.terminalHeight });
  };

  onMount(() => {
    process.on("SIGWINCH", handleResize);
  });

  onCleanup(() => {
    process.off("SIGWINCH", handleResize);
  });

  return dims;
}
