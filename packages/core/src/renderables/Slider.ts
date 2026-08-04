/**
 * Slider — a horizontal or vertical slider widget.
 */

import { RenderableEvents, SliderEvents } from "../lib/renderableEvents";
import { type ColorInput, type RGBA, parseColor } from "../lib/rgba";
import type { CliRenderer } from "../platform/cliRenderer";
import type { RawKeyEvent } from "../platform/cliRenderer";
import { Box, type BoxOptions } from "./Box";

export interface SliderOptions extends BoxOptions {
  orientation?: "horizontal" | "vertical";
  min?: number;
  max?: number;
  value?: number;
  step?: number;
  viewPortSize?: number;
  trackColor?: ColorInput;
  thumbColor?: ColorInput;
  activeTrackColor?: ColorInput;
  onChange?: (value: number) => void;
}

export type SliderRenderableOptions = SliderOptions;

let _sliderCounter = 0;

export class Slider extends Box {
  private _orientation: "horizontal" | "vertical";
  private _min: number;
  private _max: number;
  private _value: number;
  private _step: number;
  private _viewPortSize: number;
  private _trackColor: RGBA;
  private _thumbColor: RGBA;
  private _activeTrackColor: RGBA;
  private _onChange: ((value: number) => void) | undefined;
  private _contentNodeId: number;
  private readonly _keyHandler: (key: RawKeyEvent) => void;

  constructor(renderer: CliRenderer, options: SliderOptions = {}) {
    _sliderCounter++;
    super(renderer, {
      ...options,
      id: options.id ?? `slider-${_sliderCounter}`,
      focusable: true,
    });

    this._orientation = options.orientation ?? "horizontal";
    this._min = options.min ?? 0;
    this._max = options.max ?? 100;
    this._value = Math.max(this._min, Math.min(this._max, options.value ?? this._min));
    this._step = options.step ?? 1;
    this._viewPortSize = options.viewPortSize ?? 1;
    this._trackColor = parseColor(options.trackColor ?? "#333333");
    this._thumbColor = parseColor(options.thumbColor ?? "#0088ff");
    this._activeTrackColor = parseColor(options.activeTrackColor ?? "#0055cc");
    this._onChange = options.onChange;

    this._contentNodeId = renderer.createNode("Text");
    renderer.appendChild(this._nodeId, this._contentNodeId);

    this._keyHandler = this._handleKey.bind(this);
    this._render();
  }

  get value(): number {
    return this._value;
  }

  set value(v: number) {
    const clamped = Math.max(this._min, Math.min(this._max, v));
    if (this._value !== clamped) {
      this._value = clamped;
      this._render();
      this._onChange?.(this._value);
      this.emit(SliderEvents.CHANGE, this._value);
    }
  }

  get min(): number {
    return this._min;
  }

  set min(v: number) {
    this._min = v;
    this._value = Math.max(v, this._value);
    this._render();
  }

  get max(): number {
    return this._max;
  }

  set max(v: number) {
    this._max = v;
    this._value = Math.min(v, this._value);
    this._render();
  }

  get step(): number {
    return this._step;
  }

  set step(v: number) {
    this._step = v;
  }

  get orientation(): "horizontal" | "vertical" {
    return this._orientation;
  }

  override focus(): void {
    if (this._isDestroyed || this._focused) return;
    this._focused = true;
    this._render();
    this.emit(RenderableEvents.FOCUSED, this);
    this._renderer.keyInput.off("keypress", this._keyHandler);
    this._renderer.keyInput.on("keypress", this._keyHandler);
  }

  override blur(): void {
    if (this._isDestroyed) return;
    this._renderer.keyInput.off("keypress", this._keyHandler);
    if (!this._focused) return;
    this._focused = false;
    this._render();
    this.emit(RenderableEvents.BLURRED, this);
  }

  private _handleKey(key: RawKeyEvent): void {
    if (!this._focused || this._isDestroyed) return;

    if (this._orientation === "horizontal") {
      if (key.name === "left") {
        this.value -= this._step;
      } else if (key.name === "right") {
        this.value += this._step;
      } else if (key.shift && key.name === "left") {
        this.value -= this._step * 10;
      } else if (key.shift && key.name === "right") {
        this.value += this._step * 10;
      }
    } else {
      if (key.name === "up") {
        this.value += this._step;
      } else if (key.name === "down") {
        this.value -= this._step;
      }
    }
  }

  private _render(): void {
    if (this._isDestroyed) return;

    const range = this._max - this._min;
    const progress = range === 0 ? 0 : (this._value - this._min) / range;

    if (this._orientation === "horizontal") {
      // Get width from options
      const width = typeof this._options.width === "number" ? this._options.width : 20;
      const trackWidth = Math.max(1, width - 2);
      const thumbPos = Math.round(progress * (trackWidth - 1));

      const tc = `${this._trackColor.r};${this._trackColor.g};${this._trackColor.b}`;
      const ac = `${this._activeTrackColor.r};${this._activeTrackColor.g};${this._activeTrackColor.b}`;
      const thumbC = `${this._thumbColor.r};${this._thumbColor.g};${this._thumbColor.b}`;

      let track = "";
      for (let i = 0; i < trackWidth; i++) {
        if (i === thumbPos) {
          track += `\x1b[38;2;${thumbC}m█\x1b[0m`;
        } else if (i < thumbPos) {
          track += `\x1b[38;2;${ac}m─\x1b[0m`;
        } else {
          track += `\x1b[38;2;${tc}m─\x1b[0m`;
        }
      }

      this._renderer.setText(this._contentNodeId, track);
    } else {
      // Vertical slider
      const height = typeof this._options.height === "number" ? this._options.height : 10;
      const trackHeight = Math.max(1, height - 2);
      const thumbPos = Math.round((1 - progress) * (trackHeight - 1));

      const tc = `${this._trackColor.r};${this._trackColor.g};${this._trackColor.b}`;
      const ac = `${this._activeTrackColor.r};${this._activeTrackColor.g};${this._activeTrackColor.b}`;
      const thumbC = `${this._thumbColor.r};${this._thumbColor.g};${this._thumbColor.b}`;

      const lines: string[] = [];
      for (let i = 0; i < trackHeight; i++) {
        if (i === thumbPos) {
          lines.push(`\x1b[38;2;${thumbC}m█\x1b[0m`);
        } else if (i > thumbPos) {
          lines.push(`\x1b[38;2;${ac}m│\x1b[0m`);
        } else {
          lines.push(`\x1b[38;2;${tc}m│\x1b[0m`);
        }
      }

      this._renderer.setText(this._contentNodeId, lines.join("\n"));
    }
  }

  override destroy(): void {
    if (this._isDestroyed) return;
    this._renderer.keyInput.off("keypress", this._keyHandler);
    try {
      this._renderer.removeNode(this._contentNodeId);
    } catch {
      // ignore
    }
    super.destroy();
  }
}

export { SliderEvents };
