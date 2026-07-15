// Shared development keybindings installed by example screens. Maps OpenTUI's
// setupCommonDemoKeys: a quit chord plus debug toggles. Internal to examples.

import type { KeyInput } from "./keyboard";

export interface CommonExampleKeysOptions {
  onQuit?: () => void;
  onDebugToggle?: () => void;
}

// Install q/Escape to quit and a few debug affordances. Returns an unsubscribe fn.
export function useCommonExampleKeys(
  keyInput: KeyInput,
  options: CommonExampleKeysOptions = {},
): () => void {
  const handler = (event: {
    key: string;
    ctrl: boolean;
    shift: boolean;
    alt: boolean;
  }): boolean => {
    if (event.ctrl && event.key === "c") {
      options.onQuit?.();
      return true;
    }
    if (!event.ctrl && (event.key === "q" || event.key === "Escape")) {
      options.onQuit?.();
      return true;
    }
    if (!event.ctrl && event.key === ".") {
      options.onDebugToggle?.();
      return true;
    }
    return false;
  };

  return keyInput.on(handler);
}
