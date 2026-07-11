import {
  type OpaqueRoot,
  createBetterTUIReconciler,
  createContainer,
  updateContainer,
} from "@bettertui/reconciler";
import type { ReactNode } from "react";
import { Runtime } from "./runtime";

export function render(element: ReactNode): {
  root: OpaqueRoot;
  runtime: Runtime;
  dispose: () => void;
} {
  const runtime = new Runtime();
  const reconciler = createBetterTUIReconciler({
    push(command) {
      runtime.commandBuffer.push(command);
    },
  });
  const root = createContainer(reconciler, {
    push(command) {
      runtime.commandBuffer.push(command);
    },
  });
  updateContainer(reconciler, element, root);

  return {
    root,
    runtime,
    dispose: () => {
      runtime.dispose();
    },
  };
}
