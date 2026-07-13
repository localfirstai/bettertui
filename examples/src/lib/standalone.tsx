// Mount helpers shared by every example's run/destroy/standalone contract.
// Internal to the examples package.

import { render } from "@bettertui/react";
import type { RenderHandle } from "@bettertui/react";
import { KeyInput } from "./keyboard";
import { KeyInputProvider } from "./keyboard-context";
import type { ExampleModule } from "./meta";
import { exampleThemes } from "./theme";

let activeHandle: RenderHandle | null = null;

// Mount an example's React component through the real renderer, wired with the
// keyboard context the example's components read via useExampleKey().
export function mountExample(Example: React.FC, keyInput: KeyInput): RenderHandle {
  if (activeHandle) {
    activeHandle.dispose();
    activeHandle = null;
  }
  const handle = render(
    <KeyInputProvider keyInput={keyInput}>
      <Example />
    </KeyInputProvider>,
  );
  activeHandle = handle;
  return handle;
}

// Disposes the active render handle (called by destroy() and launcher Escape).
export function disposeActive(): void {
  if (activeHandle) {
    activeHandle.dispose();
    activeHandle = null;
  }
}

// Standard standalone entry: create a KeyInput, run, wire quit on q/escape.
export function runStandalone(module: ExampleModule, keyInput = new KeyInput()): void {
  keyInput.start();
  keyInput.on((event) => {
    if ((event.key === "q" || event.key === "Escape") && !event.ctrl) {
      module.destroy(keyInput);
      keyInput.stop();
      process.exit(0);
    }
  });
  module.run(keyInput);
}

export { exampleThemes };
