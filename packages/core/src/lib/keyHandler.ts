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

/**
 * Metadata attached to a {@link PasteEvent}. Used by consumers to decide
 * whether to insert, filter, or transform pasted content (e.g. skip binary
 * paste into a text field).
 */
export interface PasteMetadata {
  /** MIME type if the terminal reported one (e.g. `text/plain`). */
  mimeType?: string;
  /** Coarse kind of the pasted payload. */
  kind?: "text" | "binary" | "unknown";
}

export class PasteEvent {
  type = "paste" as const;
  bytes: Uint8Array;
  /** Optional metadata attached to the paste event (e.g. bracketed-paste info). */
  metadata?: PasteMetadata;
  private _defaultPrevented = false;
  private _propagationStopped = false;

  constructor(bytes: Uint8Array, metadata?: PasteMetadata) {
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

  public processPaste(bytes: Uint8Array, metadata?: PasteMetadata): void {
    try {
      this.emit("paste", new PasteEvent(bytes, metadata));
    } catch (error) {
      console.error("[KeyHandler] Error processing paste:", error);
    }
  }
}

/**
 * This class is used internally by the renderer to ensure global handlers
 * can preventDefault before renderable handlers process events.
 *
 * NOTE: `emit` is overridden to route every emission through `emitWithPriority`,
 * so that global listeners always run before renderable listeners and can
 * `preventDefault()` / `stopPropagation()` to short-circuit them. Previously
 * this override was missing, which meant `processParsedKey`'s direct
 * `this.emit("keypress", …)` bypassed priority dispatch entirely.
 */
export class InternalKeyHandler extends KeyHandler {
  private renderableHandlers: Map<keyof KeyHandlerEventMap, Set<EventHandler>> = new Map();

  /**
   * Override `emit` so that all emissions for the three domain event types go
   * through `emitWithPriority` (global listeners first, then renderable
   * listeners, with propagation / defaultPrevented checks in between).
   * Unknown event names fall through to the base `EventEmitter.emit` so that
   * Node's internal events (e.g. `newListener`, `removeListener`) are not
   * broken.
   */
  public emit(event: string | symbol, ...args: unknown[]): boolean {
    if (event === "keypress" || event === "keyrelease" || event === "paste") {
      return this.emitWithPriority(
        event as keyof KeyHandlerEventMap,
        // biome-ignore lint/suspicious/noExplicitAny: priority dispatch cast
        ...(args as any),
      );
    }
    return super.emit(event, ...args);
  }

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
