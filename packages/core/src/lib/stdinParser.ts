// Byte-level stdin parser that turns raw terminal input into typed StdinEvents.
//
// This replaces a two-phase token -> decode pipeline with a single state machine
// that produces fully typed events (key, mouse, paste, response) directly from
// bytes. The parser owns all byte framing and protocol recognition. It does NOT
// own event dispatch — that belongs to KeyHandler and the renderer.

import { Buffer } from "node:buffer";
import { type Clock, SystemClock, type TimerHandle } from "./clock";
import { parseKeypress } from "./parseKeypress";
import type { ParsedKey } from "./parseKeypress";
import { MouseParser, type RawMouseEvent } from "./parseMouse";

export { SystemClock, type Clock, type TimerHandle } from "./clock";

export type StdinResponseProtocol = "csi" | "cpr" | "osc" | "dcs" | "apc" | "unknown";

export type PasteMetadata = Record<string, never>;

// The four event types the parser produces. Everything stdin sends becomes
// exactly one of these.
export type StdinEvent =
  | {
      type: "key";
      raw: string;
      key: ParsedKey;
    }
  | {
      type: "mouse";
      raw: string;
      encoding: "sgr" | "x10";
      event: RawMouseEvent;
    }
  | {
      type: "paste";
      bytes: Uint8Array;
      metadata?: PasteMetadata;
    }
  | {
      type: "response";
      protocol: StdinResponseProtocol;
      sequence: string;
    };

export interface StdinParserProtocolContext {
  kittyKeyboardEnabled: boolean;
  privateCapabilityRepliesActive: boolean;
  pixelResolutionQueryActive: boolean;
  explicitWidthCprActive: boolean;
  startupCursorCprActive: boolean;
}

export interface StdinParserOptions {
  timeoutMs?: number;
  maxPendingBytes?: number;
  armTimeouts?: boolean;
  onTimeoutFlush?: () => void;
  useKittyKeyboard?: boolean;
  protocolContext?: Partial<StdinParserProtocolContext>;
  clock?: Clock;
}

// State machine tags for the byte scanner.
type ParserState =
  | { tag: "ground" }
  | { tag: "utf8"; expected: number; seen: number }
  | { tag: "esc" }
  | { tag: "ss3" }
  | { tag: "csi" }
  | { tag: "csi_sgr_mouse"; part: number; hasDigit: boolean }
  | { tag: "csi_sgr_mouse_deferred"; part: number; hasDigit: boolean }
  | {
      tag: "csi_parametric";
      semicolons: number;
      segments: number;
      hasDigit: boolean;
      firstParamValue: number | null;
    }
  | {
      tag: "csi_parametric_deferred";
      semicolons: number;
      segments: number;
      hasDigit: boolean;
      firstParamValue: number | null;
    }
  | {
      tag: "csi_parametric_ignored";
      semicolons: number;
      segments: number;
      hasDigit: boolean;
      firstParamValue: number | null;
    }
  | { tag: "csi_private_reply"; semicolons: number; hasDigit: boolean; sawDollar: boolean }
  | { tag: "csi_private_reply_deferred"; semicolons: number; hasDigit: boolean; sawDollar: boolean }
  | { tag: "osc"; sawEsc: boolean }
  | { tag: "dcs"; sawEsc: boolean }
  | { tag: "apc"; sawEsc: boolean }
  | { tag: "esc_recovery" }
  | { tag: "esc_less_mouse" }
  | { tag: "esc_less_x10_mouse" };

interface PasteCollector {
  tail: Uint8Array;
  parts: Uint8Array[];
  totalLength: number;
}

const DEFAULT_TIMEOUT_MS = 20;
const DEFAULT_MAX_PENDING_BYTES = 64 * 1024;
const INITIAL_PENDING_CAPACITY = 256;
const ESC = 0x1b;
const BEL = 0x07;
const BRACKETED_PASTE_START = Buffer.from("\x1b[200~");
const BRACKETED_PASTE_END = Buffer.from("\x1b[201~");
const EMPTY_BYTES = new Uint8Array(0);
const KEY_DECODER = new TextDecoder();
const DEFAULT_PROTOCOL_CONTEXT: StdinParserProtocolContext = {
  kittyKeyboardEnabled: false,
  privateCapabilityRepliesActive: false,
  pixelResolutionQueryActive: false,
  explicitWidthCprActive: false,
  startupCursorCprActive: false,
};

const RXVT_DOLLAR_CSI_RE = new RegExp(`^${ESC}\\[\\d+\\$$`);
const SYSTEM_CLOCK = new SystemClock();

class ByteQueue {
  private buf: Uint8Array;
  private start = 0;
  private end = 0;

  constructor(capacity = INITIAL_PENDING_CAPACITY) {
    this.buf = new Uint8Array(capacity);
  }

  get length(): number {
    return this.end - this.start;
  }

  get capacity(): number {
    return this.buf.length;
  }

  view(): Uint8Array {
    return this.buf.subarray(this.start, this.end);
  }

  take(): Uint8Array {
    const chunk = this.view();
    this.start = 0;
    this.end = 0;
    return chunk;
  }

  append(chunk: Uint8Array): void {
    if (chunk.length === 0) {
      return;
    }

    this.ensureCapacity(this.length + chunk.length);
    this.buf.set(chunk, this.end);
    this.end += chunk.length;
  }

  consume(count: number): void {
    if (count <= 0) {
      return;
    }

    if (count >= this.length) {
      this.start = 0;
      this.end = 0;
      return;
    }

    this.start += count;
    if (this.start >= this.buf.length / 2) {
      this.buf.copyWithin(0, this.start, this.end);
      this.end -= this.start;
      this.start = 0;
    }
  }

  clear(): void {
    this.start = 0;
    this.end = 0;
  }

  reset(capacity = INITIAL_PENDING_CAPACITY): void {
    this.buf = new Uint8Array(capacity);
    this.start = 0;
    this.end = 0;
  }

