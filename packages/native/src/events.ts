import type { NapiEventBus } from "./types.js";

export interface KeyEvent {
  key: string;
  ctrl: boolean;
  shift: boolean;
  alt: boolean;
}

export interface MouseEvent {
  button: "left" | "right" | "middle";
  x: number;
  y: number;
}

export type EventCallback = (event: KeyEvent | MouseEvent) => void;

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

  function start(): void {}

  function stop(): void {}

  function pushKey(
    key: string,
    ctrl: boolean,
    shift: boolean,
    alt: boolean,
    targetId: number,
  ): void {
    eventBus.pushKey(key, ctrl, shift, alt, targetId);
    for (const cb of callbacks) {
      cb({ key, ctrl, shift, alt });
    }
  }

  function pushMouse(button: string, x: number, y: number, targetId: number): void {
    eventBus.pushMouse(button, x, y, targetId);
    for (const cb of callbacks) {
      cb({ button: button as "left" | "right" | "middle", x, y });
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
