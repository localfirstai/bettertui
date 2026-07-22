import { useEffect, useState } from "react";
import { useRuntime } from "./useRuntime";

export interface TerminalDimensions {
  width: number;
  height: number;
}

/**
 * Returns the current terminal dimensions and updates whenever the terminal
 * is resized (via SIGWINCH).
 */
export function useTerminalDimensions(): TerminalDimensions {
  const renderer = useRuntime();
  const [dims, setDims] = useState<TerminalDimensions>({
    width: renderer.terminalWidth,
    height: renderer.terminalHeight,
  });

  useEffect(() => {
    const onResize = () => {
      setDims({ width: renderer.terminalWidth, height: renderer.terminalHeight });
    };
    process.on("SIGWINCH", onResize);
    return () => {
      process.off("SIGWINCH", onResize);
    };
  }, [renderer]);

  return dims;
}
