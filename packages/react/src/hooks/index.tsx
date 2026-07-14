import type { KeyEvent as SharedKeyEvent } from "@bettertui/shared";
import { DEFAULT_THEME, mergeTheme } from "@bettertui/shared";
import type { Theme } from "@bettertui/shared";
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import type { ReactNode } from "react";

// Re-export shared types for consumer convenience
export type { Theme };
export type ThemeColors = Theme["colors"];
export type ThemeSpacing = Theme["spacing"];
export type KeyEvent = SharedKeyEvent;

// Theme context
interface ThemeContextValue {
  theme: Theme;
  setTheme: (theme: Partial<Theme>) => void;
}

const ThemeContext = createContext<ThemeContextValue>({
  theme: DEFAULT_THEME,
  setTheme: () => {},
});

// Theme provider
export interface ProviderProps {
  children: ReactNode;
  theme?: Partial<Theme>;
}

export function Provider({ children, theme }: ProviderProps) {
  const [currentTheme, setCurrentTheme] = useState<Theme>(() =>
    theme ? mergeTheme(DEFAULT_THEME, theme) : DEFAULT_THEME,
  );

  const setTheme = useCallback((partial: Partial<Theme>) => {
    setCurrentTheme((prev) => mergeTheme(prev, partial));
  }, []);

  return (
    <ThemeContext.Provider value={{ theme: currentTheme, setTheme }}>
      {children}
    </ThemeContext.Provider>
  );
}

// Hooks
export function useTheme() {
  return useContext(ThemeContext);
}

// Focus management
interface FocusContextValue {
  focusedId: string | null;
  setFocusedId: (id: string | null) => void;
  focusNext: () => void;
  focusPrevious: () => void;
}

const FocusContext = createContext<FocusContextValue>({
  focusedId: null,
  setFocusedId: () => {},
  focusNext: () => {},
  focusPrevious: () => {},
});

export function FocusProvider({ children }: { children: ReactNode }) {
  const [focusedId, setFocusedId] = useState<string | null>(null);
  const focusableIds = useRef<string[]>([]);

  const focusNext = useCallback(() => {
    const ids = focusableIds.current;
    if (ids.length === 0) return;

    const currentIndex = ids.indexOf(focusedId ?? "");
    const nextIndex = (currentIndex + 1) % ids.length;
    const next = ids[nextIndex];
    if (next !== undefined) setFocusedId(next);
  }, [focusedId]);

  const focusPrevious = useCallback(() => {
    const ids = focusableIds.current;
    if (ids.length === 0) return;

    const currentIndex = ids.indexOf(focusedId ?? "");
    const previousIndex = (currentIndex - 1 + ids.length) % ids.length;
    const prev = ids[previousIndex];
    if (prev !== undefined) setFocusedId(prev);
  }, [focusedId]);

  return (
    <FocusContext.Provider value={{ focusedId, setFocusedId, focusNext, focusPrevious }}>
      {children}
    </FocusContext.Provider>
  );
}

export function useFocus() {
  return useContext(FocusContext);
}

// Keyboard handling
export function useKeyboard(handler: (event: SharedKeyEvent) => boolean) {
  const handlerRef = useRef(handler);
  handlerRef.current = handler;

  useEffect(() => {
    const handleKeyDown = (event: globalThis.KeyboardEvent) => {
      const ke = event as unknown as {
        key: string;
        code: string;
        ctrlKey: boolean;
        shiftKey: boolean;
        altKey: boolean;
        metaKey: boolean;
      };
      const keyEvent: SharedKeyEvent = {
        key: ke.key,
        code: ke.code,
        ctrl: ke.ctrlKey,
        shift: ke.shiftKey,
        alt: ke.altKey,
        meta: ke.metaKey,
      };

      if (handlerRef.current(keyEvent)) {
        event.preventDefault();
      }
    };

    if (typeof globalThis !== "undefined" && "addEventListener" in globalThis) {
      const g = globalThis as unknown as {
        addEventListener(name: string, fn: (e: globalThis.KeyboardEvent) => void): void;
        removeEventListener(name: string, fn: (e: globalThis.KeyboardEvent) => void): void;
      };
      g.addEventListener("keydown", handleKeyDown);
      return () => g.removeEventListener("keydown", handleKeyDown);
    }
    return undefined;
  }, []);
}

// Terminal information
interface TerminalContextValue {
  width: number;
  height: number;
  resize: (width: number, height: number) => void;
}