  private ensureCapacity(requiredLength: number): void {
    const currentLength = this.length;
    if (requiredLength <= this.buf.length) {
      const availableAtEnd = this.buf.length - this.end;
      if (availableAtEnd >= requiredLength - currentLength) {
        return;
      }

      this.buf.copyWithin(0, this.start, this.end);
      this.end = currentLength;
      this.start = 0;
      if (requiredLength <= this.buf.length) {
        return;
      }
    }

    let nextCapacity = this.buf.length;
    while (nextCapacity < requiredLength) {
      nextCapacity *= 2;
    }

    const next = new Uint8Array(nextCapacity);
    next.set(this.view(), 0);
    this.buf = next;
    this.start = 0;
    this.end = currentLength;
  }
}

function normalizePositiveOption(value: number | undefined, fallback: number): number {
  if (typeof value !== "number" || !Number.isFinite(value) || value <= 0) {
    return fallback;
  }
  return Math.floor(value);
}

function utf8SequenceLength(first: number): number {
  if (first < 0x80) return 1;
  if (first >= 0xc2 && first <= 0xdf) return 2;
  if (first >= 0xe0 && first <= 0xef) return 3;
  if (first >= 0xf0 && first <= 0xf4) return 4;
  return 0;
}

function bytesEqual(left: Uint8Array, right: Uint8Array): boolean {
  if (left.length !== right.length) return false;
  for (let index = 0; index < left.length; index += 1) {
    if (left[index] !== right[index]) return false;
  }
  return true;
}

function isMouseSgrSequence(sequence: Uint8Array): boolean {
  if (sequence.length < 7) return false;
  if (sequence[0] !== ESC || sequence[1] !== 0x5b || sequence[2] !== 0x3c) return false;

  const final = sequence[sequence.length - 1];
  if (final !== 0x4d && final !== 0x6d) return false;

  let part = 0;
  let hasDigit = false;
  for (let index = 3; index < sequence.length - 1; index += 1) {
    const byte = sequence[index];
    if (byte === undefined) return false;

    if (byte >= 0x30 && byte <= 0x39) {
      hasDigit = true;
      continue;
    }
    if (byte === 0x3b && hasDigit && part < 2) {
      part += 1;
      hasDigit = false;
      continue;
    }
    return false;
  }
  return part === 2 && hasDigit;
}

function isAsciiDigit(byte: number): boolean {
  return byte >= 0x30 && byte <= 0x39;
}

interface ParametricCsiLike {
  semicolons: number;
  segments: number;
  hasDigit: boolean;
  firstParamValue: number | null;
}

interface PrivateReplyCsiLike {
  semicolons: number;
  hasDigit: boolean;
  sawDollar: boolean;
}

function parsePositiveDecimalPrefix(
  sequence: Uint8Array,
  start: number,
  endExclusive: number,
): number | null {
  if (start >= endExclusive) return null;

  let value = 0;
  let sawDigit = false;
  for (let index = start; index < endExclusive; index += 1) {
    const byte = sequence[index];
    if (byte === undefined || !isAsciiDigit(byte)) return null;
    sawDigit = true;
    value = value * 10 + (byte - 0x30);
  }

  return sawDigit ? value : null;
}

function parseKittyFirstFieldCodepoint(
  sequence: Uint8Array,
  start: number,
  endExclusive: number,
): number | null {
  if (start >= endExclusive) return null;

  let firstColon = -1;
  for (let index = start; index < endExclusive; index += 1) {
    if (sequence[index] === 0x3a) {
      firstColon = index;
      break;
    }
  }

  if (firstColon === -1) return null;

  const codepoint = parsePositiveDecimalPrefix(sequence, start, firstColon);
  if (codepoint === null) return null;

  for (let index = firstColon + 1; index < endExclusive; index += 1) {
    const byte = sequence[index];
    if (byte !== 0x3a && byte !== undefined && !isAsciiDigit(byte)) return null;
  }

  return codepoint;
}

function canStillBeKittyU(state: ParametricCsiLike): boolean {
  return state.semicolons >= 1;
}

function canStillBeKittySpecial(state: ParametricCsiLike): boolean {
  return state.semicolons === 1 && state.segments > 1;
}

function canStillBeExplicitWidthCpr(state: ParametricCsiLike): boolean {
  return state.firstParamValue === 1 && state.semicolons === 1;
}

function canStillBeStartupCursorCpr(state: ParametricCsiLike): boolean {
  return state.semicolons === 1;
}

function canStillBeStartupCursorCprPrefix(state: ParametricCsiLike): boolean {
  return state.segments === 1 && state.semicolons <= 1;
}

function canStillBePixelResolution(state: ParametricCsiLike): boolean {
  return state.firstParamValue === 4 && state.semicolons === 2;
}

function canDeferParametricCsi(
  state: ParametricCsiLike,
  context: StdinParserProtocolContext,
): boolean {
  return (
    (context.kittyKeyboardEnabled && (canStillBeKittyU(state) || canStillBeKittySpecial(state))) ||
    (context.explicitWidthCprActive && canStillBeExplicitWidthCpr(state)) ||
    (context.startupCursorCprActive && canStillBeStartupCursorCpr(state)) ||
    (context.pixelResolutionQueryActive && canStillBePixelResolution(state))
  );
}

function canCompleteDeferredParametricCsi(
  state: ParametricCsiLike,
  byte: number,
  context: StdinParserProtocolContext,
): boolean {
  if (context.kittyKeyboardEnabled) {
    if (state.hasDigit && byte === 0x75) return true;
    if (
      state.hasDigit &&
      state.semicolons === 1 &&
      state.segments > 1 &&
      (byte === 0x7e || (byte >= 0x41 && byte <= 0x5a))
    ) {
      return true;
    }
  }

  if (
    context.explicitWidthCprActive &&
    state.hasDigit &&
    state.firstParamValue === 1 &&
    state.semicolons === 1 &&
    byte === 0x52
  ) {
    return true;
  }

  if (context.startupCursorCprActive && state.hasDigit && state.semicolons === 1 && byte === 0x52) {
    return true;
  }

  if (
    context.pixelResolutionQueryActive &&
    state.hasDigit &&
    state.firstParamValue === 4 &&
    state.semicolons === 2 &&
    byte === 0x74
  ) {
    return true;
  }

  return false;
}

