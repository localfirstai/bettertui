import { createContext, useCallback, useContext, useEffect, useRef, useState } from "react";
import type { ReactNode } from "react";

// Theme types
export interface ThemeColors {
  background: string;
  surface: string;
  surfaceHigh: string;
  surfaceLow: string;
  primary: string;
  primaryForeground: string;
  secondary: string;
  secondaryForeground: string;
  text: string;
  textMuted: string;
  textDim: string;
  border: string;
  borderFocused: string;
  accent: string;
  accentForeground: string;
  error: string;
  warning: string;
  success: string;
  info: string;
}

export interface ThemeSpacing {
  none: number;
  xxs: number;
  xs: number;
  sm: number;
  md: number;
  lg: number;
  xl: number;
  xxl: number;
}

export interface Theme {
  name: string;
  colors: ThemeColors;
  spacing: ThemeSpacing;
}

// Default dark theme
const defaultDarkTheme: Theme = {
  name: "dark",
  colors: {
    background: "#1e1e28",
    surface: "#1e1e28",
    surfaceHigh: "#282837",
    surfaceLow: "#14141c",
    primary: "#648cdc",
    primaryForeground: "#ffffff",
    secondary: "#8c64c8",
    secondaryForeground: "#ffffff",
    text: "#dcdce6",
    textMuted: "#8c8ca0",
    textDim: "#5a5a69",
    border: "#3c3c50",
    borderFocused: "#648cdc",
    accent: "#50c8a0",
    accentForeground: "#ffffff",
    error: "#dc5050",
    warning: "#dcb43c",
    success: "#50c878",
    info: "#50a0dc",
  },
  spacing: {
    none: 0,
    xxs: 1,
    xs: 2,
    sm: 4,
    md: 8,
    lg: 12,
    xl: 16,
    xxl: 24,
  },
};

// Theme context
interface ThemeContextValue {
  theme: Theme;
  setTheme: (theme: Theme) => void;
}

const ThemeContext = createContext<ThemeContextValue>({
  theme: defaultDarkTheme,
  setTheme: () => {},
});

// Theme provider
export interface ProviderProps {
  children: ReactNode;
  theme?: Theme;
}

export function Provider({ children, theme = defaultDarkTheme }: ProviderProps) {
  const [currentTheme, setCurrentTheme] = useState<Theme>(theme);

  const setTheme = useCallback((newTheme: Theme) => {
    setCurrentTheme(newTheme);
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
export interface KeyEvent {
  key: string;
  ctrl: boolean;
  shift: boolean;
  alt: boolean;
  meta: boolean;
}

export function useKeyboard(handler: (event: KeyEvent) => boolean) {
  const handlerRef = useRef(handler);
  handlerRef.current = handler;

  useEffect(() => {
    const handleKeyDown = (event: globalThis.KeyboardEvent) => {
      const ke = event as unknown as {
        key: string;
        ctrlKey: boolean;
        shiftKey: boolean;
        altKey: boolean;
        metaKey: boolean;
      };
      const keyEvent: KeyEvent = {
        key: ke.key,
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

// Animation
export function useAnimation(
  callback: (progress: number) => void,
  duration: number,
  deps: unknown[] = [],
) {
  const callbackRef = useRef(callback);
  callbackRef.current = callback;

  useEffect(() => {
    let animationFrame: number;
    let startTime: number;

    const animate = (currentTime: number) => {
      if (!startTime) startTime = currentTime;
      const elapsed = currentTime - startTime;
      const progress = Math.min(elapsed / duration, 1);

      callbackRef.current(progress);

      if (progress < 1) {
        animationFrame = _raf(animate);
      }
    };

    animationFrame = _raf(animate);

    return () => {
      if (animationFrame) {
        _caf(animationFrame);
      }
    };
  }, [duration, ...deps]);
}