const TerminalContext = createContext<TerminalContextValue>({
  width: 80,
  height: 24,
  resize: () => {},
});

export function TerminalProvider({ children }: { children: ReactNode }) {
  const [size, setSize] = useState({ width: 80, height: 24 });

  const resize = useCallback((width: number, height: number) => {
    setSize({ width, height });
  }, []);

  return (
    <TerminalContext.Provider value={{ width: size.width, height: size.height, resize }}>
      {children}
    </TerminalContext.Provider>
  );
}

export function useTerminal() {
  return useContext(TerminalContext);
}

// Global resize listener
export function useResize(handler: (width: number, height: number) => void) {
  const handlerRef = useRef(handler);
  handlerRef.current = handler;

  useEffect(() => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const processObj =
      typeof process !== "undefined"
        ? (process as unknown as {
            stdout?: {
              columns?: number;
              rows?: number;
              on: (event: string, listener: () => void) => void;
              off: (event: string, listener: () => void) => void;
            };
          })
        : undefined;
    if (!processObj || !processObj.stdout) return;

    const stdout = processObj.stdout;

    const onResize = () => {
      handlerRef.current(stdout.columns || 80, stdout.rows || 24);
    };

    if (typeof stdout.on === "function") {
      stdout.on("resize", onResize);
      return () => {
        if (typeof stdout.off === "function") stdout.off("resize", onResize);
      };
    }
    return () => {};
  }, []);
}

// Frame management
export function useFrame() {
  const [frameRequested, setFrameRequested] = useState(false);

  const requestFrame = useCallback(() => {
    setFrameRequested(true);
  }, []);

  useEffect(() => {
    if (frameRequested) {
      setFrameRequested(false);
    }
  }, [frameRequested]);

  return { requestFrame, frameRequested };
}

// Clipboard
export function useClipboard() {
  const [clipboard, setClipboard] = useState("");

  const copy = useCallback(async (text: string) => {
    const n = globalThis as unknown as {
      navigator?: { clipboard?: { writeText(t: string): Promise<void> } };
    };
    if (n.navigator?.clipboard) {
      await n.navigator.clipboard.writeText(text);
    }
    setClipboard(text);
  }, []);

  const paste = useCallback(async () => {
    const n = globalThis as unknown as {
      navigator?: { clipboard?: { readText(): Promise<string> } };
    };
    if (n.navigator?.clipboard) {
      const text = await n.navigator.clipboard.readText();
      setClipboard(text);
      return text;
    }
    return clipboard;
  }, [clipboard]);

  return { clipboard, copy, paste };
}

const _raf = (cb: (t: number) => void): number => {
  const g = globalThis as unknown as {
    requestAnimationFrame?: (cb: (t: number) => void) => number;
    setTimeout?: (cb: () => void, ms: number) => number;
  };
  if (g.requestAnimationFrame) return g.requestAnimationFrame(cb);
  return g.setTimeout?.(() => cb(performance.now()), 16) ?? 0;
};

const _caf = (id: number): void => {
  const g = globalThis as unknown as {
    cancelAnimationFrame?: (id: number) => void;
    clearTimeout?: (id: number) => void;
  };
  if (g.cancelAnimationFrame) g.cancelAnimationFrame(id);
  else g.clearTimeout?.(id);
};

// Easing functions
export type EasingFunction = (t: number) => number;

