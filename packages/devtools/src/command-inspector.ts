import type { CommandType, RecordedCommand } from "./types";

export interface CommandInspectorOptions {
  maxCommands?: number | undefined;
  onCommand?: ((command: RecordedCommand) => void) | undefined;
}

export class CommandInspector {
  private commands: RecordedCommand[] = [];
  private nextId = 0;
  private maxCommands: number;
  private onCommand: ((command: RecordedCommand) => void) | undefined;
  private commandCounts = new Map<string, number>();

  constructor(options: CommandInspectorOptions = {}) {
    this.maxCommands = options.maxCommands ?? 5000;
    this.onCommand = options.onCommand;
  }

  record(type: CommandType, payload: Record<string, unknown>, duration?: number): RecordedCommand {
    const command: RecordedCommand = {
      id: this.nextId++,
      timestamp: performance.now(),
      type,
      payload,
      duration,
    };

    this.commands.push(command);
    if (this.commands.length > this.maxCommands) {
      this.commands.shift();
    }

    this.commandCounts.set(type, (this.commandCounts.get(type) ?? 0) + 1);
    this.onCommand?.(command);

    return command;
  }

  getCommands(): readonly RecordedCommand[] {
    return this.commands;
  }

  getCommandsByType(type: string): RecordedCommand[] {
    return this.commands.filter((c) => c.type === type);
  }

  getCommandsInRange(start: number, end: number): RecordedCommand[] {
    return this.commands.filter((c) => c.timestamp >= start && c.timestamp <= end);
  }

  getCounts(): Map<string, number> {
    return new Map(this.commandCounts);
  }

  getTotalCount(): number {
    return this.commands.length;
  }

  getRecent(count: number): RecordedCommand[] {
    return this.commands.slice(-count);
  }

  clear(): void {
    this.commands = [];
    this.commandCounts.clear();
  }

  /** Get a summary of command activity */
  getSummary(): {
    total: number;
    byType: Record<string, number>;
    lastTimestamp: number | null;
    avgCommandsPerFrame: number;
  } {
    const byType: Record<string, number> = {};
    for (const [type, count] of this.commandCounts) {
      byType[type] = count;
    }

    const lastCommand =
      this.commands.length > 0 ? this.commands[this.commands.length - 1] : undefined;
    const lastTimestamp = lastCommand != null ? lastCommand.timestamp : null;

    return {
      total: this.commands.length,
      byType,
      lastTimestamp,
      avgCommandsPerFrame: 0, // Computed externally if needed
    };
  }
}
