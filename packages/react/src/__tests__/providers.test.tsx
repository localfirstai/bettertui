import type { Theme } from "@bettertui/shared";
import { act, renderHook } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import {
  CapabilitiesProvider,
  FocusProvider,
  Provider,
  SelectionProvider,
  TerminalProvider,
  useCapabilities,
  useFocus,
  useSelection,
  useTerminal,
  useTheme,
} from "../hooks";

describe("Provider nesting", () => {
  it("FocusProvider works without parent Provider", () => {
    const { result } = renderHook(() => useFocus(), {
      wrapper: ({ children }) => <FocusProvider>{children}</FocusProvider>,
    });
    expect(result.current.focusedId).toBeNull();
    act(() => result.current.setFocusedId("item-1"));
    expect(result.current.focusedId).toBe("item-1");
  });

  it("focusNext cycles through registered items (none registered)", () => {
    const { result } = renderHook(() => useFocus(), {
      wrapper: ({ children }) => <FocusProvider>{children}</FocusProvider>,
    });
    act(() => result.current.focusNext());
    expect(result.current.focusedId).toBeNull();
    act(() => result.current.focusNext());
    expect(result.current.focusedId).toBeNull();
  });

  it("focusPrevious cycles backward (none registered)", () => {
    const { result } = renderHook(() => useFocus(), {
      wrapper: ({ children }) => <FocusProvider>{children}</FocusProvider>,
    });
    act(() => result.current.focusPrevious());
    expect(result.current.focusedId).toBeNull();
  });

  it("setFocusedId with null clears focus", () => {
    const { result } = renderHook(() => useFocus(), {
      wrapper: ({ children }) => <FocusProvider>{children}</FocusProvider>,
    });
    act(() => result.current.setFocusedId("a"));
    expect(result.current.focusedId).toBe("a");
    act(() => result.current.setFocusedId(null));
    expect(result.current.focusedId).toBeNull();
  });

  it("FocusProvider with no Provider uses default theme", () => {
    const { result } = renderHook(() => ({ focus: useFocus(), theme: useTheme() }), {
      wrapper: ({ children }) => <FocusProvider>{children}</FocusProvider>,
    });
    expect(result.current.focus.focusedId).toBeNull();
    expect(result.current.theme.theme.name).toBe("dark");
  });

  it("CapabilitiesProvider outside Provider", () => {
    const { result } = renderHook(() => useCapabilities(), {
      wrapper: ({ children }) => <CapabilitiesProvider>{children}</CapabilitiesProvider>,
    });
    expect(result.current.capabilities.trueColor).toBe(false);
    expect(result.current.capabilities.kittyKeyboard).toBe(false);
  });

  it("updateCapabilities deep merges", () => {
    const { result } = renderHook(() => useCapabilities(), {
      wrapper: ({ children }) => <CapabilitiesProvider>{children}</CapabilitiesProvider>,
    });
    act(() => result.current.updateCapabilities({ trueColor: true }));
    expect(result.current.capabilities.trueColor).toBe(true);
    expect(result.current.capabilities.sgrMouse).toBe(false);
    act(() => result.current.updateCapabilities({ bracketedPaste: true }));
    expect(result.current.capabilities.bracketedPaste).toBe(true);
    expect(result.current.capabilities.trueColor).toBe(true);
  });

  it("SelectionProvider with setSelection(null)", () => {
    const { result } = renderHook(() => useSelection(), {
      wrapper: ({ children }) => <SelectionProvider>{children}</SelectionProvider>,
    });
    expect(result.current.selection).toBeNull();
    act(() => result.current.setSelection({ start: { x: 0, y: 0 }, end: { x: 5, y: 5 } }));
    expect(result.current.selection).toEqual({ start: { x: 0, y: 0 }, end: { x: 5, y: 5 } });
    act(() => result.current.setSelection(null));
    expect(result.current.selection).toBeNull();
  });

  it("TerminalProvider with default size", () => {
    const { result } = renderHook(() => useTerminal(), {
      wrapper: ({ children }) => <TerminalProvider>{children}</TerminalProvider>,
    });
    expect(result.current.width).toBe(80);
    expect(result.current.height).toBe(24);
  });

  it("TerminalProvider resize updates size", () => {
    const { result } = renderHook(() => useTerminal(), {
      wrapper: ({ children }) => <TerminalProvider>{children}</TerminalProvider>,
    });
    act(() => result.current.resize(120, 40));
    expect(result.current.width).toBe(120);
    expect(result.current.height).toBe(40);
    act(() => result.current.resize(80, 24));
    expect(result.current.width).toBe(80);
    expect(result.current.height).toBe(24);
  });

  it("all providers compose without crashing", () => {
    const { result } = renderHook(
      () => ({
        theme: useTheme(),
        focus: useFocus(),
        terminal: useTerminal(),
        selection: useSelection(),
        capabilities: useCapabilities(),
      }),
      {
        wrapper: ({ children }) => (
          <Provider>
            <FocusProvider>
              <TerminalProvider>
                <CapabilitiesProvider>
                  <SelectionProvider>{children}</SelectionProvider>
                </CapabilitiesProvider>
              </TerminalProvider>
            </FocusProvider>
          </Provider>
        ),
      },
    );
    expect(result.current.theme.theme.name).toBe("dark");
    expect(result.current.focus.focusedId).toBeNull();
    expect(result.current.terminal.width).toBe(80);
    expect(result.current.selection.selection).toBeNull();
    expect(result.current.capabilities.capabilities.trueColor).toBe(false);
  });
});

describe("Theme switching scenarios", () => {
  it("switching from dark to light theme", () => {
    const lightTheme: Partial<Theme> = {
      name: "light",
      colors: {
        background: "#ffffff",
        surface: "#f5f5f5",
        surfaceHigh: "#eeeeee",
        surfaceLow: "#fafafa",
        primary: "#1a73e8",
        primaryForeground: "#ffffff",
        secondary: "#5f6368",
        secondaryForeground: "#ffffff",
        text: "#202124",
        textMuted: "#5f6368",
        textDim: "#808080",
        border: "#dadce0",
        borderFocused: "#1a73e8",
        accent: "#1a73e8",
        accentForeground: "#ffffff",
        error: "#d93025",
        warning: "#f9ab00",
        success: "#1e8e3e",
        info: "#1a73e8",
        scrollbar: "#dadce0",
        scrollbarThumb: "#808080",
      },
      spacing: { none: 0, xxs: 1, xs: 2, sm: 4, md: 8, lg: 12, xl: 16, xxl: 24 },
      borders: { style: "solid" as const, fg: "#dadce0" },
    };

    const { result } = renderHook(() => useTheme(), {
      wrapper: ({ children }) => <Provider>{children}</Provider>,
    });

    expect(result.current.theme.name).toBe("dark");
    act(() => result.current.setTheme(lightTheme));
    expect(result.current.theme.name).toBe("light");
    expect(result.current.theme.colors.background).toBe("#ffffff");
  });
});
