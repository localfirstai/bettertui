import { act, renderHook } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { useClipboard, useKeyboard, useMouse, useResize } from "../hooks";

describe("useKeyboard", () => {
  it("calls handler on keydown event", () => {
    const handler = vi.fn(() => true);
    renderHook(() => useKeyboard(handler));

    const event = new KeyboardEvent("keydown", {
      key: "a",
      code: "KeyA",
      ctrlKey: false,
      shiftKey: false,
      altKey: false,
      metaKey: false,
    });
    const preventDefaultSpy = vi.spyOn(event, "preventDefault");

    act(() => {
      globalThis.dispatchEvent(event);
    });

    expect(handler).toHaveBeenCalledWith(
      expect.objectContaining({
        key: "a",
        code: "KeyA",
        ctrl: false,
        shift: false,
        alt: false,
        meta: false,
      }),
    );
    // When handler returns true, preventDefault is called
    expect(preventDefaultSpy).toHaveBeenCalled();
  });

  it("does not call preventDefault when handler returns false", () => {
    const handler = vi.fn(() => false);
    renderHook(() => useKeyboard(handler));

    const event = new KeyboardEvent("keydown", {
      key: "b",
      code: "KeyB",
      ctrlKey: false,
      shiftKey: false,
      altKey: false,
      metaKey: false,
    });
    const preventDefaultSpy = vi.spyOn(event, "preventDefault");

    act(() => {
      globalThis.dispatchEvent(event);
    });

    expect(handler).toHaveBeenCalled();
    expect(preventDefaultSpy).not.toHaveBeenCalled();
  });

  it("handles modifier keys", () => {
    const handler = vi.fn(() => true);
    renderHook(() => useKeyboard(handler));

    const event = new KeyboardEvent("keydown", {
      key: "c",
      code: "KeyC",
      ctrlKey: true,
      shiftKey: true,
      altKey: false,
      metaKey: true,
    });

    act(() => {
      globalThis.dispatchEvent(event);
    });

    expect(handler).toHaveBeenCalledWith(
      expect.objectContaining({
        key: "c",
        ctrl: true,
        shift: true,
        meta: true,
        alt: false,
      }),
    );
  });

  it("handles special keys", () => {
    const handler = vi.fn(() => true);
    renderHook(() => useKeyboard(handler));

    const event = new KeyboardEvent("keydown", {
      key: "Escape",
      code: "Escape",
    });

    act(() => {
      globalThis.dispatchEvent(event);
    });

    expect(handler).toHaveBeenCalledWith(
      expect.objectContaining({ key: "Escape", code: "Escape" }),
    );
  });

  it("uses latest handler reference", () => {
    const handler1 = vi.fn(() => true);
    const { rerender } = renderHook(
      (handler: (e: { key: string }) => boolean) => useKeyboard(handler),
      { initialProps: handler1 },
    );

    const event = new KeyboardEvent("keydown", { key: "x" });
    act(() => {
      globalThis.dispatchEvent(event);
    });
    expect(handler1).toHaveBeenCalled();

    const handler2 = vi.fn(() => true);
    rerender(handler2);

    act(() => {
      globalThis.dispatchEvent(event);
    });
    expect(handler2).toHaveBeenCalled();
  });
});

describe("useMouse", () => {
  it("calls handler on mousedown", () => {
    const handler = vi.fn(() => true);
    renderHook(() => useMouse(handler));

    const event = new MouseEvent("mousedown", {
      clientX: 100,
      clientY: 200,
      button: 0,
      shiftKey: false,
      ctrlKey: false,
      altKey: false,
    });
    const preventDefaultSpy = vi.spyOn(event, "preventDefault");

    act(() => {
      globalThis.dispatchEvent(event);
    });

    expect(handler).toHaveBeenCalledWith(
      expect.objectContaining({
        x: 100,
        y: 200,
        button: "left",
        pressed: true,
      }),
    );
    expect(preventDefaultSpy).toHaveBeenCalled();
  });

  it("calls handler on mouseup", () => {
    const handler = vi.fn(() => true);
    renderHook(() => useMouse(handler));

    const event = new MouseEvent("mouseup", {
      clientX: 50,
      clientY: 75,
      button: 2,
    });

    act(() => {
      globalThis.dispatchEvent(event);
    });

    expect(handler).toHaveBeenCalledWith(
      expect.objectContaining({
        x: 50,
        y: 75,
        button: "right",
        pressed: false,
      }),
    );
  });

  it("calls handler on mousemove (without preventDefault)", () => {
    const handler = vi.fn(() => true);
    renderHook(() => useMouse(handler));

    const event = new MouseEvent("mousemove", {
      clientX: 30,
      clientY: 40,
      button: 0,
    });

    act(() => {
      globalThis.dispatchEvent(event);
    });

    expect(handler).toHaveBeenCalledWith(
      expect.objectContaining({
        x: 30,
        y: 40,
        button: "other",
        pressed: false,
      }),
    );
  });

  it("handles middle button", () => {
    const handler = vi.fn(() => true);
    renderHook(() => useMouse(handler));

    const event = new MouseEvent("mousedown", {
      clientX: 0,
      clientY: 0,
      button: 1,
    });

    act(() => {
      globalThis.dispatchEvent(event);
    });

    expect(handler).toHaveBeenCalledWith(expect.objectContaining({ button: "middle" }));
  });

  it("handles unknown button number", () => {
    const handler = vi.fn(() => true);
    renderHook(() => useMouse(handler));

    const event = new MouseEvent("mousedown", {
      clientX: 0,
      clientY: 0,
      button: 5,
    });

    act(() => {
      globalThis.dispatchEvent(event);
    });

    expect(handler).toHaveBeenCalledWith(expect.objectContaining({ button: "other" }));
  });
});

describe("useClipboard", () => {
  it("starts with empty clipboard", () => {
    const { result } = renderHook(() => useClipboard());
    expect(result.current.clipboard).toBe("");
  });

  it("returns copy and paste as functions", () => {
    const { result } = renderHook(() => useClipboard());
    expect(typeof result.current.copy).toBe("function");
    expect(typeof result.current.paste).toBe("function");
  });
});

describe("useResize", () => {
  it("registers handler without process.stdout", () => {
    const handler = vi.fn();
    const { unmount } = renderHook(() => useResize(handler));
    expect(handler).not.toHaveBeenCalled();
    unmount();
  });
});