function classifyParametricCsiProtocol(
  state: ParametricCsiLike,
  finalByte: number,
): StdinResponseProtocol {
  if (finalByte === 0x52 && state.semicolons === 1 && state.segments === 1 && state.hasDigit) {
    return "cpr";
  }
  return "csi";
}

function canDeferPrivateReplyCsi(context: StdinParserProtocolContext): boolean {
  return context.privateCapabilityRepliesActive;
}

function canCompleteDeferredPrivateReplyCsi(
  state: PrivateReplyCsiLike,
  byte: number,
  context: StdinParserProtocolContext,
): boolean {
  if (!context.privateCapabilityRepliesActive) return false;
  if (state.sawDollar) return state.hasDigit && byte === 0x79;
  if (byte === 0x63) return state.hasDigit || state.semicolons > 0;
  if (byte === 0x6e) return state.hasDigit;
  return state.hasDigit && byte === 0x75;
}

function withEscPrefix(bytes: Uint8Array): Uint8Array {
  const prefixed = new Uint8Array(bytes.length + 1);
  prefixed[0] = ESC;
  prefixed.set(bytes, 1);
  return prefixed;
}

function indexOfBytes(haystack: Uint8Array, needle: Uint8Array): number {
  if (needle.length === 0) return 0;
  const limit = haystack.length - needle.length;
  for (let offset = 0; offset <= limit; offset += 1) {
    let matched = true;
    for (let index = 0; index < needle.length; index += 1) {
      if (haystack[offset + index] !== needle[index]) {
        matched = false;
        break;
      }
    }
    if (matched) return offset;
  }
  return -1;
}

function decodeLatin1(bytes: Uint8Array): string {
  return Buffer.from(bytes.buffer, bytes.byteOffset, bytes.byteLength).toString("latin1");
}

function decodeUtf8(bytes: Uint8Array): string {
  return KEY_DECODER.decode(bytes);
}

function createPasteCollector(): PasteCollector {
  return {
    tail: EMPTY_BYTES,
    parts: [],
    totalLength: 0,
  };
}

function joinPasteBytes(parts: Uint8Array[], totalLength: number): Uint8Array {
  if (totalLength === 0) return EMPTY_BYTES;
  if (parts.length === 1 && parts[0]) return parts[0];
  const bytes = new Uint8Array(totalLength);
  let offset = 0;
  for (const part of parts) {
    bytes.set(part, offset);
    offset += part.length;
  }
  return bytes;
}

export class StdinParser {
  private readonly pending = new ByteQueue(INITIAL_PENDING_CAPACITY);
  private readonly events: StdinEvent[] = [];
  private readonly timeoutMs: number;
  private readonly maxPendingBytes: number;
  private readonly armTimeouts: boolean;
  private readonly onTimeoutFlush: (() => void) | null;
  private readonly useKittyKeyboard: boolean;
  private readonly mouseParser = new MouseParser();
  private readonly clock: Clock;
  private protocolContext: StdinParserProtocolContext;
  private timeoutId: TimerHandle | null = null;
  private destroyed = false;
  private pendingSinceMs: number | null = null;
  private forceFlush = false;
  private justFlushedEsc = false;
  private state: ParserState = { tag: "ground" };
  private cursor = 0;
  private unitStart = 0;
  private paste: PasteCollector | null = null;

  constructor(options: StdinParserOptions = {}) {
    this.timeoutMs = normalizePositiveOption(options.timeoutMs, DEFAULT_TIMEOUT_MS);
    this.maxPendingBytes = normalizePositiveOption(
      options.maxPendingBytes,
      DEFAULT_MAX_PENDING_BYTES,
    );
    this.armTimeouts = options.armTimeouts ?? true;
    this.onTimeoutFlush = options.onTimeoutFlush ?? null;
    this.useKittyKeyboard = options.useKittyKeyboard ?? true;
    this.clock = options.clock ?? SYSTEM_CLOCK;
    this.protocolContext = {
      ...DEFAULT_PROTOCOL_CONTEXT,
      kittyKeyboardEnabled: options.protocolContext?.kittyKeyboardEnabled ?? false,
      privateCapabilityRepliesActive:
        options.protocolContext?.privateCapabilityRepliesActive ?? false,
      pixelResolutionQueryActive: options.protocolContext?.pixelResolutionQueryActive ?? false,
      explicitWidthCprActive: options.protocolContext?.explicitWidthCprActive ?? false,
      startupCursorCprActive: options.protocolContext?.startupCursorCprActive ?? false,
    };
  }

  public get bufferCapacity(): number {
    return this.pending.capacity;
  }

  public updateProtocolContext(patch: Partial<StdinParserProtocolContext>): void {
    this.ensureAlive();
    this.protocolContext = { ...this.protocolContext, ...patch };
    this.reconcileDeferredStateWithProtocolContext();
    this.reconcileTimeoutState();
  }

  private getAbortableStartupCursorCprState(): Extract<
    ParserState,
    { tag: "csi_parametric_ignored" }
  > | null {
    if (this.pending.length === 0) return null;

    switch (this.state.tag) {
      case "csi": {
        const bytes = this.pending.view();
        const firstParamStart = this.unitStart + 2;
        if (this.cursor < firstParamStart) return null;

        let firstParamValue: number | null = null;
        for (let index = firstParamStart; index < this.cursor; index += 1) {
          const byte = bytes[index];
          if (byte === undefined || !isAsciiDigit(byte)) return null;
          firstParamValue = (firstParamValue ?? 0) * 10 + (byte - 0x30);
        }

        return {
          tag: "csi_parametric_ignored",
          semicolons: 0,
          segments: 1,
          hasDigit: this.cursor > firstParamStart,
          firstParamValue,
        };
      }
      case "csi_parametric":
      case "csi_parametric_deferred":
        if (
          !canStillBeStartupCursorCprPrefix(this.state) ||
          (this.protocolContext.explicitWidthCprActive && canStillBeExplicitWidthCpr(this.state))
        ) {
          return null;
        }
        return {
          tag: "csi_parametric_ignored",
          semicolons: this.state.semicolons,
          segments: this.state.segments,
          hasDigit: this.state.hasDigit,
          firstParamValue: this.state.firstParamValue,
        };
    }
    return null;
  }

