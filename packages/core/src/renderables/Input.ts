import { InputRenderableEvents, RenderableEvents } from "../lib/renderableEvents";
import { type ColorInput, type RGBA, parseColor, rgbaToEngineColor } from "../lib/rgba";
import type { CliRenderer } from "../platform/cliRenderer";
import type { RawKeyEvent } from "../platform/cliRenderer";
import { type BoxOptions, BoxRenderable } from "./Box";

export interface InputRenderableOptions extends BoxOptions {
  value?: string;
  placeholder?: string;
  placeholderColor?: ColorInput;
  textColor?: ColorInput;
  focusedTextColor?: ColorInput;
  cursorColor?: ColorInput;
  backgroundColor?: ColorInput;
  focusedBackgroundColor?: ColorInput;
  maxLength?: number;
  minLength?: number;
  showCursor?: boolean;
  password?: boolean;
}

let _inputCounter = 0;

export class InputRenderable extends BoxRenderable {
  private _value: string;
  private _placeholder: string;
  private _placeholderColor: RGBA;
  private _textColor: RGBA;
  private _focusedTextColor: RGBA;
  private _cursorColor: RGBA;
  private _focusedBackgroundColor: RGBA | null = null;
  private _maxLength: number;
  private _minLength: number;
  private _showCursor: boolean;
  private _password: boolean;
  private _cursorPos: number;
  private _lastCommittedValue: string;
  private _textNodeId: number;
  private readonly _keyHandler: (key: RawKeyEvent) => void;

  constructor(renderer: CliRenderer, options: InputRenderableOptions = {}) {
    _inputCounter++;
    super(renderer, {
      ...options,
      id: options.id ?? `input-${_inputCounter}`,
      focusable: true,
    });

    this._value = (options.value ?? "").substring(0, options.maxLength ?? 1000);
    this._placeholder = options.placeholder ?? "";
    this._placeholderColor = parseColor(options.placeholderColor ?? "#666666");
    this._textColor = parseColor(options.textColor ?? "#ffffff");
    this._focusedTextColor = parseColor(options.focusedTextColor ?? "#ffffff");
    this._cursorColor = parseColor(options.cursorColor ?? "#ffff00");
    this._maxLength = options.maxLength ?? 1000;
    this._minLength = options.minLength ?? 0;
    this._showCursor = options.showCursor !== false;
    this._password = options.password ?? false;
    this._cursorPos = this._value.length;
    this._lastCommittedValue = this._value;

    if (options.focusedBackgroundColor) {
      this._focusedBackgroundColor = parseColor(options.focusedBackgroundColor);
    }

    this._textNodeId = renderer.createNode("Text");
    renderer.appendChild(this._nodeId, this._textNodeId);

    this._keyHandler = this._handleKey.bind(this);
    this._render();
  }

  // ── Getters/Setters ───────────────────────────────────────────────────────────

  get value(): string {
    return this._value;
  }

  set value(v: string) {
    const newVal = v.replace(/[\n\r]/g, "").substring(0, this._maxLength);
    if (this._value !== newVal) {
      this._value = newVal;
      this._cursorPos = Math.min(this._cursorPos, newVal.length);
      this._render();
      this.emit(InputRenderableEvents.INPUT, newVal);
    }
  }

  get plainText(): string {
    return this._value;
  }

  get cursorOffset(): number {
    return this._cursorPos;
  }

  set cursorOffset(pos: number) {
    this._cursorPos = Math.max(0, Math.min(pos, this._value.length));
    this._render();
  }

  set textColor(color: ColorInput) {
    this._textColor = parseColor(color);
    this._render();
  }

  set focusedTextColor(color: ColorInput) {
    this._focusedTextColor = parseColor(color);
    if (this._focused) this._render();
  }

  set placeholder(value: string) {
    this._placeholder = value;
    this._render();
  }

  set placeholderColor(color: ColorInput) {
    this._placeholderColor = parseColor(color);
    this._render();
  }

  set cursorColor(color: ColorInput) {
    this._cursorColor = parseColor(color);
    if (this._focused) this._render();
  }

  set showCursor(value: boolean) {
    this._showCursor = value;
    this._render();
  }

  // ── Focus ─────────────────────────────────────────────────────────────────────

  override focus(): void {
    if (this._isDestroyed) return;
    this._focused = true;
    this._lastCommittedValue = this._value;
    if (this._focusedBackgroundColor) {
      this._renderer.setNodeStyle(this._nodeId, {
        bg: rgbaToEngineColor(this._focusedBackgroundColor),
      });
    }
    this._render();
    this.emit(RenderableEvents.FOCUSED, this);
    this._renderer.keyInput.on("keypress", this._keyHandler);
  }

