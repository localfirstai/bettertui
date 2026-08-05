import { MockKeyboard } from "../keyboard/keyboard";
import { MockMouse } from "../mouse/mouse";
import { TestTerminal } from "../terminal/test-terminal";

export interface FrameworkTestingDriver {
  name: string;
  render(component: unknown): Promise<TestTerminal>;
  type(text: string): Promise<void>;
  pressKey(key: string): Promise<void>;
  click(x: number, y: number): Promise<void>;
  getFrameText(): string;
  unmount(): Promise<void>;
}

export class BetterTUIDriver implements FrameworkTestingDriver {
  public name = "BetterTUI";
  private activeTerminal: TestTerminal | null = null;

  public async render(_component: unknown): Promise<TestTerminal> {
    this.activeTerminal = new TestTerminal({ width: 80, height: 24 });
    return this.activeTerminal;
  }

  public async type(text: string): Promise<void> {
    if (!this.activeTerminal)
      throw new Error("BetterTUIDriver: No active terminal render session.");
    const keyboard = new MockKeyboard(this.activeTerminal);
    await keyboard.type(text);
  }

  public async pressKey(key: string): Promise<void> {
    if (!this.activeTerminal)
      throw new Error("BetterTUIDriver: No active terminal render session.");
    const keyboard = new MockKeyboard(this.activeTerminal);
    keyboard.press(key);
  }

  public async click(x: number, y: number): Promise<void> {
    if (!this.activeTerminal)
      throw new Error("BetterTUIDriver: No active terminal render session.");
    const mouse = new MockMouse(this.activeTerminal);
    await mouse.click(x, y);
  }

  public getFrameText(): string {
    if (!this.activeTerminal) return "";
    return this.activeTerminal.matrix.renderTextFrame();
  }

  public async unmount(): Promise<void> {
    if (this.activeTerminal) {
      this.activeTerminal.clear();
      this.activeTerminal = null;
    }
  }
}
