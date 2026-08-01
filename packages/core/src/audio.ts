/**
 * Audio API stubs for BetterTUI.
 *
 * These provide the full TypeScript interface matching OpenTUI's audio API so
 * that examples compile and run without crashing.  Actual playback requires
 * the native audio engine which is not yet implemented.
 */

import { EventEmitter } from "node:events";

// ── Type aliases ──────────────────────────────────────────────────────────────

export type AudioGroup = number;

// ── Error class ───────────────────────────────────────────────────────────────

export type AudioStreamAction =
  | "connect"
  | "read"
  | "decode"
  | "play"
  | "reconnect"
  | "stop"
  | "dispose";

export interface AudioStreamErrorContext {
  action: AudioStreamAction;
  status?: number;
  errorCode?: number;
  attempt?: number;
}

export class AudioStreamError extends Error {
  readonly context: AudioStreamErrorContext;

  constructor(message: string, context: AudioStreamErrorContext, cause?: unknown) {
    super(message, cause ? { cause } : undefined);
    this.name = "AudioStreamError";
    this.context = context;
  }
}

// ── Audio stream types ────────────────────────────────────────────────────────

export type AudioStreamState =
  | "initializing"
  | "buffering"
  | "playing"
  | "reconnecting"
  | "ended"
  | "errored"
  | "disposed"
  | "idle";

export interface AudioStreamStats {
  state: AudioStreamState;
  sampleRate: number;
  channels: number;
  bufferedFrames: number;
  capacityFrames: number;
  bufferedDurationMs: number;
  bytesReceived: bigint;
  framesDecoded: bigint;
  framesPlayed: bigint;
  underruns: number;
  reconnectAttempts: number;
}

export type AudioStreamMetadataFormat = "icy" | string;

export interface AudioStreamMetadata {
  readonly format: AudioStreamMetadataFormat;
  readonly headers: Readonly<Record<string, string>>;
  readonly fields: Readonly<Record<string, string>>;
}

export interface AudioStreamReconnectEvent {
  attempt: number;
  delayMs: number;
  error: AudioStreamError;
}

export interface AudioStreamUrlOptions {
  format?: "mp3" | "flac" | string;
  signal?: AbortSignal;
  volume?: number;
  pan?: number;
  groupId?: number;
  buffer?: { capacityMs?: number; startupMs?: number; resumeMs?: number };
  reconnect?: {
    maxRetries?: number;
    retryOnEnd?: boolean;
    initialDelayMs?: number;
    maxDelayMs?: number;
    backoffFactor?: number;
  };
}

export class AudioStream<M = AudioStreamMetadata> extends EventEmitter {
  private _state: AudioStreamState = "disposed";

  get state(): AudioStreamState {
    return this._state;
  }

  getStats(): AudioStreamStats {
    return {
      state: this._state,
      sampleRate: 0,
      channels: 0,
      bufferedFrames: 0,
      capacityFrames: 0,
      bufferedDurationMs: 0,
      bytesReceived: 0n,
      framesDecoded: 0n,
      framesPlayed: 0n,
      underruns: 0,
      reconnectAttempts: 0,
    };
  }

  getMetadata(): M | null {
    return null;
  }

  setVolume(_volume: number): boolean {
    return false;
  }

  setPan(_pan: number): boolean {
    return false;
  }

  setGroup(_groupId: number): boolean {
    return false;
  }

  dispose(): void {
    this._state = "disposed";
    this.emit("disposed");
    this.removeAllListeners();
  }
}

// ── Native audio types ────────────────────────────────────────────────────────

export interface AudioPlaybackDevice {
  name: string;
  id: string;
  isDefault: boolean;
}

export interface AudioSound {
  id: string;
  duration: number;
}

export interface AudioVoice {
  id: string;
  sound: AudioSound;
}

export interface AudioSetupOptions {
  autoStart?: boolean;
  sampleRate?: number;
  channels?: number;
  bufferSize?: number;
}

export interface AudioStartOptions {
  deviceId?: string;
}

export interface AudioPlayOptions {
  volume?: number;
  pan?: number;
  loop?: boolean;
  groupId?: number;
}

export interface AudioStats {
  lastPeak: number;
  lastRms: number;
  framesProcessed: bigint;
}

export interface AudioTapResult {
  framesRead: number;
  frames: Float32Array;
}

// ── Audio class ───────────────────────────────────────────────────────────────

export class Audio extends EventEmitter {
  readonly sampleRate: number;
  private _started = false;
  private _mixerStarted = false;
  private _disposed = false;

  private constructor(options: AudioSetupOptions = {}) {
    super();
    this.sampleRate = options.sampleRate ?? 48_000;
  }

  /** Factory — matches OpenTUI's `Audio.create()` API. */
  static create(options: AudioSetupOptions = {}): Audio {
    return new Audio(options);
  }

  start(_options?: AudioStartOptions): boolean {
    // Native audio not available; report graceful failure
    return false;
  }

  startMixer(): boolean {
    if (this._disposed) return false;
    this._mixerStarted = true;
    return true;
  }

  stop(): boolean {
    this._started = false;
    this._mixerStarted = false;
    return true;
  }

  isStarted(): boolean {
    return this._started;
  }

  isMixerStarted(): boolean {
    return this._mixerStarted;
  }

  /**
   * Create an audio group.  Returns the group id (a small integer starting at 1).
   * Matches both OpenTUI's `audio.createGroup()` and the example's `audio.group()` call.
   */
  private _groupCounter = 0;

  group(_name: string): AudioGroup {
    return ++this._groupCounter;
  }

  createGroup(_name: string): AudioGroup {
    return ++this._groupCounter;
  }

  setGroupVolume(_group: AudioGroup, _volume: number): boolean {
    return true;
  }

  setMasterVolume(_volume: number): boolean {
    return true;
  }

  enableTap(_bufferSize: number): void {}

  disableTap(): void {}

  readTapFrames(_frameCount: number, _channels: number): AudioTapResult | null {
    return null;
  }

  mixFrames(_frameCount: number, _channels: number): void {}

  getStats(): AudioStats | null {
    return null;
  }

  loadSound(_data: Uint8Array | ArrayBuffer): AudioSound | null {
    return null;
  }

  async loadSoundFile(_path: string): Promise<AudioSound | null> {
    return null;
  }

  unloadSound(_sound: AudioSound): boolean {
    return false;
  }

  play(_sound: AudioSound, _options?: AudioPlayOptions): AudioVoice | null {
    return null;
  }

  stopVoice(_voice: AudioVoice): boolean {
    return false;
  }

  setVoiceGroup(_voice: AudioVoice, _group: AudioGroup): boolean {
    return false;
  }

  async playStreamUrl(_url: string | URL, _options?: AudioStreamUrlOptions): Promise<AudioStream> {
    const stream = new AudioStream();
    // Immediately end the stream since audio is not available
    setImmediate(() => {
      stream.emit("ended");
    });
    return stream;
  }

  dispose(): void {
    if (this._disposed) return;
    this._disposed = true;
    this._started = false;
    this._mixerStarted = false;
    this.emit("disposed");
    this.removeAllListeners();
  }
}
