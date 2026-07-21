import type { KeyEvent as SharedKeyEvent, MouseEvent as SharedMouseEvent } from "@bettertui/shared";
import { SystemClock } from "../lib/clock";
import type { Clock, TimerHandle } from "../lib/clock";
import type { NapiEventBus } from "./types";

export type KeyEvent = SharedKeyEvent;
export type MouseEvent = SharedMouseEvent;

export type EventCallback = (event: SharedKeyEvent | SharedMouseEvent) => void;

export interface EventLoopOptions {
  pollIntervalMs?: number;
  clock?: Clock;
}

export interface EventLoop {
  start(): void;
  stop(): void;
  pushKey(key: string, ctrl: boolean, shift: boolean, alt: boolean): void;
  pushMouse(button: string, x: number, y: number): void;
  drain(): string;
  onEvent(callback: EventCallback): void;
}

export function createEventLoop(eventBus: NapiEventBus, options?: EventLoopOptions): EventLoop {
  const callbacks: EventCallback[] = [];
  let running = false;
  let drainInterval: TimerHandle | null = null;
  const clock = options?.clock ?? new SystemClock();
  const pollIntervalMs = options?.pollIntervalMs ?? 16;

  function start(): void {
    if (running) return;
    running = true;
    const poll = () => {
      if (!running) return;
      const raw = eventBus.drain();
      if (raw) {
        try {
          const events = JSON.parse(raw) as Array<SharedKeyEvent | SharedMouseEvent>;
          for (const event of events) {
            for (const cb of callbacks) {
              cb(event);
            }
          }
        } catch {}
      }
      drainInterval = clock.setTimeout(poll, pollIntervalMs);
    };
    poll();
  }

  function stop(): void {
    running = false;
    if (drainInterval !== null) {
      clock.clearTimeout(drainInterval);
      drainInterval = null;
    }
  }

  function pushKey(key: string, ctrl: boolean, shift: boolean, alt: boolean): void {
    const keyEvent: SharedKeyEvent = {
      key,
      code: "",
      ctrl,
      shift,
      alt,
      meta: false,
      eventType: "press",
      source: "raw",
    };
    eventBus.pushKey(key, ctrl, shift, alt);
    for (const cb of callbacks) {
      cb(keyEvent);
    }
  }

  function pushMouse(button: string, x: number, y: number): void {
    const mouseEvent: SharedMouseEvent = {
      button: button as "left" | "right" | "middle" | "none",
      position: { x, y },
      ctrl: false,
      shift: false,
      alt: false,
    };
    eventBus.pushMouse(button, x, y);
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
