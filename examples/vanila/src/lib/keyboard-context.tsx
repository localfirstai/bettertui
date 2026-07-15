// React context + hook exposing the owned KeyInput to example components.
// Mirrors how OpenTUI passes `renderer` into `run(renderer)`; here the launcher
// owns a single KeyInput and shares it through context so examples subscribe
// to terminal keypresses (which the public useKeyboard hook cannot receive).

import type { KeyEvent } from "@bettertui/shared";
import { createContext, useContext, useEffect, useRef } from "react";
import type { KeyInput } from "./keyboard";

const KeyInputContext = createContext<KeyInput | null>(null);

export function KeyInputProvider({
  keyInput,
  children,
}: {
  keyInput: KeyInput;
  children: React.ReactNode;
}) {
  return <KeyInputContext.Provider value={keyInput}>{children}</KeyInputContext.Provider>;
}

export function useKeyInput(): KeyInput {
  const keyInput = useContext(KeyInputContext);
  if (!keyInput) {
    throw new Error("useKeyInput must be used within a KeyInputProvider");
  }
  return keyInput;
}

// Subscribes for the lifetime of the component; returns nothing. `return true`
// from the handler mirrors useKeyboard's preventDefault semantics (no-op here,
// but kept for parity of call shape).
export function useExampleKey(handler: (event: KeyEvent) => boolean): void {
  const keyInput = useKeyInput();
  const handlerRef = useRef(handler);
  handlerRef.current = handler;

  useEffect(() => {
    const unsub = keyInput.on((event) => {
      handlerRef.current(event);
    });
    return unsub;
  }, [keyInput]);
}
