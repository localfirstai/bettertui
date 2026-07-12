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
  useFrame,
  useSelection,
  useTerminal,
  useTheme,
} from "../hooks";

describe("useTheme", () => {
  it("returns default theme when no Provider", () => {
    const { result } = renderHook(() => useTheme());
    expect(result.current.theme.name).toBe("dark");
    expect(typeof result.current.setTheme).toBe("function");
  });

  it("returns theme from Provider", () => {
    const customTheme = {
      name: "custom",
      colors: {},
      spacing: {},
      borders: {},
    } as Theme;
    const { result } = renderHook(() => useTheme(), {
      wrapper: ({ children }) => <Provider theme={customTheme}>{children}</Provider>,
    });
    expect(result.current.theme.name).toBe("custom");
  });

  it("setTheme updates the theme", () => {
    const newTheme = { name: "updated", colors: {}, spacing: {}, borders: {} } as Theme;
    const { result } = renderHook(() => useTheme(), {
      wrapper: ({ children }) => <Provider>{children}</Provider>,
    });
    act(() => result.current.setTheme(newTheme));
    expect(result.current.theme.name).toBe("updated");
  });

  it("can nest themes and children see the innermost", () => {
    const outer = { name: "outer", colors: {}, spacing: {}, borders: {} } as Theme;
    const inner = { name: "inner", colors: {}, spacing: {}, borders: {} } as Theme;
    const { result } = renderHook(() => useTheme(), {
      wrapper: ({ children }) => (
        <Provider theme={outer}>
          <Provider theme={inner}>{children}</Provider>
        </Provider>
      ),
    });
    expect(result.current.theme.name).toBe("inner");
  });
});

describe("useFocus", () => {
  it("returns default focus state outside provider", () => {
    const { result } = renderHook(() => useFocus());
    expect(result.current.focusedId).toBeNull();
    expect(typeof result.current.setFocusedId).toBe("function");
    expect(typeof result.current.focusNext).toBe("function");
    expect(typeof result.current.focusPrevious).toBe("function");
  });

  it("setFocusedId updates focusedId", () => {
    const { result } = renderHook(() => useFocus(), {
      wrapper: ({ children }) => <FocusProvider>{children}</FocusProvider>,
    });
    act(() => result.current.setFocusedId("item-1"));
    expect(result.current.focusedId).toBe("item-1");
  });

  it("setFocusedId to null clears focus", () => {
    const { result } = renderHook(() => useFocus(), {
      wrapper: ({ children }) => <FocusProvider>{children}</FocusProvider>,
    });
    act(() => result.current.setFocusedId("item-1"));
    act(() => result.current.setFocusedId(null));
    expect(result.current.focusedId).toBeNull();
  });

  it("focusNext cycles forward through focusable ids (none registered)", () => {
    const { result } = renderHook(() => useFocus(), {
      wrapper: ({ children }) => <FocusProvider>{children}</FocusProvider>,
    });
    act(() => result.current.focusNext());
    expect(result.current.focusedId).toBeNull();
  });

  it("focusPrevious cycles backward through focusable ids (none registered)", () => {
    const { result } = renderHook(() => useFocus(), {
      wrapper: ({ children }) => <FocusProvider>{children}</FocusProvider>,
    });
    act(() => result.current.focusPrevious());
    expect(result.current.focusedId).toBeNull();
  });
});

describe("useTerminal", () => {
  it("returns default terminal size", () => {
    const { result } = renderHook(() => useTerminal());
    expect(result.current.width).toBe(80);
    expect(result.current.height).toBe(24);
  });

  it("resize updates terminal size", () => {
    const { result } = renderHook(() => useTerminal(), {
      wrapper: ({ children }) => <TerminalProvider>{children}</TerminalProvider>,
    });
    act(() => result.current.resize(120, 40));
    expect(result.current.width).toBe(120);
    expect(result.current.height).toBe(40);
  });

  it("resize is a function", () => {
    const { result } = renderHook(() => useTerminal());
    expect(typeof result.current.resize).toBe("function");
  });
});

describe("useFrame", () => {
  it("starts with frameRequested false", () => {
    const { result } = renderHook(() => useFrame());
    expect(result.current.frameRequested).toBe(false);
  });

  it("requestFrame is a function", () => {
    const { result } = renderHook(() => useFrame());
    expect(typeof result.current.requestFrame).toBe("function");
  });

  it("requestFrame toggles frameRequested", () => {
    const { result } = renderHook(() => useFrame());
    act(() => {
      result.current.requestFrame();
    });
    expect(result.current.frameRequested).toBe(false);
  });
});

describe("useCapabilities", () => {
  it("returns default capabilities outside provider", () => {
    const { result } = renderHook(() => useCapabilities());
    expect(result.current.capabilities.kittyKeyboard).toBe(false);
    expect(result.current.capabilities.trueColor).toBe(false);
    expect(result.current.capabilities.bracketedPaste).toBe(false);
  });

  it("updateCapabilities merges capabilities", () => {
    const { result } = renderHook(() => useCapabilities(), {
      wrapper: ({ children }) => <CapabilitiesProvider>{children}</CapabilitiesProvider>,
    });
    act(() => result.current.updateCapabilities({ trueColor: true, kittyKeyboard: true }));
    expect(result.current.capabilities.trueColor).toBe(true);
    expect(result.current.capabilities.kittyKeyboard).toBe(true);
    expect(result.current.capabilities.sgrMouse).toBe(false);
  });

  it("updateCapabilities preserves existing capabilities", () => {
    const { result } = renderHook(() => useCapabilities(), {
      wrapper: ({ children }) => <CapabilitiesProvider>{children}</CapabilitiesProvider>,
    });
    act(() => result.current.updateCapabilities({ bracketedPaste: true }));
    expect(result.current.capabilities.bracketedPaste).toBe(true);
    expect(result.current.capabilities.trueColor).toBe(false);
  });
});

describe("useSelection", () => {
  it("returns default selection state outside provider", () => {
    const { result } = renderHook(() => useSelection());
    expect(result.current.selection).toBeNull();
    expect(result.current.getSelectedText()).toBe("");
  });

  it("setSelection updates selection", () => {
    const sel = { start: { x: 0, y: 0 }, end: { x: 10, y: 5 } };
    const { result } = renderHook(() => useSelection(), {
      wrapper: ({ children }) => <SelectionProvider>{children}</SelectionProvider>,
    });
    act(() => result.current.setSelection(sel));
    expect(result.current.selection).toEqual(sel);
  });

  it("setSelection to null clears selection", () => {
    const sel = { start: { x: 0, y: 0 }, end: { x: 10, y: 5 } };
    const { result } = renderHook(() => useSelection(), {
      wrapper: ({ children }) => <SelectionProvider>{children}</SelectionProvider>,
    });
    act(() => result.current.setSelection(sel));
    act(() => result.current.setSelection(null));
    expect(result.current.selection).toBeNull();
  });
});

describe("Provider composition", () => {
  it("all providers can be composed together", () => {
    const { result } = renderHook(
      () => ({ theme: useTheme(), focus: useFocus(), terminal: useTerminal() }),
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
  });
});
