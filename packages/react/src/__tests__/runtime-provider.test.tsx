import { CommandRuntime } from "@bettertui/core";
import { act, renderHook } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { RuntimeProvider, useRuntime } from "../runtime";

describe("RuntimeProvider", () => {
  it("provides runtime via useRuntime", () => {
    const runtime = new CommandRuntime();
    const { result } = renderHook(() => useRuntime(), {
      wrapper: ({ children }) => <RuntimeProvider runtime={runtime}>{children}</RuntimeProvider>,
    });
    expect(result.current).not.toBeNull();
    expect(result.current?.runtime).toBe(runtime);
  });

  it("useRuntime returns null outside provider", () => {
    const { result } = renderHook(() => useRuntime());
    expect(result.current).toBeNull();
  });

  it("onKey registers and unregisters handlers", () => {
    const runtime = new CommandRuntime();
    const handler = vi.fn();
    const { result } = renderHook(() => useRuntime(), {
      wrapper: ({ children }) => <RuntimeProvider runtime={runtime}>{children}</RuntimeProvider>,
    });
    let unsub: (() => void) | undefined;
    act(() => {
      unsub = result.current?.onKey(handler);
    });
    expect(typeof unsub).toBe("function");
    if (unsub) {
      act(() => unsub?.());
    }
  });
});
