import { beforeEach, describe, expect, it, vi } from "vitest";

vi.mock("bettertui_bindings", () => {
  return {
    NapiEngine: class {
      processCommands() {
        return JSON.stringify({ success: 1, errors: [], idMappings: [] });
      }
      render() {
        return { outputData: new Uint8Array(0), width: 80, height: 24, dirtyRegionCount: 0 };
      }
      renderFull() {
        return { outputData: new Uint8Array(0), width: 80, height: 24, dirtyRegionCount: 0 };
      }
      resize() {}
      beginFrame() {}
      commitFrame() {}
      nodeCount() {
        return 0;
      }
      frameCount() {
        return "0";
      }
      treeSummary() {
        return "";
      }
      printTree() {
        return "";
      }
      validate() {
        return true;
      }
      root() {
        return "0";
      }
      generation() {
        return "0";
      }
      createNode() {
        return 0;
      }
      appendChild() {
        return true;
      }
      removeNode() {}
      setText() {}
      shutdown() {}
      dimensions() {
        return [80, 24];
      }
      shouldRender() {
        return "";
      }
      requestFrame() {}
    },
    NapiEventBus: class {
      pushKey() {}
      pushMouse() {}
      pushMouseMotion() {}
      pushPaste() {}
      pushResize() {}
      drain() {
        return "[]";
      }
      len() {
        return 0;
      }
      isEmpty() {
        return true;
      }
      clear() {}
    },
    NapiFocusManager: class {
      focus() {
        return true;
      }
      blur() {
        return true;
      }
      blurCurrent() {
        return true;
      }
      focused() {
        return 0;
      }
      focusedInScope() {
        return null;
      }
      traverse() {
        return 0;
      }
      setScope() {}
      clearScope() {}
      scopeId() {
        return null;
      }
      focusOrder() {
        return [];
      }
      isFocused() {
        return false;
      }
    },
    NapiTextEngine: class {
      insertText() {}
      deleteCharBackward() {}
      deleteCharForward() {}
      deleteWordBackward() {}
      deleteWordForward() {}
      deleteLineBackward() {}
      deleteLineForward() {}
      cursorLeft() {}
      cursorRight() {}
      cursorUp() {}
      cursorDown() {}
      cursorLineStart() {}
      cursorLineEnd() {}
      cursorPosition() {
        return 0;
      }
      setCursorPosition() {}
      text() {
        return "";
      }
      insertAt() {}
      deleteAt() {
        return "";
      }
      charAt() {
        return "";
      }
      substring() {
        return "";
      }
      find() {
        return [];
      }
      replaceAll() {
        return 0;
      }
      undo() {
        return true;
      }
      redo() {
        return true;
      }
      canUndo() {
        return false;
      }
      canRedo() {
        return false;
      }
      clear() {}
      length() {
        return 0;
      }
      isEmpty() {
        return true;
      }
      lines() {
        return [];
      }
      lineCount() {
        return 0;
      }
    },
    NapiScheduler: class {
      beginFrame() {
        return true;
      }
      endFrame() {}
      requestFrame() {}
      frameCount() {
        return "0";
      }
      droppedFrames() {
        return "0";
      }
      fps() {
        return "60";
      }
      frameBudgetMs() {
        return "16";
      }
      isIdle() {
        return true;
      }
    },
    detectCapabilities() {
      return JSON.stringify({
        terminalSize: { columns: 80, rows: 24 },
        pixelSize: null,
        brand: "test",
        trueColor: true,
        mouse: true,
        bracketedPaste: false,
        sync: false,
        sgrPixel: false,
        kittyKeyboard: false,
        csi_u: false,
        focusEvents: false,
        osc8: false,
        underlineColor: false,
        strikethrough: false,
        cursorStyle: false,
        alternateScroll: false,
        hyperlinks: false,
        inlineImages: false,
        sixel: false,
      });
    },
    getVersion() {
      return "0.0.0-test";
    },
  };
});
import { CommandBuffer } from "../command-buffer";
import {
  createEngine,
  createEventBus,
  createFocusManager,
  createScheduler,
  createTextEngine,
  detectCapabilities,
  getVersion,
} from "../platform";
import { createEventLoop } from "../platform/events";
import { createRuntime } from "../platform/runtime";
import type { NapiEngine, NapiEventBus } from "../platform/types";

function createMockEngine(overrides: Partial<NapiEngine> = {}): NapiEngine {
  return {
    processCommands: vi.fn(() => JSON.stringify({ success: 1, errors: [], idMappings: [] })),
    render: vi.fn(() => ({
      outputData: new Uint8Array([72, 73]),
      width: 80,
      height: 24,
      dirtyRegionCount: 1,
    })),
    renderFull: vi.fn(() => ({
      outputData: new Uint8Array([72, 73]),
      width: 80,
      height: 24,
      dirtyRegionCount: 1,
    })),
    resize: vi.fn(),
    beginFrame: vi.fn(),
    commitFrame: vi.fn(),
    nodeCount: vi.fn(() => 5),
    frameCount: vi.fn(() => "42"),
    treeSummary: vi.fn(() => "root"),
    printTree: vi.fn(() => ""),
    validate: vi.fn(() => true),
    root: vi.fn(() => "0"),
    generation: vi.fn(() => "1"),
    createNode: vi.fn(() => 1),
    appendChild: vi.fn(() => true),
    removeNode: vi.fn(),
    setText: vi.fn(),
    shutdown: vi.fn(),
    dimensions: vi.fn(() => [80, 24]),
    shouldRender: vi.fn(() => "true"),
    requestFrame: vi.fn(),
    ...overrides,
  } as NapiEngine;
}