export const easings = {
  linear: (t: number) => t,
  inQuad: (t: number) => t * t,
  outQuad: (t: number) => t * (2 - t),
  inOutQuad: (t: number) => (t < 0.5 ? 2 * t * t : -1 + (4 - 2 * t) * t),
  inCubic: (t: number) => t * t * t,
  outCubic: (t: number) => {
    const t1 = t - 1;
    return t1 * t1 * t1 + 1;
  },
  inOutCubic: (t: number) => (t < 0.5 ? 4 * t * t * t : (t - 1) * (2 * t - 2) * (2 * t - 2) + 1),
  inExpo: (t: number) => (t === 0 ? 0 : 2 ** (10 * (t - 1))),
  outExpo: (t: number) => (t === 1 ? 1 : 1 - 2 ** (-10 * t)),
  inOutExpo: (t: number) => {
    if (t === 0 || t === 1) return t;
    return t < 0.5 ? 2 ** (20 * t - 10) / 2 : (2 - 2 ** (-20 * t + 10)) / 2;
  },
  inSine: (t: number) => 1 - Math.cos((t * Math.PI) / 2),
  outSine: (t: number) => Math.sin((t * Math.PI) / 2),
  inOutSine: (t: number) => -(Math.cos(Math.PI * t) - 1) / 2,
  inBounce: (t: number) => 1 - easings.outBounce(1 - t),
  outBounce: (t: number) => {
    if (t < 1 / 2.75) return 7.5625 * t * t;
    let t2 = t;
    if (t2 < 2 / 2.75) {
      t2 -= 1.5 / 2.75;
      return 7.5625 * t2 * t2 + 0.75;
    }
    if (t2 < 2.5 / 2.75) {
      t2 -= 2.25 / 2.75;
      return 7.5625 * t2 * t2 + 0.9375;
    }
    t2 -= 2.625 / 2.75;
    return 7.5625 * t2 * t2 + 0.984375;
  },
  inOutBounce: (t: number) =>
    t < 0.5 ? easings.inBounce(t * 2) * 0.5 : easings.outBounce(t * 2 - 1) * 0.5 + 0.5,
  inElastic: (t: number) => {
    if (t === 0 || t === 1) return t;
    return -(2 ** (10 * (t - 1))) * Math.sin((t - 1.1) * 5 * Math.PI);
  },
  outElastic: (t: number) => {
    if (t === 0 || t === 1) return t;
    return 2 ** (-10 * t) * Math.sin((t - 0.1) * 5 * Math.PI) + 1;
  },
  inOutElastic: (t: number) => {
    if (t === 0 || t === 1) return t;
    return t < 0.5
      ? -(2 ** (20 * t - 10) * Math.sin((20 * t - 11.125) * ((2 * Math.PI) / 4.5))) / 2
      : (2 ** (-20 * t + 10) * Math.sin((20 * t - 11.125) * ((2 * Math.PI) / 4.5))) / 2 + 1;
  },
  inBack: (t: number) => t * t * (2.70158 * t - 1.70158),
  outBack: (t: number) => {
    const s = 1.70158;
    const t1 = t - 1;
    return t1 * t1 * ((s + 1) * t1 + s) + 1;
  },
  inOutBack: (t: number) => {
    const s = 1.70158 * 1.525;
    const t2 = t * 2;
    if (t2 < 1) return 0.5 * (t2 * t2 * ((s + 1) * t2 - s));
    const t3 = t2 - 2;
    return 0.5 * (t3 * t3 * ((s + 1) * t3 + s) + 2);
  },
};

// Animation
export interface UseAnimationOptions {
  duration: number;
  easing?: EasingFunction | keyof typeof easings;
  loop?: boolean | number;
  alternate?: boolean;
  delay?: number;
  onComplete?: () => void;
  onStart?: () => void;
}

export function useAnimation(
  callback: (progress: number) => void,
  options: UseAnimationOptions | number,
  deps: unknown[] = [],
) {
  const opts: UseAnimationOptions = typeof options === "number" ? { duration: options } : options;
  const {
    duration,
    easing = "linear",
    loop = false,
    alternate = false,
    delay = 0,
    onComplete,
    onStart,
  } = opts;

  const callbackRef = useRef(callback);
  callbackRef.current = callback;
  const onCompleteRef = useRef(onComplete);
  onCompleteRef.current = onComplete;
  const onStartRef = useRef(onStart);
  onStartRef.current = onStart;
  const loopRef = useRef(loop);
  loopRef.current = loop;
  const alternateRef = useRef(alternate);
  alternateRef.current = alternate;

  const easingFn: EasingFunction =
    typeof easing === "function" ? easing : (easings[easing] ?? easings.linear);
  const easingFnRef = useRef(easingFn);
  easingFnRef.current = easingFn;

  useEffect(() => {
    let animationFrame: number;
    let startTime: number;
    let iterationCount = 0;
    const maxIterations =
      typeof loopRef.current === "number"
        ? loopRef.current
        : loopRef.current
          ? Number.POSITIVE_INFINITY
          : 1;

    const animate = (currentTime: number) => {
      if (!startTime) startTime = currentTime;
      const elapsed = currentTime - startTime - delay;

      if (elapsed < 0) {
        animationFrame = _raf(animate);
        return;
      }

      const rawProgress = Math.min(elapsed / duration, 1);
      const easedProgress = easingFnRef.current(rawProgress);
      const direction =
        alternateRef.current && iterationCount % 2 === 1 ? 1 - easedProgress : easedProgress;

      callbackRef.current(direction);

      if (rawProgress === 1) {
        iterationCount++;
        if (iterationCount < maxIterations) {
          startTime = currentTime;
          if (iterationCount === 1) onStartRef.current?.();
          animationFrame = _raf(animate);
        } else {
          onCompleteRef.current?.();
        }
      } else {
        animationFrame = _raf(animate);
      }
    };

    animationFrame = _raf(animate);

    return () => {
      if (animationFrame) {
        _caf(animationFrame);
      }
    };
  }, [duration, delay, ...deps]);
}

