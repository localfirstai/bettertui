/**
 * Central keyboard event handler with priority-based dispatch.
 * Global handlers run before renderable handlers for proper event propagation.
 */

import { EventEmitter } from "node:events";
import type { KeyEventType, ParsedKey } from "./parseKeypress";

type EventHandler = (...args: unknown[]) => void;

export class KeyEvent implements ParsedKey {
  name: string;
  ctrl: boolean;
  meta: boolean;
  shift: boolean;
  option: boolean;
  sequence: string;
  number: boolean;
  raw: string;
  eventType: KeyEventType;
  source: "raw" | "kitty";
  code?: string;
  super?: boolean;
  hyper?: boolean;
  capsLock?: boolean;
  numLock?: boolean;
  baseCode?: number;
  repeated?: boolean;

  private _defaultPrevented = false;
  private _propagationStopped = false;

  constructor(key: ParsedKey) {
    this.name = key.name;
    this.ctrl = key.ctrl;
    this.meta = key.meta;
    this.shift = key.shift;
    this.option = key.option;
    this.sequence = key.sequence;
    this.number = key.number;
    this.raw = key.raw;
    this.eventType = key.eventType;
    this.source = key.source;
    if (key.code !== undefined) this.code = key.code;
    if (key.super !== undefined) this.super = key.super;
    if (key.hyper !== undefined) this.hyper = key.hyper;
    if (key.capsLock !== undefined) this.capsLock = key.capsLock;
    if (key.numLock !== undefined) this.numLock = key.numLock;
    if (key.baseCode !== undefined) this.baseCode = key.baseCode;
    if (key.repeated !== undefined) this.repeated = key.repeated;
  }

  /** Alias for `option` — backward-compatible with RawKeyEvent.alt. */
  get alt(): boolean {
    return this.option;
  }

  get defaultPrevented(): boolean {
    return this._defaultPrevented;
  }

  get propagationStopped(): boolean {
    return this._propagationStopped;
  }

  preventDefault(): void {
    this._defaultPrevented = true;
  }

  stopPropagation(): void {
    this._propagationStopped = true;
  }
}

export class PasteEvent {
  type = "paste" as const;
  bytes: Uint8Array;
  /** Optional metadata attached to the paste event (e.g. bracketed-paste info). */
  metadata?: Record<string, unknown>;
  private _defaultPrevented = false;
  private _propagationStopped = false;

  constructor(bytes: Uint8Array, metadata?: Record<string, unknown>) {
    this.bytes = bytes;
    this.metadata = metadata;
  }

  get defaultPrevented(): boolean {
    return this._defaultPrevented;
  }

  get propagationStopped(): boolean {
    return this._propagationStopped;
  }

  preventDefault(): void {
    this._defaultPrevented = true;
  }

  stopPropagation(): void {
    this._propagationStopped = true;
  }
}

export type KeyHandlerEventMap = {
  keypress: [KeyEvent];
  keyrelease: [KeyEvent];
  paste: [PasteEvent];
};

export class KeyHandler extends EventEmitter<KeyHandlerEventMap> {
  public processParsedKey(parsedKey: ParsedKey): boolean {
    try {
      switch (parsedKey.eventType) {
        case "press":
          this.emit("keypress", new KeyEvent(parsedKey));
          break;
        case "release":
          this.emit("keyrelease", new KeyEvent(parsedKey));
          break;
        default:
          this.emit("keypress", new KeyEvent(parsedKey));
          break;
      }
    } catch (error) {
      console.error("[KeyHandler] Error processing parsed key:", error);
      return true;
    }

    return true;
  }

  public processPaste(bytes: Uint8Array): void {
    try {
      this.emit("paste", new PasteEvent(bytes));
    } catch (error) {
      console.error("[KeyHandler] Error processing paste:", error);
    }
  }
}

/**
 * This class is used internally by the renderer to ensure global handlers
 * can preventDefault before renderable handlers process events.
 */
export class InternalKeyHandler extends KeyHandler {
  private renderableHandlers: Map<keyof KeyHandlerEventMap, Set<EventHandler>> = new Map();

  public emitWithPriority<K extends keyof KeyHandlerEventMap>(
    event: K,
    ...args: KeyHandlerEventMap[K]
  ): boolean {
    let hasGlobalListeners = false;

    const globalListeners = this.listeners(event as never);
    if (globalListeners.length > 0) {
      hasGlobalListeners = true;

      for (const listener of globalListeners) {
        try {
          (listener as EventHandler)(...args);
        } catch (error) {
          console.error(`[KeyHandler] Error in global ${event} handler:`, error);
        }

        if (event === "keypress" || event === "keyrelease" || event === "paste") {
          const keyEvent = args[0];
          if (keyEvent.propagationStopped) {
            return hasGlobalListeners;
          }
        }
      }
    }

    const renderableSet = this.renderableHandlers.get(event);
    const renderableHandlers = renderableSet && renderableSet.size > 0 ? [...renderableSet] : [];
    let hasRenderableListeners = false;

    if (renderableSet && renderableSet.size > 0) {
      hasRenderableListeners = true;

      if (event === "keypress" || event === "keyrelease" || event === "paste") {
        const keyEvent = args[0];
        if (keyEvent.defaultPrevented) return hasGlobalListeners || hasRenderableListeners;
        if (keyEvent.propagationStopped) return hasGlobalListeners || hasRenderableListeners;
      }

      for (const handler of renderableHandlers) {
        try {
          (handler as EventHandler)(...args);
        } catch (error) {
          console.error(`[KeyHandler] Error in renderable ${event} handler:`, error);
        }

        if (event === "keypress" || event === "keyrelease" || event === "paste") {
          const keyEvent = args[0];
          if (keyEvent.propagationStopped) {
            return hasGlobalListeners || hasRenderableListeners;
          }
        }
      }
    }

    return hasGlobalListeners || hasRenderableListeners;
  }

  public onInternal<K extends keyof KeyHandlerEventMap>(
    event: K,
    handler: (...args: KeyHandlerEventMap[K]) => void,
  ): void {
    if (!this.renderableHandlers.has(event)) {
      this.renderableHandlers.set(event, new Set());
    }
    this.renderableHandlers.get(event)?.add(handler as EventHandler);
  }

  public offInternal<K extends keyof KeyHandlerEventMap>(
    event: K,
    handler: (...args: KeyHandlerEventMap[K]) => void,
  ): void {
    const handlers = this.renderableHandlers.get(event);
    if (handlers) {
      handlers.delete(handler as EventHandler);
    }
  }
}
