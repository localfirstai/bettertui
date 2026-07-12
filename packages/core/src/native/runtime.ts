import type { Command, CommandBuffer } from "../command-buffer.js";
import type { NapiEngine, NapiEventBus, ProcessResult } from "./types.js";

export interface RenderResult {
  output_data: number[];
  width: number;
  height: number;
  dirty_region_count: number;
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

export function createRuntime(
  engine: NapiEngine,
  eventBus: NapiEventBus,
  buffer: CommandBuffer,
): Runtime {
  function serializeCommands(commands: Command[]): string {
    return JSON.stringify(commands);
  }

  function processCommands(): ProcessResult {
    const commands = buffer.drain();
    if (commands.length === 0) {
      return { success: 0, errors: [], idMappings: [] };
    }
    const json = serializeCommands(commands);
    const resultStr = engine.processCommands(json);
    return JSON.parse(resultStr) as ProcessResult;
  }

  function renderFrame(): RenderResult {
    engine.beginFrame();
    const result = engine.render();
    engine.commitFrame();
    return result as unknown as RenderResult;
  }

  function resize(width: number, height: number): void {
    engine.resize(width, height);
  }

  function shutdown(): void {
    buffer.push({ type: "Shutdown" });
    processCommands();
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
