import type { Command, CommandBuffer } from "../command-buffer";
import type { NapiEngine, NapiEventBus, NapiRenderResult, ProcessResult } from "./types";

export interface RenderResult {
  outputData: Uint8Array;
  width: number;
  height: number;
  dirtyRegionCount: number;
}

export interface RuntimeOptions {
  width?: number;
  height?: number;
  fps?: number;
}

export interface Runtime {
  engine: NapiEngine;
  eventBus: NapiEventBus;
  buffer: CommandBuffer;
  processCommands(): ProcessResult;
  renderFrame(): RenderResult;
  resize(width: number, height: number): void;
  shutdown(): void;
}

const ID_KEYS = new Set(["id", "parent", "child", "reference", "node", "newParent", "old", "new"]);

function toEngineJson(commands: Command[]): string {
  const converted = commands.map((cmd) => {
    const out: Record<string, unknown> = { type: cmd.type };
    for (const [key, value] of Object.entries(cmd)) {
      if (key === "type") continue;
      if (ID_KEYS.has(key) && typeof value === "string") {
        out[key] = Number(value);
      } else {
        out[key] = value;
      }
    }
    return out;
  });
  return JSON.stringify(converted);
}

export function createRuntime(
  engine: NapiEngine,
  eventBus: NapiEventBus,
  buffer: CommandBuffer,
): Runtime {
  function processCommands(): ProcessResult {
    const commands = buffer.drain();
    if (commands.length === 0) {
      return { success: 0, errors: [], idMappings: [] };
    }
    const json = toEngineJson(commands);
    const resultStr = engine.processCommands(json);
    const result = JSON.parse(resultStr) as ProcessResult;
    if (result.errors.length > 0) console.error("ENGINE ERRORS:", result.errors);
    return result;
  }

  function renderFrame(): RenderResult {
    engine.beginFrame();
    const result: NapiRenderResult = engine.render();
    engine.commitFrame();
    return {
      outputData: result.outputData,
      width: result.width,
      height: result.height,
      dirtyRegionCount: result.dirtyRegionCount,
    };
  }

  function resize(width: number, height: number): void {
    engine.resize(width, height);
  }

  function shutdown(): void {
    buffer.push({ type: "Shutdown" });
    processCommands();
    engine.shutdown();
  }

  return {
    engine,
    eventBus,
    buffer,
    processCommands,
    renderFrame,
    resize,
    shutdown,
  };
}