  public abortPendingStartupCursorCpr(): void {
    this.ensureAlive();
    const nextState = this.getAbortableStartupCursorCprState();
    if (!nextState) return;

    this.state = nextState;
    if (this.pendingSinceMs === null) {
      this.markPending();
    }
    this.forceFlush = false;
    this.reconcileTimeoutState();
  }

  public push(data: Uint8Array): void {
    this.ensureAlive();
    if (data.length === 0) {
      this.emitKeyOrResponse("unknown", "");
      return;
    }

    let remainder = data;
    while (remainder.length > 0) {
      if (this.paste) {
        remainder = this.consumePasteBytes(remainder);
        continue;
      }

      const immediatePasteStartIndex =
        this.state.tag === "ground" && this.pending.length === 0
          ? indexOfBytes(remainder, BRACKETED_PASTE_START)
          : -1;
      const appendEnd =
        immediatePasteStartIndex === -1
          ? remainder.length
          : immediatePasteStartIndex + BRACKETED_PASTE_START.length;

      this.pending.append(remainder.subarray(0, appendEnd));
      remainder = remainder.subarray(appendEnd);
      this.scanPending();

      if (this.paste && this.pending.length > 0) {
        remainder = this.consumePasteBytes(this.takePendingBytes());
        continue;
      }

      if (!this.paste && this.pending.length > this.maxPendingBytes) {
        this.flushPendingOverflow();
        this.scanPending();

        if (this.paste && this.pending.length > 0) {
          remainder = this.consumePasteBytes(this.takePendingBytes());
        }
      }
    }
    this.reconcileTimeoutState();
  }

  public read(): StdinEvent | null {
    this.ensureAlive();
    if (this.events.length === 0 && this.forceFlush) {
      this.scanPending();
      this.reconcileTimeoutState();
    }
    return this.events.shift() ?? null;
  }

  public drain(onEvent: (event: StdinEvent) => void): void {
    this.ensureAlive();
    while (true) {
      if (this.destroyed) return;
      const event = this.read();
      if (!event) return;
      onEvent(event);
    }
  }

  public flushTimeout(nowMsValue: number = this.clock.now()): void {
    this.ensureAlive();
    if (
      this.pendingSinceMs !== null &&
      (nowMsValue < this.pendingSinceMs || nowMsValue - this.pendingSinceMs < this.timeoutMs)
    ) {
      return;
    }
    this.tryForceFlush();
  }

  private tryForceFlush(): void {
    if (this.paste || this.pendingSinceMs === null || this.pending.length === 0) return;
    this.forceFlush = true;
  }

  public reset(): void {
    if (this.destroyed) return;
    this.clearTimeout();
    this.resetState();
  }

  public resetMouseState(): void {
    this.ensureAlive();
    this.mouseParser.reset();
  }

  public destroy(): void {
    if (this.destroyed) return;
    this.clearTimeout();
    this.destroyed = true;
    this.resetState();
  }

  private ensureAlive(): void {
    if (this.destroyed) throw new Error("StdinParser has been destroyed");
  }