  override blur(): void {
    if (this._isDestroyed) return;
    this._renderer.keyInput.off("keypress", this._keyHandler);
    const current = this._value;
    if (current !== this._lastCommittedValue) {
      this._lastCommittedValue = current;
      this.emit(InputRenderableEvents.CHANGE, current);
    }
    this._focused = false;
    if (this._focusedBackgroundColor && this._backgroundColor) {
      this._renderer.setNodeStyle(this._nodeId, {
        bg: rgbaToEngineColor(this._backgroundColor),
      });
    } else if (this._focusedBackgroundColor) {
      this._renderer.setNodeStyle(this._nodeId, { bg: "transparent" });
    }
    this._render();
    this.emit(RenderableEvents.BLURRED, this);
  }

  // ── Key handling ──────────────────────────────────────────────────────────────

  private _handleKey(key: RawKeyEvent): void {
    if (!this._focused || this._isDestroyed) return;

    if (key.name === "return" || key.name === "linefeed") {
      this._submit();
      return;
    }

    if (key.name === "left") {
      this._cursorPos = Math.max(0, this._cursorPos - 1);
      this._render();
      return;
    }

    if (key.name === "right") {
      this._cursorPos = Math.min(this._value.length, this._cursorPos + 1);
      this._render();
      return;
    }

    if (key.name === "home" || (key.ctrl && key.name === "a")) {
      this._cursorPos = 0;
      this._render();
      return;
    }

    if (key.name === "end" || (key.ctrl && key.name === "e")) {
      this._cursorPos = this._value.length;
      this._render();
      return;
    }

    if (key.name === "backspace" || (key.name === "delete" && !key.ctrl)) {
      if (key.name === "backspace" && this._cursorPos > 0) {
        this._value =
          this._value.slice(0, this._cursorPos - 1) + this._value.slice(this._cursorPos);
        this._cursorPos--;
        this._render();
        this.emit(InputRenderableEvents.INPUT, this._value);
      } else if (key.name === "delete" && this._cursorPos < this._value.length) {
        this._value =
          this._value.slice(0, this._cursorPos) + this._value.slice(this._cursorPos + 1);
        this._render();
        this.emit(InputRenderableEvents.INPUT, this._value);
      }
      return;
    }

    // Ctrl+K: delete to end of line
    if (key.ctrl && key.name === "k") {
      this._value = this._value.slice(0, this._cursorPos);
      this._render();
      this.emit(InputRenderableEvents.INPUT, this._value);
      return;
    }

    // Ctrl+U: delete to start of line
    if (key.ctrl && key.name === "u") {
      this._value = this._value.slice(this._cursorPos);
      this._cursorPos = 0;
      this._render();
      this.emit(InputRenderableEvents.INPUT, this._value);
      return;
    }

    // Regular character input (ignore control sequences)
    if (key.sequence && !key.ctrl && !key.alt && !key.meta) {
      const char = key.sequence;
      if (char.length === 1 && char.charCodeAt(0) >= 32) {
        if (this._value.length < this._maxLength) {
          this._value =
            this._value.slice(0, this._cursorPos) + char + this._value.slice(this._cursorPos);
          this._cursorPos++;
          this._render();
          this.emit(InputRenderableEvents.INPUT, this._value);
        }
      }
    }
  }

  private _submit(): void {
    if (this._value.length < this._minLength) return;
    const current = this._value;
    if (current !== this._lastCommittedValue) {
      this._lastCommittedValue = current;
      this.emit(InputRenderableEvents.CHANGE, current);
    }
    this.emit(InputRenderableEvents.ENTER, current);
  }

  private _render(): void {
    if (this._isDestroyed) return;

    let display: string;

    if (this._value === "" && !this._focused) {
      // Show placeholder
      const ph = this._placeholder;
      display = `\x1b[38;2;${this._placeholderColor.r};${this._placeholderColor.g};${this._placeholderColor.b}m${ph}\x1b[0m`;
    } else {
      const displayValue = this._password ? "•".repeat(this._value.length) : this._value;
      const textColor = this._focused ? this._focusedTextColor : this._textColor;

      if (this._focused && this._showCursor) {
        const before = displayValue.slice(0, this._cursorPos);
        const cursorChar = displayValue[this._cursorPos] ?? " ";
        const after = displayValue.slice(this._cursorPos + 1);
        const tc = `${textColor.r};${textColor.g};${textColor.b}`;
        const cc = `${this._cursorColor.r};${this._cursorColor.g};${this._cursorColor.b}`;
        display =
          `\x1b[38;2;${tc}m${before}` +
          `\x1b[38;2;${cc}m\x1b[7m${cursorChar}\x1b[0m` +
          `\x1b[38;2;${tc}m${after}\x1b[0m`;
      } else {
        const tc = `${textColor.r};${textColor.g};${textColor.b}`;
        display = `\x1b[38;2;${tc}m${displayValue}\x1b[0m`;
      }
    }

    this._renderer.setText(this._textNodeId, display);
  }

  override destroy(): void {
    if (this._isDestroyed) return;
    this._renderer.keyInput.off("keypress", this._keyHandler);
    try {
      this._renderer.removeNode(this._textNodeId);
    } catch {
      // ignore
    }
    super.destroy();
  }
}

export { InputRenderableEvents };
