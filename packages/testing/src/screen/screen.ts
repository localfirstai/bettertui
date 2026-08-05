import type { TestTerminal } from "../terminal/test-terminal";

export interface TargetElement {
  text: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

export class ScreenQueryEngine {
  private activeTerminal: TestTerminal | null = null;

  public setTerminal(terminal: TestTerminal): void {
    this.activeTerminal = terminal;
  }

  private get terminal(): TestTerminal {
    if (!this.activeTerminal) {
      throw new Error(
        "ScreenQueryEngine has no active terminal instance set. Call setTerminal() first or use render().",
      );
    }
    return this.activeTerminal;
  }

  public getByText(matcher: string | RegExp): TargetElement {
    const found = this.queryByText(matcher);
    if (!found) {
      const currentFrame = this.terminal.matrix.renderTextFrame();
      throw new Error(
        `getByText: Unable to find an element with text matching: ${matcher}\n\nCurrent Terminal Frame:\n${currentFrame}`,
      );
    }
    return found;
  }

  public queryByText(matcher: string | RegExp): TargetElement | null {
    const frame = this.terminal.matrix.renderTextFrame();
    const lines = frame.split("\n");

    for (let y = 0; y < lines.length; y++) {
      const line = lines[y];
      const matchIndex = typeof matcher === "string" ? line.indexOf(matcher) : line.search(matcher);

      if (matchIndex !== -1) {
        const matchText = typeof matcher === "string" ? matcher : line.match(matcher)?.[0] || "";
        return {
          text: matchText,
          x: matchIndex,
          y,
          width: matchText.length,
          height: 1,
        };
      }
    }

    return null;
  }

  public async findByText(matcher: string | RegExp, timeoutMs = 1000): Promise<TargetElement> {
    const startTime = Date.now();
    while (Date.now() - startTime < timeoutMs) {
      const found = this.queryByText(matcher);
      if (found) return found;
      await new Promise((r) => setTimeout(r, 20));
    }
    return this.getByText(matcher);
  }

  public getByRole(role: string): TargetElement {
    return this.getByText(`[${role}]`);
  }

  public debug(): void {
    console.log("=== BetterTUI Screen Frame Output ===");
    console.log(this.terminal.matrix.renderTextFrame());
    console.log("=====================================");
  }
}

export const screen = new ScreenQueryEngine();