  private scanPending(): void {
    while (!this.paste) {
      const bytes = this.pending.view();
      if (this.state.tag === "ground" && this.cursor >= bytes.length) {
        this.pending.clear();
        this.cursor = 0;
        this.unitStart = 0;
        this.pendingSinceMs = null;
        this.forceFlush = false;
        return;
      }

      const byte = this.cursor < bytes.length ? (bytes[this.cursor] ?? -1) : -1;
      switch (this.state.tag) {
        case "ground": {
          this.unitStart = this.cursor;

          if (this.justFlushedEsc) {
            if (byte === 0x5b) {
              this.justFlushedEsc = false;
              this.cursor += 1;
              this.state = { tag: "esc_recovery" };
              continue;
            }
            this.justFlushedEsc = false;
          }

          if (byte === ESC) {
            this.cursor += 1;
            this.state = { tag: "esc" };
            continue;
          }

          if (byte < 0x80) {
            this.emitKeyOrResponse(
              "unknown",
              decodeUtf8(bytes.subarray(this.cursor, this.cursor + 1)),
            );
            this.consumePrefix(this.cursor + 1);
            continue;
          }

          const expected = utf8SequenceLength(byte);
          if (expected === 0) {
            if (!this.forceFlush && this.cursor + 1 === bytes.length) {
              this.markPending();
              return;
            }
            this.emitLegacyHighByte(byte);
            this.consumePrefix(this.cursor + 1);
            continue;
          }

          this.cursor += 1;
          this.state = { tag: "utf8", expected, seen: 1 };
          continue;
        }

        case "utf8": {
          if (this.cursor >= bytes.length) {
            if (!this.forceFlush) {
              this.markPending();
              return;
            }
            this.emitLegacyHighByte(bytes[this.unitStart] ?? 0);
            this.state = { tag: "ground" };
            this.consumePrefix(this.unitStart + 1);
            continue;
          }

          if ((byte & 0xc0) !== 0x80) {
            this.emitLegacyHighByte(bytes[this.unitStart] ?? 0);
            this.state = { tag: "ground" };
            this.consumePrefix(this.unitStart + 1);
            continue;
          }

          const nextSeen = this.state.seen + 1;
          this.cursor += 1;
          if (nextSeen < this.state.expected) {
            this.state = { tag: "utf8", expected: this.state.expected, seen: nextSeen };
            continue;
          }

          this.emitKeyOrResponse(
            "unknown",
            decodeUtf8(bytes.subarray(this.unitStart, this.cursor)),
          );
          this.state = { tag: "ground" };
          this.consumePrefix(this.cursor);
          continue;
        }

        case "esc": {
          if (this.cursor >= bytes.length) {
            if (!this.forceFlush) {
              this.markPending();
              return;
            }
            const flushedLoneEsc =
              this.cursor === this.unitStart + 1 && bytes[this.unitStart] === ESC;
            this.emitKeyOrResponse(
              "unknown",
              decodeUtf8(bytes.subarray(this.unitStart, this.cursor)),
            );
            this.justFlushedEsc = flushedLoneEsc;
            this.state = { tag: "ground" };
            this.consumePrefix(this.cursor);
            continue;
          }

          switch (byte) {
            case 0x5b:
              this.cursor += 1;
              this.state = { tag: "csi" };
              continue;
            case 0x4f:
              this.cursor += 1;
              this.state = { tag: "ss3" };
              continue;
            case 0x5d:
              this.cursor += 1;
              this.state = { tag: "osc", sawEsc: false };
              continue;
            case 0x50:
              this.cursor += 1;
              this.state = { tag: "dcs", sawEsc: false };
              continue;
            case 0x5f:
              this.cursor += 1;
              this.state = { tag: "apc", sawEsc: false };
              continue;
            case ESC:
              this.cursor += 1;
              continue;
            default:
              this.cursor += 1;
              this.emitKeyOrResponse(
                "unknown",
                decodeUtf8(bytes.subarray(this.unitStart, this.cursor)),
              );
              this.state = { tag: "ground" };
              this.consumePrefix(this.cursor);
              continue;
          }
        }

        case "ss3": {
          if (this.cursor >= bytes.length) {
            if (!this.forceFlush) {
              this.markPending();
              return;
            }
            this.emitOpaqueResponse("unknown", bytes.subarray(this.unitStart, this.cursor));
            this.state = { tag: "ground" };
            this.consumePrefix(this.cursor);
            continue;
          }

          if (byte === ESC) {
            this.emitOpaqueResponse("unknown", bytes.subarray(this.unitStart, this.cursor));
            this.state = { tag: "ground" };
            this.consumePrefix(this.cursor);
            continue;
          }

          this.cursor += 1;
          this.emitKeyOrResponse(
            "unknown",
            decodeUtf8(bytes.subarray(this.unitStart, this.cursor)),
          );
          this.state = { tag: "ground" };
          this.consumePrefix(this.cursor);
          continue;
        }

        case "esc_recovery": {
          if (this.cursor >= bytes.length) {
            if (!this.forceFlush) {
              this.markPending();
              return;
            }
            this.emitKeyOrResponse(
              "unknown",
              decodeUtf8(bytes.subarray(this.unitStart, this.cursor)),
            );
            this.state = { tag: "ground" };
            this.consumePrefix(this.cursor);
            continue;
          }

          if (byte === 0x3c) {
            this.cursor += 1;
            this.state = { tag: "esc_less_mouse" };
            continue;
          }

          if (byte === 0x4d) {
            this.cursor += 1;
            this.state = { tag: "esc_less_x10_mouse" };
            continue;
          }

          this.emitKeyOrResponse(
            "unknown",
            decodeUtf8(bytes.subarray(this.unitStart, this.unitStart + 1)),
          );
          this.state = { tag: "ground" };
          this.consumePrefix(this.unitStart + 1);
          continue;
        }

        case "csi": {
          if (this.cursor >= bytes.length) {
            if (!this.forceFlush) {
              this.markPending();
              return;
            }
            this.emitOpaqueResponse("unknown", bytes.subarray(this.unitStart, this.cursor));
            this.state = { tag: "ground" };
            this.consumePrefix(this.cursor);
            continue;
          }

          if (byte === ESC) {
            this.emitOpaqueResponse("unknown", bytes.subarray(this.unitStart, this.cursor));
            this.state = { tag: "ground" };
            this.consumePrefix(this.cursor);
            continue;
          }

          if (byte === 0x4d && this.cursor === this.unitStart + 2) {
            const end = this.cursor + 4;
            if (bytes.length < end) {
              if (!this.forceFlush) {
                this.markPending();
                return;
              }
              this.emitOpaqueResponse("unknown", bytes.subarray(this.unitStart, bytes.length));
              this.state = { tag: "ground" };
              this.consumePrefix(bytes.length);
              continue;
            }
            this.emitMouse(bytes.subarray(this.unitStart, end), "x10");
            this.state = { tag: "ground" };
            this.consumePrefix(end);
            continue;
          }

          if (byte === 0x24) {
            const candidateEnd = this.cursor + 1;
            const candidate = decodeUtf8(bytes.subarray(this.unitStart, candidateEnd));
            if (RXVT_DOLLAR_CSI_RE.test(candidate)) {
              this.emitKeyOrResponse("csi", candidate);
              this.state = { tag: "ground" };
              this.consumePrefix(candidateEnd);
              continue;
            }
            if (!this.forceFlush && candidateEnd >= bytes.length) {
              this.markPending();
              return;
            }
          }

          if (byte === 0x3c && this.cursor === this.unitStart + 2) {
            this.cursor += 1;
            this.state = { tag: "csi_sgr_mouse", part: 0, hasDigit: false };
            continue;
          }

          if (byte === 0x5b && this.cursor === this.unitStart + 2) {
            this.cursor += 1;
            continue;
          }

          if (byte === 0x3f && this.cursor === this.unitStart + 2) {
            this.cursor += 1;
            this.state = {
              tag: "csi_private_reply",
              semicolons: 0,
              hasDigit: false,
              sawDollar: false,
            };
            continue;
          }

          if (byte === 0x3b) {
            const firstParamStart = this.unitStart + 2;
            const firstParamEnd = this.cursor;
            let firstParamValue = parsePositiveDecimalPrefix(bytes, firstParamStart, firstParamEnd);

            if (firstParamValue === null && this.protocolContext.kittyKeyboardEnabled) {
              firstParamValue = parseKittyFirstFieldCodepoint(
                bytes,
                firstParamStart,
                firstParamEnd,
              );
            }

            if (firstParamValue !== null) {
              this.cursor += 1;
              this.state = {
                tag: "csi_parametric",
                semicolons: 1,
                segments: 1,
                hasDigit: false,
                firstParamValue,
              };
              continue;
            }
          }

          if (byte >= 0x40 && byte <= 0x7e) {
            const end = this.cursor + 1;
            const rawBytes = bytes.subarray(this.unitStart, end);

            if (bytesEqual(rawBytes, BRACKETED_PASTE_START)) {
              this.state = { tag: "ground" };
              this.consumePrefix(end);
              this.paste = createPasteCollector();
              continue;
            }

            if (isMouseSgrSequence(rawBytes)) {
              this.emitMouse(rawBytes, "sgr");
              this.state = { tag: "ground" };
              this.consumePrefix(end);
              continue;
            }

            this.emitKeyOrResponse("csi", decodeUtf8(rawBytes));
            this.state = { tag: "ground" };
            this.consumePrefix(end);
            continue;
          }

          this.cursor += 1;
          continue;
        }

        case "csi_sgr_mouse": {
          if (this.cursor >= bytes.length) {
            if (!this.forceFlush) {
              this.markPending();
              return;
            }
            this.state = {
              tag: "csi_sgr_mouse_deferred",
              part: this.state.part,
              hasDigit: this.state.hasDigit,
            };
            this.pendingSinceMs = null;
            this.forceFlush = false;
            return;
          }

          if (byte === ESC) {
            this.emitOpaqueResponse("unknown", bytes.subarray(this.unitStart, this.cursor));
            this.state = { tag: "ground" };
            this.consumePrefix(this.cursor);
            continue;
          }

          if (isAsciiDigit(byte)) {
            this.cursor += 1;
            this.state = { tag: "csi_sgr_mouse", part: this.state.part, hasDigit: true };
            continue;
          }

          if (byte === 0x3b && this.state.hasDigit && this.state.part < 2) {
            this.cursor += 1;
            this.state = { tag: "csi_sgr_mouse", part: this.state.part + 1, hasDigit: false };
            continue;
          }

          if (byte >= 0x40 && byte <= 0x7e) {
            const end = this.cursor + 1;
            const rawBytes = bytes.subarray(this.unitStart, end);
            if (isMouseSgrSequence(rawBytes)) {
              this.emitMouse(rawBytes, "sgr");
            } else {
              this.emitKeyOrResponse("csi", decodeUtf8(rawBytes));
            }
            this.state = { tag: "ground" };
            this.consumePrefix(end);
            continue;
          }

          this.state = { tag: "csi" };
          continue;
        }

        case "csi_sgr_mouse_deferred": {
          if (this.cursor >= bytes.length) {
            this.pendingSinceMs = null;
            this.forceFlush = false;
            return;
          }

          if (byte === ESC) {
            this.emitOpaqueResponse("unknown", bytes.subarray(this.unitStart, this.cursor));
            this.state = { tag: "ground" };
            this.consumePrefix(this.cursor);
            continue;
          }

          if (isAsciiDigit(byte) || byte === 0x3b || byte === 0x4d || byte === 0x6d) {
            this.state = {
              tag: "csi_sgr_mouse",
              part: this.state.part,
              hasDigit: this.state.hasDigit,
            };
            continue;
          }

          this.emitOpaqueResponse("unknown", bytes.subarray(this.unitStart, this.cursor));
          this.state = { tag: "ground" };
          this.consumePrefix(this.cursor);
          continue;
        }

        case "csi_parametric": {
          if (this.cursor >= bytes.length) {
            if (!this.forceFlush) {
              this.markPending();
              return;
            }

            if (canDeferParametricCsi(this.state, this.protocolContext)) {
              this.state = {
                tag: "csi_parametric_deferred",
                semicolons: this.state.semicolons,
                segments: this.state.segments,
                hasDigit: this.state.hasDigit,
                firstParamValue: this.state.firstParamValue,
              };
              this.pendingSinceMs = null;
              this.forceFlush = false;
              return;
            }

            this.emitOpaqueResponse("unknown", bytes.subarray(this.unitStart, this.cursor));
            this.state = { tag: "ground" };
            this.consumePrefix(this.cursor);
            continue;
          }

          if (byte === ESC) {
            this.emitOpaqueResponse("unknown", bytes.subarray(this.unitStart, this.cursor));
            this.state = { tag: "ground" };
            this.consumePrefix(this.cursor);
            continue;
          }

          if (isAsciiDigit(byte)) {
            this.cursor += 1;
            this.state = {
              tag: "csi_parametric",
              semicolons: this.state.semicolons,
              segments: this.state.segments,
              hasDigit: true,
              firstParamValue: this.state.firstParamValue,
            };
            continue;
          }

          if (byte === 0x3a && this.state.hasDigit && this.state.segments < 3) {
            this.cursor += 1;
            this.state = {
              tag: "csi_parametric",
              semicolons: this.state.semicolons,
              segments: this.state.segments + 1,
              hasDigit: false,
              firstParamValue: this.state.firstParamValue,
            };
            continue;
          }

          if (byte === 0x3b && this.state.semicolons < 2) {
            this.cursor += 1;
            this.state = {
              tag: "csi_parametric",
              semicolons: this.state.semicolons + 1,
              segments: 1,
              hasDigit: false,
              firstParamValue: this.state.firstParamValue,
            };
            continue;
          }

          if (byte >= 0x40 && byte <= 0x7e) {
            const end = this.cursor + 1;
            const protocol = classifyParametricCsiProtocol(this.state, byte);
            this.emitKeyOrResponse(protocol, decodeUtf8(bytes.subarray(this.unitStart, end)));
            this.state = { tag: "ground" };
            this.consumePrefix(end);
            continue;
          }

          this.state = { tag: "csi" };
          continue;
        }

        case "csi_parametric_deferred": {
          if (this.cursor >= bytes.length) {
            this.pendingSinceMs = null;
            this.forceFlush = false;
            return;
          }

          if (byte === ESC) {
            this.emitOpaqueResponse("unknown", bytes.subarray(this.unitStart, this.cursor));
            this.state = { tag: "ground" };
            this.consumePrefix(this.cursor);
            continue;
          }

          if (isAsciiDigit(byte) || byte === 0x3a || byte === 0x3b) {
            this.state = {
              tag: "csi_parametric",
              semicolons: this.state.semicolons,
              segments: this.state.segments,
              hasDigit: this.state.hasDigit,
              firstParamValue: this.state.firstParamValue,
            };
            continue;
          }

          if (canCompleteDeferredParametricCsi(this.state, byte, this.protocolContext)) {
            this.state = {
              tag: "csi_parametric",
              semicolons: this.state.semicolons,
              segments: this.state.segments,
              hasDigit: this.state.hasDigit,
              firstParamValue: this.state.firstParamValue,
            };
            continue;
          }

          this.emitOpaqueResponse("unknown", bytes.subarray(this.unitStart, this.cursor));
          this.state = { tag: "ground" };
          this.consumePrefix(this.cursor);
          continue;
        }

        case "csi_parametric_ignored": {
          if (this.cursor >= bytes.length) {
            if (!this.forceFlush) {
              this.markPending();
              return;
            }
            this.state = { tag: "ground" };
            this.consumePrefix(this.cursor);
            continue;
          }

          if (byte === ESC) {
            this.state = { tag: "ground" };
            this.consumePrefix(this.cursor);
            continue;
          }

          if (isAsciiDigit(byte)) {
            this.cursor += 1;
            this.state = {
              tag: "csi_parametric_ignored",
              semicolons: this.state.semicolons,
              segments: this.state.segments,
              hasDigit: true,
              firstParamValue:
                this.state.semicolons === 0
                  ? (this.state.firstParamValue ?? 0) * 10 + (byte - 0x30)
                  : this.state.firstParamValue,
            };
            continue;
          }

          if (byte === 0x3b && this.state.semicolons === 0 && this.state.hasDigit) {
            if (this.protocolContext.explicitWidthCprActive && this.state.firstParamValue === 1) {
              this.state = { tag: "csi" };
              continue;
            }

            this.cursor += 1;
            this.state = {
              tag: "csi_parametric_ignored",
              semicolons: 1,
              segments: 1,
              hasDigit: false,
              firstParamValue: this.state.firstParamValue,
            };
            continue;
          }

          if (byte === 0x52 && this.state.semicolons === 1 && this.state.hasDigit) {
            const end = this.cursor + 1;
            this.state = { tag: "ground" };
            this.consumePrefix(end);
            continue;
          }

          if (this.state.semicolons === 0) {
            this.state = { tag: "csi" };
            continue;
          }

          this.state = { tag: "ground" };
          this.consumePrefix(this.cursor);
          continue;
        }

        case "csi_private_reply": {
          if (this.cursor >= bytes.length) {
            if (!this.forceFlush) {
              this.markPending();
              return;
            }

            if (canDeferPrivateReplyCsi(this.protocolContext)) {
              this.state = {
                tag: "csi_private_reply_deferred",
                semicolons: this.state.semicolons,
                hasDigit: this.state.hasDigit,
                sawDollar: this.state.sawDollar,
              };
              this.pendingSinceMs = null;
              this.forceFlush = false;
              return;
            }

            this.emitOpaqueResponse("unknown", bytes.subarray(this.unitStart, this.cursor));
            this.state = { tag: "ground" };
            this.consumePrefix(this.cursor);
            continue;
          }

          if (byte === ESC) {
            this.emitOpaqueResponse("unknown", bytes.subarray(this.unitStart, this.cursor));
            this.state = { tag: "ground" };
            this.consumePrefix(this.cursor);
            continue;
          }

          if (isAsciiDigit(byte)) {
            this.cursor += 1;
            this.state = {
              tag: "csi_private_reply",
              semicolons: this.state.semicolons,
              hasDigit: true,
              sawDollar: this.state.sawDollar,
            };
            continue;
          }

          if (byte === 0x3b) {
            this.cursor += 1;
            this.state = {
              tag: "csi_private_reply",
              semicolons: this.state.semicolons + 1,
              hasDigit: false,
              sawDollar: false,
            };
            continue;
          }

          if (byte === 0x24 && this.state.hasDigit && !this.state.sawDollar) {
            this.cursor += 1;
            this.state = {
              tag: "csi_private_reply",
              semicolons: this.state.semicolons,
              hasDigit: true,
              sawDollar: true,
            };
            continue;
          }

          if (byte >= 0x40 && byte <= 0x7e) {
            const end = this.cursor + 1;
            this.emitOpaqueResponse("csi", bytes.subarray(this.unitStart, end));
            this.state = { tag: "ground" };
            this.consumePrefix(end);
            continue;
          }

          this.state = { tag: "csi" };
          continue;
        }

        case "csi_private_reply_deferred": {
          if (this.cursor >= bytes.length) {
            this.pendingSinceMs = null;
            this.forceFlush = false;
            return;
          }

          if (byte === ESC) {
            this.emitOpaqueResponse("unknown", bytes.subarray(this.unitStart, this.cursor));
            this.state = { tag: "ground" };
            this.consumePrefix(this.cursor);
            continue;
          }

          if (isAsciiDigit(byte) || byte === 0x3b || byte === 0x24) {
            this.state = {
              tag: "csi_private_reply",
              semicolons: this.state.semicolons,
              hasDigit: this.state.hasDigit,
              sawDollar: this.state.sawDollar,
            };
            continue;
          }

          if (canCompleteDeferredPrivateReplyCsi(this.state, byte, this.protocolContext)) {
            this.state = {
              tag: "csi_private_reply",
              semicolons: this.state.semicolons,
              hasDigit: this.state.hasDigit,
              sawDollar: this.state.sawDollar,
            };
            continue;
          }

          this.emitOpaqueResponse("unknown", bytes.subarray(this.unitStart, this.cursor));
          this.state = { tag: "ground" };
          this.consumePrefix(this.cursor);
          continue;
        }

        case "osc":
        case "dcs":
        case "apc": {
          if (this.cursor >= bytes.length) {
            if (!this.forceFlush) {
              this.markPending();
              return;
            }

            this.emitOpaqueResponse("unknown", bytes.subarray(this.unitStart, this.cursor));
            this.state = { tag: "ground" };
            this.consumePrefix(this.cursor);
            continue;
          }

          if (byte === ESC) {
            this.cursor += 1;
            this.state = { tag: this.state.tag, sawEsc: true };
            continue;
          }

          if (this.state.sawEsc && byte === 0x5c) {
            const end = this.cursor + 1;
            this.emitOpaqueResponse(this.state.tag, bytes.subarray(this.unitStart, end));
            this.state = { tag: "ground" };
            this.consumePrefix(end);
            continue;
          }

          if (this.state.tag === "osc" && byte === BEL) {
            const end = this.cursor + 1;
            this.emitOpaqueResponse("osc", bytes.subarray(this.unitStart, end));
            this.state = { tag: "ground" };
            this.consumePrefix(end);
            continue;
          }

          this.cursor += 1;
          this.state = { tag: this.state.tag, sawEsc: false };
          continue;
        }

        case "esc_less_mouse": {
          if (this.cursor >= bytes.length) {
            if (!this.forceFlush) {
              this.markPending();
              return;
            }

            this.emitKeyOrResponse(
              "unknown",
              decodeUtf8(bytes.subarray(this.unitStart, this.cursor)),
            );
            this.state = { tag: "ground" };
            this.consumePrefix(this.cursor);
            continue;
          }

          if (byte === 0x4d || byte === 0x6d) {
            const end = this.cursor + 1;
            const fullBytes = withEscPrefix(bytes.subarray(this.unitStart, end));
            if (isMouseSgrSequence(fullBytes)) {
              this.emitMouse(fullBytes, "sgr");
              this.state = { tag: "ground" };
              this.consumePrefix(end);
              continue;
            }
          }

          this.emitKeyOrResponse(
            "unknown",
            decodeUtf8(bytes.subarray(this.unitStart, this.unitStart + 1)),
          );
          this.state = { tag: "ground" };
          this.consumePrefix(this.unitStart + 1);
          continue;
        }

        case "esc_less_x10_mouse": {
          const expectedEnd = this.unitStart + 4;
          if (bytes.length < expectedEnd) {
            if (!this.forceFlush) {
              this.markPending();
              return;
            }
            this.emitOpaqueResponse("unknown", bytes.subarray(this.unitStart, bytes.length));
            this.state = { tag: "ground" };
            this.consumePrefix(bytes.length);
            continue;
          }

          this.emitMouse(withEscPrefix(bytes.subarray(this.unitStart, expectedEnd)), "x10");
          this.state = { tag: "ground" };
          this.consumePrefix(expectedEnd);
          continue;
        }
      }
    }
  }

