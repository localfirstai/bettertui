/**
 * Mouse event parser for X10 and SGR mouse sequences.
 * Parses terminal mouse escape sequences into RawMouseEvent objects.
 */

export type MouseEventType =
  | "down"
  | "up"
  | "move"
  | "drag"
  | "drag-end"
  | "drop"
  | "over"
  | "out"
  | "scroll";

export interface ScrollInfo {
  direction: "up" | "down" | "left" | "right";
  delta: number;
}

export interface RawMouseEvent {
  type: MouseEventType;
  button: number;
  x: number;
  y: number;
  modifiers: { shift: boolean; alt: boolean; ctrl: boolean };
  scroll?: ScrollInfo;
}

type ParsedMouseSequence = {
  event: RawMouseEvent;
  consumed: number;
};

export class MouseParser {
  private mouseButtonsPressed = new Set<number>();

  private static readonly SCROLL_DIRECTIONS: Record<number, "up" | "down" | "left" | "right"> = {
    0: "up",
    1: "down",
    2: "left",
    3: "right",
  };

  public reset(): void {
    this.mouseButtonsPressed.clear();
  }

  private decodeInput(data: Uint8Array): string {
    return Buffer.from(data.buffer, data.byteOffset, data.byteLength).toString("latin1");
  }

  public parseMouseEvent(data: Uint8Array): RawMouseEvent | null {
    const str = this.decodeInput(data);
    const parsed = this.parseMouseSequenceAt(str, 0);
    return parsed?.event ?? null;
  }

  public parseAllMouseEvents(data: Uint8Array): RawMouseEvent[] {
    const str = this.decodeInput(data);
    const events: RawMouseEvent[] = [];
    let offset = 0;

    while (offset < str.length) {
      const parsed = this.parseMouseSequenceAt(str, offset);
      if (!parsed) {
        break;
      }

      events.push(parsed.event);
      offset += parsed.consumed;
    }

    return events;
  }

  private parseMouseSequenceAt(str: string, offset: number): ParsedMouseSequence | null {
    if (!str.startsWith("\x1b[", offset)) return null;
    const introducer = str[offset + 2];

    if (introducer === "<") {
      return this.parseSgrSequence(str, offset);
    }

    if (introducer === "M") {
      return this.parseBasicSequence(str, offset);
    }

    return null;
  }

  private parseSgrSequence(str: string, offset: number): ParsedMouseSequence | null {
    let index = offset + 3;
    const values: [number, number, number] = [0, 0, 0];
    let part = 0;
    let hasDigit = false;

    while (index < str.length) {
      const char = str[index];
      const charCode = str.charCodeAt(index);

      if (charCode >= 48 && charCode <= 57) {
        hasDigit = true;
        const currentVal = values[part] ?? 0;
        values[part] = currentVal * 10 + (charCode - 48);
        index++;
        continue;
      }

      switch (char) {
        case ";": {
          if (!hasDigit || part >= 2) return null;
          part++;
          hasDigit = false;
          index++;
          break;
        }
        case "M":
        case "m": {
          if (!hasDigit || part !== 2) return null;

          return {
            event: this.decodeSgrEvent(values[0], values[1], values[2], char),
            consumed: index - offset + 1,
          };
        }
        default:
          return null;
      }
    }

    return null;
  }

  private parseBasicSequence(str: string, offset: number): ParsedMouseSequence | null {
    if (offset + 6 > str.length) return null;

    const buttonByte = str.charCodeAt(offset + 3) - 32;
    const x = str.charCodeAt(offset + 4) - 33;
    const y = str.charCodeAt(offset + 5) - 33;

    return {
      event: this.decodeBasicEvent(buttonByte, x, y),
      consumed: 6,
    };
  }

  private decodeSgrEvent(
    rawButtonCode: number,
    wireX: number,
    wireY: number,
    pressRelease: "M" | "m",
  ): RawMouseEvent {
    const button = rawButtonCode & 3;
    const isScroll = (rawButtonCode & 64) !== 0;

    const isMotion = (rawButtonCode & 32) !== 0;
    const modifiers = {
      shift: (rawButtonCode & 4) !== 0,
      alt: (rawButtonCode & 8) !== 0,
      ctrl: (rawButtonCode & 16) !== 0,
    };

    let type: MouseEventType;
    let scrollInfo: ScrollInfo | undefined;

    if (isMotion) {
      const isDragging = this.mouseButtonsPressed.size > 0;

      if (button === 3) {
        type = "move";
      } else if (isDragging) {
        type = "drag";
      } else {
        type = "move";
      }
    } else if (isScroll && pressRelease === "M") {
      type = "scroll";
      const direction = MouseParser.SCROLL_DIRECTIONS[button];
      scrollInfo = direction
        ? {
            direction,
            delta: 1,
          }
        : undefined;
    } else {
      type = pressRelease === "M" ? "down" : "up";

      if (type === "down" && button !== 3) {
        this.mouseButtonsPressed.add(button);
      } else if (type === "up") {
        this.mouseButtonsPressed.clear();
      }
    }

    return {
      type,
      button: button === 3 ? 0 : button,
      x: wireX - 1,
      y: wireY - 1,
      modifiers,
      ...(scrollInfo ? { scroll: scrollInfo } : {}),
    };
  }

  private decodeBasicEvent(buttonByte: number, x: number, y: number): RawMouseEvent {
    const button = buttonByte & 3;
    const isScroll = (buttonByte & 64) !== 0;
    const isMotion = (buttonByte & 32) !== 0;

    const modifiers = {
      shift: (buttonByte & 4) !== 0,
      alt: (buttonByte & 8) !== 0,
      ctrl: (buttonByte & 16) !== 0,
    };

    let type: MouseEventType;
    let actualButton: number;
    let scrollInfo: ScrollInfo | undefined;

    if (isMotion) {
      type = "move";
      actualButton = button === 3 ? -1 : button;
    } else if (isScroll) {
      type = "scroll";
      actualButton = 0;
      const direction = MouseParser.SCROLL_DIRECTIONS[button];
      scrollInfo = direction
        ? {
            direction,
            delta: 1,
          }
        : undefined;
    } else {
      type = button === 3 ? "up" : "down";
      actualButton = button === 3 ? 0 : button;
    }

    return {
      type,
      button: actualButton,
      x,
      y,
      modifiers,
      ...(scrollInfo ? { scroll: scrollInfo } : {}),
    };
  }
}