// Timeline
export interface TimelineAnimation {
  id: number;
  target: Record<string, number>;
  props: Record<string, { from: number; to: number }>;
  startTime: number;
  duration: number;
  easing: EasingFunction;
  onComplete?: () => void;
}

export interface Timeline {
  add: (
    target: Record<string, number>,
    props: Record<string, { from: number; to: number }>,
    startTime?: number,
    duration?: number,
    easing?: EasingFunction | keyof typeof easings,
  ) => number;
  call: (callback: () => void, startTime: number) => void;
  play: () => void;
  pause: () => void;
  reset: () => void;
  isPlaying: () => boolean;
  progress: () => number;
}

export function useTimeline(deps: unknown[] = []): Timeline {
  const animationsRef = useRef<TimelineAnimation[]>([]);
  const callbacksRef = useRef<Array<{ time: number; fn: () => void }>>([]);
  const playingRef = useRef(false);
  const startTimeRef = useRef(0);
  const currentTimeRef = useRef(0);
  const idCounterRef = useRef(0);

  const add = useCallback(
    (
      target: Record<string, number>,
      props: Record<string, { from: number; to: number }>,
      startTime = 0,
      duration = 300,
      easing: EasingFunction | keyof typeof easings = "linear",
    ) => {
      const id = idCounterRef.current++;
      const easingFn: EasingFunction =
        typeof easing === "function" ? easing : (easings[easing] ?? easings.linear);
      animationsRef.current.push({
        id,
        target,
        props,
        startTime,
        duration,
        easing: easingFn,
      });
      return id;
    },
    [],
  );

  const call = useCallback((callback: () => void, startTime: number) => {
    callbacksRef.current.push({ time: startTime, fn: callback });
  }, []);

  const play = useCallback(() => {
    playingRef.current = true;
    startTimeRef.current = performance.now() - currentTimeRef.current;
  }, []);

  const pause = useCallback(() => {
    playingRef.current = false;
  }, []);

  const reset = useCallback(() => {
    playingRef.current = false;
    currentTimeRef.current = 0;
    for (const anim of animationsRef.current) {
      for (const key of Object.keys(anim.props)) {
        const prop = anim.props[key];
        if (prop) {
          anim.target[key] = prop.from;
        }
      }
    }
  }, []);

  const isPlaying = useCallback(() => playingRef.current, []);

  const progress = useCallback(() => {
    const maxDuration = Math.max(...animationsRef.current.map((a) => a.startTime + a.duration), 1);
    return currentTimeRef.current / maxDuration;
  }, []);

  useEffect(() => {
    let animationFrame: number;

    const animate = (currentTime: number) => {
      if (!playingRef.current) {
        animationFrame = _raf(animate);
        return;
      }

      const elapsed = currentTime - startTimeRef.current;
      currentTimeRef.current = elapsed;

      // Execute callbacks
      const cbs = callbacksRef.current;
      if (cbs) {
        const remaining: Array<{ time: number; fn: () => void }> = [];
        for (const cb of cbs) {
          if (elapsed >= cb.time) {
            cb.fn();
          } else {
            remaining.push(cb);
          }
        }
        callbacksRef.current = remaining;
      }

      // Update animations
      for (const anim of animationsRef.current) {
        const animElapsed = elapsed - anim.startTime;
        if (animElapsed < 0) continue;

        const rawProgress = Math.min(animElapsed / anim.duration, 1);
        const easedProgress = anim.easing(rawProgress);

        for (const key of Object.keys(anim.props)) {
          const prop = anim.props[key];
          if (prop) {
            const { from, to } = prop;
            anim.target[key] = from + (to - from) * easedProgress;
          }
        }

        if (rawProgress === 1 && anim.onComplete) {
          anim.onComplete();
        }
      }

      animationFrame = _raf(animate);
    };

    animationFrame = _raf(animate);

    return () => {
      if (animationFrame) {
        _caf(animationFrame);
      }
    };
  }, [...deps]);

  return useMemo(
    () => ({
      add,
      call,
      play,
      pause,
      reset,
      isPlaying,
      progress,
    }),
    [add, call, play, pause, reset, isPlaying, progress],
  );
}