  private consumePasteBytes(bytes: Uint8Array): Uint8Array {
    if (!this.paste) return bytes;

    const endIndex = indexOfBytes(bytes, BRACKETED_PASTE_END);
    if (endIndex !== -1) {
      const endLimit = endIndex + BRACKETED_PASTE_END.length;
      const bodyBytes = bytes.subarray(0, endIndex);

      this.paste.parts.push(bodyBytes);
      this.paste.totalLength += bodyBytes.length;

      this.events.push({
        type: "paste",
        bytes: joinPasteBytes(this.paste.parts, this.paste.totalLength),
      });

      this.paste = null;
      this.state = { tag: "ground" };
      return bytes.subarray(endLimit);
    }

    this.paste.parts.push(bytes);
    this.paste.totalLength += bytes.length;
    return EMPTY_BYTES;
  }

  private takePendingBytes(): Uint8Array {
    const bytes = this.pending.take();
    this.cursor = 0;
    this.unitStart = 0;
    this.pendingSinceMs = null;
    this.forceFlush = false;
    return bytes;
  }

  private flushPendingOverflow(): void {
    if (this.pending.length === 0) return;
    const bytes = this.takePendingBytes();
    this.emitOpaqueResponse("unknown", bytes);
  }

  private emitLegacyHighByte(byte: number): void {
    const str = String.fromCharCode(byte);
    this.emitKeyOrResponse("unknown", str);
  }