function createMockEventBus(overrides: Partial<NapiEventBus> = {}): NapiEventBus {
  return {
    pushKey: vi.fn(),
    pushMouse: vi.fn(),
    pushMouseMotion: vi.fn(),
    pushPaste: vi.fn(),
    pushResize: vi.fn(),
    drain: vi.fn(() => "[]"),
    len: vi.fn(() => 0),
    isEmpty: vi.fn(() => true),
    clear: vi.fn(),
    ...overrides,
  } as NapiEventBus;
}

describe("createRuntime", () => {
  let engine: NapiEngine;
  let eventBus: NapiEventBus;
  let buffer: CommandBuffer;

  beforeEach(() => {
    engine = createMockEngine();
    eventBus = createMockEventBus();
    buffer = new CommandBuffer();
  });

  it("returns Runtime interface with all methods", () => {
    const runtime = createRuntime(engine, eventBus, buffer);
    expect(runtime.engine).toBe(engine);
    expect(runtime.eventBus).toBe(eventBus);
    expect(runtime.buffer).toBe(buffer);
    expect(typeof runtime.processCommands).toBe("function");
    expect(typeof runtime.renderFrame).toBe("function");
    expect(typeof runtime.resize).toBe("function");
    expect(typeof runtime.shutdown).toBe("function");
  });

  it("processCommands drains buffer and sends to engine", () => {
    buffer.push({ type: "CreateNode", id: "0", kind: "Box" });
    const runtime = createRuntime(engine, eventBus, buffer);
    const result = runtime.processCommands();
    expect(result.success).toBe(1);
    expect(engine.processCommands).toHaveBeenCalledOnce();
  });

  it("processCommands returns success 0 when buffer is empty", () => {
    const runtime = createRuntime(engine, eventBus, buffer);
    const result = runtime.processCommands();
    expect(result.success).toBe(0);
    expect(engine.processCommands).not.toHaveBeenCalled();
  });

  it("processCommands logs engine errors", () => {
    const consoleSpy = vi.spyOn(console, "error").mockImplementation(() => {});
    const errEngine = createMockEngine({
      processCommands: vi.fn(() =>
        JSON.stringify({
          success: 0,
          errors: ["something went wrong"],
          idMappings: [],
        }),
      ),
    });
    buffer.push({ type: "CreateNode", id: "0", kind: "Box" });
    const runtime = createRuntime(errEngine, eventBus, buffer);
    runtime.processCommands();
    expect(consoleSpy).toHaveBeenCalledWith("ENGINE ERRORS:", ["something went wrong"]);
  });

  it("renderFrame calls beginFrame, render, commitFrame", () => {
    const runtime = createRuntime(engine, eventBus, buffer);
    const result = runtime.renderFrame();
    expect(engine.beginFrame).toHaveBeenCalledOnce();
    expect(engine.render).toHaveBeenCalledOnce();
    expect(engine.commitFrame).toHaveBeenCalledOnce();
    expect(result.outputData).toEqual(new Uint8Array([72, 73]));
    expect(result.width).toBe(80);
    expect(result.height).toBe(24);
    expect(result.dirtyRegionCount).toBe(1);
  });

  it("resize calls engine.resize", () => {
    const runtime = createRuntime(engine, eventBus, buffer);
    runtime.resize(120, 40);
    expect(engine.resize).toHaveBeenCalledWith(120, 40);
  });

  it("shutdown pushes Shutdown command and shuts down engine", () => {
    const runtime = createRuntime(engine, eventBus, buffer);
    runtime.shutdown();
    expect(buffer.isEmpty).toBe(true);
    expect(engine.processCommands).toHaveBeenCalled();
    expect(engine.shutdown).toHaveBeenCalledOnce();
  });
});

describe("toEngineJson (via processCommands)", () => {
  it("converts string IDs to numbers in commands", () => {
    const engine = createMockEngine();
    const eventBus = createMockEventBus();
    const buffer = new CommandBuffer();
    buffer.push({ type: "AppendChild", parent: "5", child: "10" });
    const runtime = createRuntime(engine, eventBus, buffer);
    runtime.processCommands();
    const callArg = (engine.processCommands as ReturnType<typeof vi.fn>).mock
      .calls[0]?.[0] as string;
    const parsed = JSON.parse(callArg);
    expect(parsed[0]?.parent).toBe(5);
    expect(parsed[0]?.child).toBe(10);
  });

  it("preserves non-ID keys as-is", () => {
    const engine = createMockEngine();
    const eventBus = createMockEventBus();
    const buffer = new CommandBuffer();
    buffer.push({ type: "SetStyle", id: "3", style: { bold: true } });
    const runtime = createRuntime(engine, eventBus, buffer);
    runtime.processCommands();
    const callArg = (engine.processCommands as ReturnType<typeof vi.fn>).mock
      .calls[0]?.[0] as string;
    const parsed = JSON.parse(callArg);
    expect(parsed[0]?.id).toBe(3);
    expect(parsed[0]?.style).toEqual({ bold: true });
  });
});

