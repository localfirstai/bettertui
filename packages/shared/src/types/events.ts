/** Event type for keyboard events. */
export type KeyEventType = "press" | "repeat" | "release";

/** Source of a keyboard event. */
export type KeyEventSource = "raw" | "kitty";

/** Mouse button identifier. */
export type MouseButton = "left" | "right" | "middle" | "none";

/** A keyboard event from the terminal. */
export interface KeyEvent {
  /** The key value (e.g. "a", "Enter", "Escape") */
  key: string;
  /** Physical key code (e.g. "KeyA", "Enter") */
  code: string;
  /** Whether Ctrl was held */
  ctrl: boolean;
  /** Whether Shift was held */
  shift: boolean;
  /** Whether Alt was held */
  alt: boolean;
  /** Whether Meta (Cmd/Windows) was held */
  meta: boolean;
  /** Event type: press, repeat, or release */
  eventType: KeyEventType;
  /** Source of the event: raw terminal or Kitty keyboard protocol */
  source: KeyEventSource;
  /** Whether Super (Cmd/Windows) was held */
  super?: boolean;
  /** Whether Hyper was held */
  hyper?: boolean;
  /** Whether CapsLock was active */
  capsLock?: boolean;
  /** Whether NumLock was active */
  numLock?: boolean;
  /** Base layout codepoint for layout-independent shortcut matching */
  baseCode?: number;
  /** Whether this is a repeated keypress */
  repeated?: boolean;
}

/** A mouse event from the terminal. */
export interface MouseEvent {
  button: MouseButton;
  /** Terminal-grid position where the event occurred */
  position: { x: number; y: number };
  ctrl: boolean;
  shift: boolean;
  alt: boolean;
}