  private emitKeyOrResponse(protocol: StdinResponseProtocol, sequence: string): void {
    if (sequence === "") return;

    if (protocol !== "unknown") {
      this.events.push({ type: "response", protocol, sequence });
      return;
    }

    const key = parseKeypress(sequence, { useKittyKeyboard: this.useKittyKeyboard });
    if (!key) {
      this.events.push({ type: "response", protocol: "unknown", sequence });
      return;
    }

    this.events.push({ type: "key", raw: sequence, key });
  }

  private emitOpaqueResponse(protocol: StdinResponseProtocol, bytes: Uint8Array): void {
    this.events.push({ type: "response", protocol, sequence: decodeLatin1(bytes) });
  }

  private emitMouse(bytes: Uint8Array, encoding: "sgr" | "x10"): void {
    const event = this.mouseParser.parseMouseEvent(bytes);
    if (!event) return;
    this.events.push({
      type: "mouse",
      raw: decodeLatin1(bytes),
      encoding,
      event,
    });
  }

  private consumePrefix(endExclusive: number): void {
    this.pending.consume(endExclusive);
    this.cursor = 0;
    this.unitStart = 0;
    this.pendingSinceMs = null;
    this.forceFlush = false;
  }

  private markPending(): void {
    if (this.pendingSinceMs === null) {
      this.pendingSinceMs = this.clock.now();
    }
  }

