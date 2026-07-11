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
  pushKey(key: string, modifiers: string, targetId: string): void;
  pushMouse(button: string, x: number, y: number, targetId: string): void;
  drain(): string[];
  onEvent(callback: EventCallback): void;
}

export function createEventLoop(eventBus: NapiEventBus): EventLoop {
  let isRunning = false;
  const callbacks: EventCallback[] = [];

  function start(): void {
    isRunning = true;
  }

  function stop(): void {
    isRunning = false;
  }

  function pushKey(key: string, modifiers: string, targetId: string): void {
    eventBus.pushKey(key, modifiers, targetId);
    for (const cb of callbacks) {
      cb({ key, ctrl: modifiers.includes("ctrl"), shift: modifiers.includes("shift"), alt: modifiers.includes("alt") });
    }
  }

  function pushMouse(button: string, x: number, y: number, targetId: string): void {
    eventBus.pushMouseButton(button, x, y, targetId);
    for (const cb of callbacks) {
      cb({ button: button as "left" | "right" | "middle", x, y });
    }
  }

  function drain(): string[] {
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
