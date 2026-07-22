import type { CliRenderer } from "../platform/cliRenderer";

export const KeyCodes = {
  RETURN: "\r",
  LINEFEED: "\n",
  TAB: "\t",
  BACKSPACE: "\x7f",
  DELETE: "\x1b[3~",
  HOME: "\x1b[H",
  END: "\x1b[F",
  ESCAPE: "\x1b",

  ARROW_UP: "\x1b[A",
  ARROW_DOWN: "\x1b[B",
  ARROW_RIGHT: "\x1b[C",
  ARROW_LEFT: "\x1b[D",

  F1: "\x1bOP",
  F2: "\x1bOQ",
  F3: "\x1bOR",
  F4: "\x1bOS",
  F5: "\x1b[15~",
  F6: "\x1b[17~",
  F7: "\x1b[18~",
  F8: "\x1b[19~",
  F9: "\x1b[20~",
  F10: "\x1b[21~",
  F11: "\x1b[23~",
  F12: "\x1b[24~",

  PAGE_UP: "\x1b[5~",
  PAGE_DOWN: "\x1b[6~",
} as const;

export type TestKeyInput = string | keyof typeof KeyCodes;

export interface MockKeysOptions {
  kittyKeyboard?: boolean | undefined;
}

export interface KeyModifiers {
  shift?: boolean;
  ctrl?: boolean;
  alt?: boolean;
  meta?: boolean;
}

function resolveKeyInput(key: TestKeyInput): string {
  if (typeof key === "string") {
    if (key in KeyCodes) {
      return KeyCodes[key as keyof typeof KeyCodes];
    }
    return key;
  }
  return KeyCodes[key];
}

export function createMockKeys(_renderer: CliRenderer, _options?: MockKeysOptions) {
  const keyHistory: string[] = [];

  const pressKey = (key: TestKeyInput, modifiers?: KeyModifiers): void => {
    let keyCode = resolveKeyInput(key);
    keyHistory.push(keyCode);

    if (modifiers) {
      if (modifiers.ctrl && keyCode.length === 1) {
        const char = keyCode.toLowerCase();
        if (char >= "a" && char <= "z") {
          keyCode = String.fromCharCode(char.charCodeAt(0) - 96);
        }
      }
      if (modifiers.alt) {
        keyCode = `\x1b${keyCode}`;
      }
    }

    process.stdin.emit("data", Buffer.from(keyCode));
  };

  const pressKeys = async (keys: TestKeyInput[], delayMs = 0): Promise<void> => {
    for (const key of keys) {
      pressKey(key);
      if (delayMs > 0) {
        await new Promise((resolve) => setTimeout(resolve, delayMs));
      }
    }
  };

  const typeText = async (text: string, delayMs = 0): Promise<void> => {
    const keys = text.split("");
    await pressKeys(keys, delayMs);
  };

  const pressEnter = (modifiers?: KeyModifiers): void => {
    pressKey(KeyCodes.RETURN, modifiers);
  };

  const pressEscape = (modifiers?: KeyModifiers): void => {
    pressKey(KeyCodes.ESCAPE, modifiers);
  };

  const pressTab = (modifiers?: KeyModifiers): void => {
    pressKey(KeyCodes.TAB, modifiers);
  };

  const pressBackspace = (modifiers?: KeyModifiers): void => {
    pressKey(KeyCodes.BACKSPACE, modifiers);
  };

  const pressArrow = (
    direction: "up" | "down" | "left" | "right",
    modifiers?: KeyModifiers,
  ): void => {
    const keyMap = {
      up: KeyCodes.ARROW_UP,
      down: KeyCodes.ARROW_DOWN,
      left: KeyCodes.ARROW_LEFT,
      right: KeyCodes.ARROW_RIGHT,
    };
    pressKey(keyMap[direction], modifiers);
  };

  const pressCtrlC = (): void => {
    pressKey("c", { ctrl: true });
  };

  const pressCtrlD = (): void => {
    pressKey("d", { ctrl: true });
  };

  const pressPageUp = (modifiers?: KeyModifiers): void => {
    pressKey(KeyCodes.PAGE_UP, modifiers);
  };

  const pressPageDown = (modifiers?: KeyModifiers): void => {
    pressKey(KeyCodes.PAGE_DOWN, modifiers);
  };

  const pressHome = (modifiers?: KeyModifiers): void => {
    pressKey(KeyCodes.HOME, modifiers);
  };

  const pressEnd = (modifiers?: KeyModifiers): void => {
    pressKey(KeyCodes.END, modifiers);
  };

  const pressDelete = (modifiers?: KeyModifiers): void => {
    pressKey(KeyCodes.DELETE, modifiers);
  };

  const getKeyHistory = (): string[] => [...keyHistory];
  const clearHistory = (): void => {
    keyHistory.length = 0;
  };

  return {
    pressKey,
    pressKeys,
    typeText,
    pressEnter,
    pressEscape,
    pressTab,
    pressBackspace,
    pressArrow,
    pressCtrlC,
    pressCtrlD,
    pressPageUp,
    pressPageDown,
    pressHome,
    pressEnd,
    pressDelete,
    getKeyHistory,
    clearHistory,
  };
}
