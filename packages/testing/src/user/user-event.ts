import { type KeyboardModifiers, MockKeyboard } from "../keyboard/keyboard";
import { MockMouse, MouseButton } from "../mouse/mouse";
import type { TargetElement } from "../screen/screen";
import type { TestTerminal } from "../terminal/test-terminal";

export class UserEventInstance {
  public readonly keyboard: MockKeyboard;
  public readonly mouse: MockMouse;

  constructor(private readonly terminal: TestTerminal) {
    this.keyboard = new MockKeyboard(terminal);
    this.mouse = new MockMouse(terminal);
  }

  public async type(targetOrText: TargetElement | string, text?: string): Promise<void> {
    if (typeof targetOrText === "string") {
      await this.keyboard.type(targetOrText);
    } else if (text) {
      await this.mouse.click(targetOrText.x, targetOrText.y);
      await this.keyboard.type(text);
    }
  }

  public async click(target: TargetElement): Promise<void> {
    await this.mouse.click(target.x, target.y, MouseButton.Left);
  }

  public async doubleClick(target: TargetElement): Promise<void> {
    await this.mouse.doubleClick(target.x, target.y, MouseButton.Left);
  }

  public async pressKey(key: string, modifiers?: KeyboardModifiers): Promise<void> {
    this.keyboard.press(key, modifiers);
  }

  public async paste(content: string): Promise<void> {
    this.keyboard.paste(content);
  }

  public async scroll(target: TargetElement, direction: "up" | "down", delta = 1): Promise<void> {
    this.mouse.scroll(target.x, target.y, direction, delta);
  }
}

export function createUserEvent(terminal: TestTerminal): UserEventInstance {
  return new UserEventInstance(terminal);
}