describe("createEventLoop", () => {
  let eventBus: NapiEventBus;

  beforeEach(() => {
    eventBus = createMockEventBus();
  });

  it("returns EventLoop interface", () => {
    const loop = createEventLoop(eventBus);
    expect(typeof loop.start).toBe("function");
    expect(typeof loop.stop).toBe("function");
    expect(typeof loop.pushKey).toBe("function");
    expect(typeof loop.pushMouse).toBe("function");
    expect(typeof loop.drain).toBe("function");
    expect(typeof loop.onEvent).toBe("function");
  });

  it("pushKey calls eventBus.pushKey and triggers callbacks", () => {
    const loop = createEventLoop(eventBus);
    const cb = vi.fn();
    loop.onEvent(cb);
    loop.pushKey("a", false, false, false, 1);
    expect(eventBus.pushKey).toHaveBeenCalledWith("a", false, false, false, 1);
    expect(cb).toHaveBeenCalledWith(
      expect.objectContaining({ key: "a", ctrl: false, meta: false }),
    );
  });

  it("pushMouse calls eventBus.pushMouse and triggers callbacks", () => {
    const loop = createEventLoop(eventBus);
    const cb = vi.fn();
    loop.onEvent(cb);
    loop.pushMouse("left", 10, 20, 1);
    expect(eventBus.pushMouse).toHaveBeenCalledWith("left", 10, 20, 1);
    expect(cb).toHaveBeenCalledWith(
      expect.objectContaining({
        button: "left",
        position: { x: 10, y: 20 },
      }),
    );
  });

  it("drain delegates to eventBus.drain", () => {
    const bus = createMockEventBus({ drain: vi.fn(() => "some data") });
    const loop = createEventLoop(bus);
    expect(loop.drain()).toBe("some data");
  });

  it("start and stop work", () => {
    const loop = createEventLoop(eventBus);
    loop.start();
    loop.stop();
  });

  it("start is idempotent", () => {
    const loop = createEventLoop(eventBus);
    loop.start();
    loop.start();
    loop.stop();
  });

  it("start interval drains events and calls callbacks", () => {
    vi.useFakeTimers();
    const bus = createMockEventBus({
      drain: vi
        .fn()
        .mockReturnValueOnce(
          '[{"key":"a","code":"","ctrl":false,"shift":false,"alt":false,"meta":false}]',
        )
        .mockReturnValueOnce(""),
    });
    const loop = createEventLoop(bus);
    const cb = vi.fn();
    loop.onEvent(cb);
    loop.start();
    vi.advanceTimersByTime(16);
    expect(cb).toHaveBeenCalledWith(expect.objectContaining({ key: "a" }));
    vi.advanceTimersByTime(16);
    expect(cb).toHaveBeenCalledTimes(1);
    loop.stop();
    vi.useRealTimers();
  });

  it("start interval handles malformed JSON gracefully", () => {
    vi.useFakeTimers();
    const bus = createMockEventBus({
      drain: vi.fn().mockReturnValue("not valid json"),
    });
    const loop = createEventLoop(bus);
    const cb = vi.fn();
    loop.onEvent(cb);
    loop.start();
    vi.advanceTimersByTime(16);
    expect(cb).not.toHaveBeenCalled();
    loop.stop();
    vi.useRealTimers();
  });
});

describe.skip("native factory functions", () => {
  it("createEngine returns an engine instance", () => {
    const engine = createEngine(80, 24);
    expect(engine).toBeDefined();
    expect(typeof engine.processCommands).toBe("function");
  });

  it("createEngine works without dimensions", () => {
    const engine = createEngine();
    expect(engine).toBeDefined();
  });

  it("createEventBus returns an event bus", () => {
    const bus = createEventBus();
    expect(bus).toBeDefined();
    expect(typeof bus.drain).toBe("function");
  });

  it("createFocusManager returns a focus manager", () => {
    const fm = createFocusManager();
    expect(fm).toBeDefined();
    expect(typeof fm.focus).toBe("function");
  });

  it("createTextEngine returns a text engine", () => {
    const te = createTextEngine();
    expect(te).toBeDefined();
    expect(typeof te.insertText).toBe("function");
  });

  it("createScheduler returns a scheduler", () => {
    const s = createScheduler();
    expect(s).toBeDefined();
    expect(typeof s.beginFrame).toBe("function");
  });

  it("detectCapabilities returns terminal capabilities", () => {
    const caps = detectCapabilities();
    expect(caps).toBeDefined();
    expect(typeof caps).toBe("object");
  });

  it("getVersion returns version string", () => {
    const version = getVersion();
    expect(typeof version).toBe("string");
  });
});
