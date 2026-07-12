import type { KeyEvent as SharedKeyEvent, MouseEvent as SharedMouseEvent } from "@bettertui/shared";
import type { NapiEventBus } from "./types";

// Re-export shared event types for consumer convenience
export type KeyEvent = SharedKeyEvent;
export type MouseEvent = SharedMouseEvent;

export type EventCallback = (event: SharedKeyEvent | SharedMouseEvent) => void;

export interface EventLoop {
  start(): void;
  stop(): void;
  pushKey(key: string, ctrl: boolean, shift: boolean, alt: boolean, targetId: number): void;
  pushMouse(button: string, x: number, y: number, targetId: number): void;
  drain(): string;
  onEvent(callback: EventCallback): void;
}

export function createEventLoop(eventBus: NapiEventBus): EventLoop {
  const callbacks: EventCallback[] = [];
  let running = false;
  let drainInterval: ReturnType<typeof setInterval> | null = null;

  function start(): void {
    if (running) return;
    running = true;
    drainInterval = setInterval(() => {
      const raw = eventBus.drain();
      if (raw) {
        try {
          const events = JSON.parse(raw) as Array<SharedKeyEvent | SharedMouseEvent>;
          for (const event of events) {
            for (const cb of callbacks) {
              cb(event);
            }
          }
        } catch {
          // Malformed event data, skip
        }
      }
    }, 16);
  }

  function stop(): void {
    running = false;
    if (drainInterval !== null) {
      clearInterval(drainInterval);
      drainInterval = null;
    }
  }

  function pushKey(
    key: string,
    ctrl: boolean,
    shift: boolean,
    alt: boolean,
    targetId: number,
  ): void {
    const keyEvent: SharedKeyEvent = {
      key,
      code: "",
      ctrl,
      shift,
      alt,
      meta: false,
    };
    eventBus.pushKey(key, ctrl, shift, alt, targetId);
    for (const cb of callbacks) {
      cb(keyEvent);
    }
  }

  function pushMouse(button: string, x: number, y: number, targetId: number): void {
    const mouseEvent: SharedMouseEvent = {
      button: button as "left" | "right" | "middle" | "none",
      position: { x, y },
      ctrl: false,
      shift: false,
      alt: false,
    };
    eventBus.pushMouse(button, x, y, targetId);
    for (const cb of callbacks) {
      cb(mouseEvent);
    }
  }

  function drain(): string {
    return eventBus.drain();
  }

  function onEvent(callback: EventCallback): void {
    callbacks.push(callback);
  }

  return {
    start,
    stop,
    pushKey,
    pushMouse,
    drain,
    onEvent,
  };
}