// Mouse events
export interface MouseState {
  x: number;
  y: number;
  button: "left" | "right" | "middle" | "other";
  pressed: boolean;
  shift: boolean;
  ctrl: boolean;
  alt: boolean;
}

type MouseEventHandler = (e: MouseEvent) => void;

export function useMouse(handler: (event: MouseState) => boolean) {
  const handlerRef = useRef(handler);
  handlerRef.current = handler;

  useEffect(() => {
    const buttonMap: Record<number, MouseState["button"]> = {
      0: "left",
      1: "middle",
      2: "right",
    };

    const handleMouseDown: MouseEventHandler = (e) => {
      const mouseEvent: MouseState = {
        x: e.clientX,
        y: e.clientY,
        button: buttonMap[e.button] ?? "other",
        pressed: true,
        shift: e.shiftKey,
        ctrl: e.ctrlKey,
        alt: e.altKey,
      };
      if (handlerRef.current(mouseEvent)) {
        e.preventDefault();
      }
    };

    const handleMouseUp: MouseEventHandler = (e) => {
      const mouseEvent: MouseState = {
        x: e.clientX,
        y: e.clientY,
        button: buttonMap[e.button] ?? "other",
        pressed: false,
        shift: e.shiftKey,
        ctrl: e.ctrlKey,
        alt: e.altKey,
      };
      if (handlerRef.current(mouseEvent)) {
        e.preventDefault();
      }
    };

    const handleMouseMove: MouseEventHandler = (e) => {
      const mouseEvent: MouseState = {
        x: e.clientX,
        y: e.clientY,
        button: "other",
        pressed: false,
        shift: e.shiftKey,
        ctrl: e.ctrlKey,
        alt: e.altKey,
      };
      handlerRef.current(mouseEvent);
    };

    const g = globalThis as unknown as {
      addEventListener(name: string, fn: MouseEventHandler): void;
      removeEventListener(name: string, fn: MouseEventHandler): void;
    };
    g.addEventListener("mousedown", handleMouseDown);
    g.addEventListener("mouseup", handleMouseUp);
    g.addEventListener("mousemove", handleMouseMove);
    return () => {
      g.removeEventListener("mousedown", handleMouseDown);
      g.removeEventListener("mouseup", handleMouseUp);
      g.removeEventListener("mousemove", handleMouseMove);
    };
  }, []);
}

// Selection
interface SelectionContextValue {
  selection: { start: { x: number; y: number }; end: { x: number; y: number } } | null;
  setSelection: (
    sel: { start: { x: number; y: number }; end: { x: number; y: number } } | null,
  ) => void;
  getSelectedText: () => string;
}

const SelectionContext = createContext<SelectionContextValue>({
  selection: null,
  setSelection: () => {},
  getSelectedText: () => "",
});

export function SelectionProvider({ children }: { children: ReactNode }) {
  const [selection, setSelection] = useState<SelectionContextValue["selection"]>(null);

  const getSelectedText = useCallback(() => {
    if (!selection) return "";
    const n = globalThis as unknown as {
      getSelection?: () => { toString(): string } | null;
    };
    return n.getSelection?.()?.toString() ?? "";
  }, [selection]);

  return (
    <SelectionContext.Provider value={{ selection, setSelection, getSelectedText }}>
      {children}
    </SelectionContext.Provider>
  );
}

export function useSelection() {
  return useContext(SelectionContext);
}

// Capabilities
interface CapabilitiesContextValue {
  capabilities: {
    kittyKeyboard: boolean;
    sgrMouse: boolean;
    urxvtMouse: boolean;
    bracketedPaste: boolean;
    focusEvents: boolean;
    osc52Clipboard: boolean;
    trueColor: boolean;
  };
  updateCapabilities: (caps: Partial<CapabilitiesContextValue["capabilities"]>) => void;
}

const CapabilitiesContext = createContext<CapabilitiesContextValue>({
  capabilities: {
    kittyKeyboard: false,
    sgrMouse: false,
    urxvtMouse: false,
    bracketedPaste: false,
    focusEvents: false,
    osc52Clipboard: false,
    trueColor: false,
  },
  updateCapabilities: () => {},
});

