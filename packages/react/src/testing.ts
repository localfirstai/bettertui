import { CommandBuffer } from "@bettertui/core";
import { createEngine } from "@bettertui/core";
import type { ReactNode } from "react";
import { createBetterTUIReconciler, createContainer, updateContainer } from "./renderer";

export interface TestRendererOptions {
  width?: number;
  height?: number;
}

export async function renderToStringAsync(
  element: ReactNode,
  options?: TestRendererOptions,
): Promise<string> {
  const width = options?.width ?? 80;
  const height = options?.height ?? 24;
  const engine = createEngine(width, height);
  const buffer = new CommandBuffer();
  const rootId = engine.root();

  let resolveRender: () => void;
  const renderPromise = new Promise<void>((resolve) => {
    resolveRender = resolve;
  });

  const flush = () => {
    const commands = buffer.drain();
    if (commands.length > 0) {
      const idKeys = new Set([
        "id",
        "parent",
        "child",
        "reference",
        "node",
        "newParent",
        "old",
        "new",
      ]);
      const converted = commands.map((cmd) => {
        const out: Record<string, unknown> = { type: cmd.type };
        for (const [key, value] of Object.entries(cmd)) {
          if (key === "type") continue;
          if (idKeys.has(key) && typeof value === "string") {
            out[key] = Number(value);
          } else {
            out[key] = value;
          }
        }
        return out;
      });
      engine.processCommands(JSON.stringify(converted));
    }
    resolveRender();
  };

  const reconciler = createBetterTUIReconciler({
    push(command) {
      buffer.push(command);
    },
  });

  const root = createContainer(
    reconciler,
    {
      push(command) {
        buffer.push(command);
      },
    },
    {
      id: rootId,
      onCommit: flush,
    },
  );

  // Pass the resolve function to the updateContainer callback
  // which will run after the component mounts
  updateContainer(reconciler, element, root, () => {
    // If there were any commands they would have been flushed by onCommit.
    // However if nothing was flushed we ensure the promise resolves here.
    resolveRender();
  });

  // Wait for React to finish rendering and committing
  await renderPromise;

  // Wait an extra tick for any pending effects
  await new Promise((r) => setTimeout(r, 10));

  engine.beginFrame();
  const frame = engine.renderFull();
  engine.commitFrame();

  if (!frame || !frame.outputData) {
    return "";
  }

  const decoder = new TextDecoder();
  const data = frame.outputData;
  return decoder.decode(
    Buffer.isBuffer(data) || ArrayBuffer.isView(data) ? data : new Uint8Array(data),
  );
}