  private resetState(): void {
    this.pending.reset();
    this.events.length = 0;
    this.pendingSinceMs = null;
    this.forceFlush = false;
    this.justFlushedEsc = false;
    this.state = { tag: "ground" };
    this.cursor = 0;
    this.unitStart = 0;
    this.paste = null;
    this.mouseParser.reset();
  }

  private reconcileDeferredStateWithProtocolContext(): void {
    if (this.state.tag === "csi_parametric_deferred") {
      if (!canDeferParametricCsi(this.state, this.protocolContext)) {
        this.forceFlush = true;
      }
    } else if (this.state.tag === "csi_private_reply_deferred") {
      if (!canDeferPrivateReplyCsi(this.protocolContext)) {
        this.forceFlush = true;
      }
    }
  }

  private reconcileTimeoutState(): void {
    if (!this.armTimeouts) return;

    const hasPendingTimeableData =
      this.pendingSinceMs !== null && !this.forceFlush && !this.paste && this.pending.length > 0;

    if (!hasPendingTimeableData) {
      this.clearTimeout();
      return;
    }

    if (this.timeoutId !== null) return;

    this.timeoutId = this.clock.setTimeout(() => {
      this.timeoutId = null;
      if (this.destroyed) return;
      this.tryForceFlush();
      if (this.onTimeoutFlush) {
        this.onTimeoutFlush();
      }
    }, this.timeoutMs);
  }

  private clearTimeout(): void {
    if (this.timeoutId !== null) {
      this.clock.clearTimeout(this.timeoutId);
      this.timeoutId = null;
    }
  }
}