export function CapabilitiesProvider({ children }: { children: ReactNode }) {
  const [capabilities, setCapabilities] = useState<CapabilitiesContextValue["capabilities"]>({
    kittyKeyboard: false,
    sgrMouse: false,
    urxvtMouse: false,
    bracketedPaste: false,
    focusEvents: false,
    osc52Clipboard: false,
    trueColor: false,
  });

  const updateCapabilities = useCallback(
    (caps: Partial<CapabilitiesContextValue["capabilities"]>) => {
      setCapabilities((prev) => ({ ...prev, ...caps }));
    },
    [],
  );

  return (
    <CapabilitiesContext.Provider value={{ capabilities, updateCapabilities }}>
      {children}
    </CapabilitiesContext.Provider>
  );
}

export function useCapabilities() {
  return useContext(CapabilitiesContext);
}

// ─── Keymap ──────────────────────────────────────────────────────

import { Keymap as CoreKeymap } from "@bettertui/core";
import type {
  BindingInfo,
  CommandContext,
  CommandHandler,
  KeymapEvent,
  KeymapOptions,
} from "@bettertui/core";

interface KeymapContextValue {
  keymap: CoreKeymap;
}

const KeymapContext = createContext<KeymapContextValue | null>(null);

export interface KeymapProviderProps {
  children: ReactNode;
  keymap?: CoreKeymap;
  options?: KeymapOptions;
}

export function KeymapProvider({ children, keymap: existing, options }: KeymapProviderProps) {
  const keymap = useMemo(() => existing ?? new CoreKeymap(undefined, options), [existing, options]);

  useEffect(() => {
    return () => {
      keymap.clearPending();
    };
  }, [keymap]);

  return <KeymapContext.Provider value={{ keymap }}>{children}</KeymapContext.Provider>;
}

export function useKeymap(): CoreKeymap {
  const ctx = useContext(KeymapContext);
  if (!ctx) {
    throw new Error("useKeymap must be used within a KeymapProvider");
  }
  return ctx.keymap;
}

export function useKeymapEvent(handler: (event: KeymapEvent) => void, deps: unknown[] = []): void {
  const keymap = useKeymap();
  const handlerRef = useRef(handler);
  handlerRef.current = handler;

  useEffect(() => {
    return keymap.on("state", (event: KeymapEvent) => {
      handlerRef.current(event);
    });
  }, [keymap, ...deps]);
}

export function useActiveBindings(): BindingInfo[] {
  const keymap = useKeymap();
  const [bindings, setBindings] = useState<BindingInfo[]>(() => keymap.activeBindings());

  useEffect(() => {
    const update = () => setBindings(keymap.activeBindings());
    return keymap.on("state", update);
  }, [keymap]);

  return bindings;
}

export function usePendingSequence(): { hasPending: boolean; keys: string[] } {
  const keymap = useKeymap();
  const [state, setState] = useState(() => ({
    hasPending: keymap.hasPending(),
    keys: keymap.pendingKeys(),
  }));

  useEffect(() => {
    const update = () =>
      setState({
        hasPending: keymap.hasPending(),
        keys: keymap.pendingKeys(),
      });
    return keymap.on("pendingSequence", update);
  }, [keymap]);

  return state;
}

export function useCommand(name: string, handler: CommandHandler): void {
  const keymap = useKeymap();
  const handlerRef = useRef(handler);
  handlerRef.current = handler;

  useEffect(() => {
    const stable = (ctx: unknown) => handlerRef.current(ctx as CommandContext);
    keymap.registerCommand(name, stable);
    return () => {
      keymap.unregisterCommand(name);
    };
  }, [keymap, name]);
}

export function useKeyIntercept(
  type: "key" | "key:after",
  handler: (key: string) => boolean,
): void {
  const keymap = useKeymap();
  const handlerRef = useRef(handler);
  handlerRef.current = handler;

  useEffect(() => {
    return keymap.intercept(type, (ctx) => {
      if (handlerRef.current(ctx.key)) {
        ctx.preventDefault();
      }
    });
  }, [keymap, type]);
}

export function useKeymapMode(mode: string): void {
  const keymap = useKeymap();

  useEffect(() => {
    keymap.setMode(mode);
    return () => {
      keymap.clearMode();
    };
  }, [keymap, mode]);
}

export { CoreKeymap as Keymap };
export type { KeymapEvent, KeymapOptions, CommandHandler, BindingInfo };
