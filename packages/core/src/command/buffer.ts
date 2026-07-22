import type { Command } from "./command.types";

export class CommandBuffer {
  private commands: Command[] = [];

  push(command: Command): void {
    this.commands.push(command);
  }

  drain(): Command[] {
    const commands = this.commands;
    this.commands = [];
    return commands;
  }

  peek(): readonly Command[] {
    return this.commands;
  }

  clear(): void {
    this.commands = [];
  }

  get length(): number {
    return this.commands.length;
  }

  get isEmpty(): boolean {
    return this.commands.length === 0;
  }
}
