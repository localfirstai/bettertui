import { describe, expect, it } from "vitest";
import {
  CapabilityInspector,
  CommandInspector,
  EventInspector,
  FocusInspector,
  Logger,
  PerformanceTracker,
  SchedulerInspector,
  SnapshotManager,
  Timeline,
  TreeInspector,
  createDevTools,
  createExport,
  createSummary,
  exportToJson,
} from "../index";

// ─── createDevTools ──────────────────────────────────────────────────────────

describe("createDevTools", () => {
  it("returns no-op DevTools when called with no options", () => {
    const devtools = createDevTools();
    expect(devtools.enabled).toBe(false);
  });

  it("returns no-op DevTools when enabled is false", () => {
    const devtools = createDevTools({ enabled: false });
    expect(devtools.enabled).toBe(false);
  });

  it("returns no-op DevTools when enabled is undefined", () => {
    const devtools = createDevTools({ enabled: undefined });
    expect(devtools.enabled).toBe(false);
  });

  it("returns enabled DevTools when enabled is true", () => {
    const devtools = createDevTools({ enabled: true });
    expect(devtools.enabled).toBe(true);
  });

  it("no-op recordCommand does not throw", () => {
    const devtools = createDevTools();
    expect(() => devtools.recordCommand("CreateNode", { id: "1" })).not.toThrow();
  });

  it("no-op recordFrame does not throw", () => {
    const devtools = createDevTools();
    expect(() => devtools.recordFrame({ duration: 16 })).not.toThrow();
  });

  it("no-op recordKeyboard does not throw", () => {
    const devtools = createDevTools();
    expect(() =>
      devtools.recordKeyboard("a", { ctrl: false, shift: false, alt: false, meta: false }),
    ).not.toThrow();
  });

  it("no-op recordMouse does not throw", () => {
    const devtools = createDevTools();
    expect(() => devtools.recordMouse("click", 10, 20, "left")).not.toThrow();
  });

  it("no-op recordFocus does not throw", () => {
    const devtools = createDevTools();
    expect(() => devtools.recordFocus("focus", "node-1")).not.toThrow();
  });

  it("no-op recordResize does not throw", () => {
    const devtools = createDevTools();
    expect(() => devtools.recordResize(120, 40)).not.toThrow();
  });

  it("no-op exportData returns valid export", () => {
    const devtools = createDevTools();
    const data = devtools.exportData();
    expect(data.version).toBe("1.0.0");
    expect(data.commands).toHaveLength(0);
  });

  it("no-op exportJson returns valid JSON", () => {
    const devtools = createDevTools();
    const json = devtools.exportJson();
    expect(() => JSON.parse(json)).not.toThrow();
  });

  it("no-op getSummary returns string", () => {
    const devtools = createDevTools();
    expect(typeof devtools.getSummary()).toBe("string");
  });

  it("no-op reset does not throw", () => {
    const devtools = createDevTools();
    expect(() => devtools.reset()).not.toThrow();
  });

  it("no-op dispose does not throw", () => {
    const devtools = createDevTools();
    expect(() => devtools.dispose()).not.toThrow();
  });

  it("no-op updateCapabilities does not throw", () => {
    const devtools = createDevTools();
    expect(() => devtools.updateCapabilities({ trueColor: true })).not.toThrow();
  });

  it("no-op updateScheduler does not throw", () => {
    const devtools = createDevTools();
    expect(() => devtools.updateScheduler({ frameCount: 1 })).not.toThrow();
  });

  it("no-op captureSnapshot returns 0", () => {
    const devtools = createDevTools();
    const tree = { id: "root", type: "Box", props: {}, children: [] };
    expect(devtools.captureSnapshot(tree)).toBe(0);
  });

  // ─── Enabled DevTools ──────────────────────────────────────────────────

  it("enabled DevTools records commands", () => {
    const devtools = createDevTools({ enabled: true });
    devtools.recordCommand("CreateNode", { id: "1", kind: "Box" });
    expect(devtools.commands.getTotalCount()).toBe(1);
  });

  it("enabled DevTools records keyboard events", () => {
    const devtools = createDevTools({ enabled: true });
    devtools.recordKeyboard("a", { ctrl: false, shift: false, alt: false, meta: false });
    expect(devtools.events.count).toBe(1);
  });

  it("enabled DevTools records mouse events", () => {
    const devtools = createDevTools({ enabled: true });
    devtools.recordMouse("click", 10, 20, "left");
    expect(devtools.events.count).toBe(1);
  });

  it("enabled DevTools records focus events", () => {
    const devtools = createDevTools({ enabled: true });
    devtools.recordFocus("focus", "node-1");
    expect(devtools.focus.getSnapshot().focusedNodeId).toBe("node-1");
  });

  it("enabled DevTools records blur events through recordFocus", () => {
    const devtools = createDevTools({ enabled: true });
    devtools.recordFocus("focus", "node-1");
    devtools.recordFocus("blur", "node-1");
    expect(devtools.focus.getSnapshot().focusedNodeId).toBeNull();
  });

  it("enabled DevTools records keyboard events with target", () => {
    const devtools = createDevTools({ enabled: true });
    devtools.recordKeyboard(
      "Enter",
      { ctrl: false, shift: false, alt: false, meta: false },
      "btn-1",
    );
    expect(devtools.events.count).toBe(1);
  });

  it("enabled DevTools records mouse events with button and target", () => {
    const devtools = createDevTools({ enabled: true });
    devtools.recordMouse("mousedown", 50, 100, "right", "btn-2");
    expect(devtools.events.count).toBe(1);
  });

  it("enabled DevTools records resize events", () => {
    const devtools = createDevTools({ enabled: true });
    devtools.recordResize(120, 40, 80, 24);
    expect(devtools.events.count).toBe(1);
  });

  it("enabled DevTools records frame metrics", () => {
    const devtools = createDevTools({ enabled: true });
    devtools.recordFrame({ duration: 16.5, commandCount: 10, dirtyRegionCount: 3 });
    expect(devtools.performance.count).toBe(1);
  });

  it("enabled DevTools updates capabilities", () => {
    const devtools = createDevTools({ enabled: true });
    devtools.updateCapabilities({ trueColor: true, terminalBrand: "kitty" });
    expect(devtools.capabilities.get().trueColor).toBe(true);
  });

  it("enabled DevTools updates scheduler", () => {
    const devtools = createDevTools({ enabled: true });
    devtools.updateScheduler({ frameCount: 42, droppedFrames: 2 });
    expect(devtools.scheduler.getSnapshot().frameCount).toBe(42);
  });

  it("enabled DevTools captures snapshots", () => {
    const devtools = createDevTools({ enabled: true });
    const tree = { id: "root", type: "Box", props: {}, children: [] };
    const id = devtools.captureSnapshot(tree);
    expect(id).toBe(0);
    expect(devtools.snapshots.getSnapshots()).toHaveLength(1);
  });

  it("enabled DevTools exports full data", () => {
    const devtools = createDevTools({ enabled: true });
    devtools.recordCommand("CreateNode", { id: "1" });
    devtools.recordKeyboard("a", { ctrl: false, shift: false, alt: false, meta: false });
    devtools.recordFrame({ duration: 16 });
    const data = devtools.exportData();
    expect(data.commands).toHaveLength(1);
    expect(data.events).toHaveLength(1);
    expect(data.frames).toHaveLength(1);
  });

  it("enabled DevTools reset clears all data", () => {
    const devtools = createDevTools({ enabled: true });
    devtools.recordCommand("CreateNode", { id: "1" });
    devtools.recordKeyboard("a", { ctrl: false, shift: false, alt: false, meta: false });
    devtools.reset();
    expect(devtools.commands.getTotalCount()).toBe(0);
    expect(devtools.events.count).toBe(0);
  });

  it("enabled DevTools exports tree when present", () => {
    const devtools = createDevTools({ enabled: true });
    devtools.tree.buildTree([{ id: "root", type: "Box" }]);
    const data = devtools.exportData();
    expect(data.tree).toBeDefined();
    expect(data.tree?.id).toBe("root");
  });

  it("enabled DevTools getSummary includes all sections", () => {
    const devtools = createDevTools({ enabled: true });
    devtools.recordCommand("CreateNode", { id: "1" });
    devtools.recordKeyboard("a", { ctrl: false, shift: false, alt: false, meta: false });
    devtools.recordFrame({ duration: 16 });
    devtools.updateScheduler({ frameCount: 10 });
    devtools.recordFocus("focus", "node-1");
    devtools.updateCapabilities({ trueColor: true, terminalBrand: "kitty" });
    const summary = devtools.getSummary();
    expect(summary).toContain("Performance");
    expect(summary).toContain("Activity");
    expect(summary).toContain("Scheduler");
    expect(summary).toContain("Focus");
    expect(summary).toContain("Terminal");
    expect(summary).toContain("Avg Commands/Frame");
  });

  it("enabled DevTools respects maxEvents option", () => {
    const devtools = createDevTools({ enabled: true, maxEvents: 5 });
    for (let i = 0; i < 10; i++) {
      devtools.recordCommand("CreateNode", { id: String(i) });
    }
    expect(devtools.commands.getTotalCount()).toBe(5);
  });

  it("enabled DevTools respects logLevel option", () => {
    const devtools = createDevTools({ enabled: true, logLevel: "warn" });
    devtools.recordCommand("CreateNode", { id: "1" });
    const entries = devtools.logger.getEntries();
    expect(entries).toHaveLength(0);
  });

  it("enabled DevTools dispose clears everything", () => {
    const devtools = createDevTools({ enabled: true });
    devtools.recordCommand("CreateNode", { id: "1" });
    devtools.dispose();
    expect(devtools.commands.getTotalCount()).toBe(0);
    expect(devtools.events.count).toBe(0);
  });
});

// ─── Logger ──────────────────────────────────────────────────────────────────

describe("Logger", () => {
  it("records entries at all levels", () => {
    const logger = new Logger();
    logger.debug("test", "debug msg");
    logger.info("test", "info msg");
    logger.warn("test", "warn msg");
    logger.error("test", "error msg");
    logger.trace("test", "trace msg");
    expect(logger.count).toBe(5);
  });

  it("filters by minimum level", () => {
    const logger = new Logger({ minLevel: "warn" });
    logger.debug("test", "debug msg");
    logger.info("test", "info msg");
    logger.warn("test", "warn msg");
    logger.error("test", "error msg");
    expect(logger.count).toBe(2);
  });

  it("respects maxEntries", () => {
    const logger = new Logger({ maxEntries: 3 });
    for (let i = 0; i < 5; i++) {
      logger.info("test", `msg ${i}`);
    }
    expect(logger.count).toBe(3);
  });

  it("filters by level", () => {
    const logger = new Logger();
    logger.debug("test", "debug");
    logger.info("test", "info");
    logger.error("test", "error");
    expect(logger.getEntriesByLevel("error")).toHaveLength(1);
    expect(logger.getEntriesByLevel("debug")).toHaveLength(1);
  });

  it("filters by category", () => {
    const logger = new Logger();
    logger.info("auth", "login");
    logger.info("render", "paint");
    expect(logger.getEntriesByCategory("auth")).toHaveLength(1);
  });

  it("searches entries", () => {
    const logger = new Logger();
    logger.info("auth", "user login");
    logger.info("auth", "user logout");
    logger.info("render", "paint frame");
    expect(logger.search("login")).toHaveLength(1);
    expect(logger.search("user")).toHaveLength(2);
  });

  it("clears all entries", () => {
    const logger = new Logger();
    logger.info("test", "msg");
    logger.clear();
    expect(logger.count).toBe(0);
  });

  it("records entry with data payload", () => {
    const logger = new Logger();
    logger.info("test", "msg", { key: "value" });
    const entries = logger.getEntries();
    expect(entries[0]?.data).toEqual({ key: "value" });
  });

  it("calls onEntry callback", () => {
    let captured = false;
    const logger = new Logger({
      onEntry: () => {
        captured = true;
      },
    });
    logger.info("test", "msg");
    expect(captured).toBe(true);
  });
});

// ─── CommandInspector ────────────────────────────────────────────────────────

describe("CommandInspector", () => {
  it("records commands", () => {
    const inspector = new CommandInspector();
    inspector.record("CreateNode", { id: "1" });
    expect(inspector.getTotalCount()).toBe(1);
  });

  it("tracks command counts by type", () => {
    const inspector = new CommandInspector();
    inspector.record("CreateNode", { id: "1" });
    inspector.record("CreateNode", { id: "2" });
    inspector.record("SetText", { id: "1", text: "hello" });
    const counts = inspector.getCounts();
    expect(counts.get("CreateNode")).toBe(2);
    expect(counts.get("SetText")).toBe(1);
  });

  it("filters by type", () => {
    const inspector = new CommandInspector();
    inspector.record("CreateNode", { id: "1" });
    inspector.record("SetText", { id: "1" });
    expect(inspector.getCommandsByType("CreateNode")).toHaveLength(1);
  });

  it("gets recent commands", () => {
    const inspector = new CommandInspector();
    for (let i = 0; i < 10; i++) {
      inspector.record("CreateNode", { id: String(i) });
    }
    expect(inspector.getRecent(3)).toHaveLength(3);
  });

  it("respects maxCommands", () => {
    const inspector = new CommandInspector({ maxCommands: 5 });
    for (let i = 0; i < 10; i++) {
      inspector.record("CreateNode", { id: String(i) });
    }
    expect(inspector.getTotalCount()).toBe(5);
  });

  it("returns summary", () => {
    const inspector = new CommandInspector();
    inspector.record("CreateNode", { id: "1" });
    const summary = inspector.getSummary();
    expect(summary.total).toBe(1);
    expect(summary.byType["CreateNode"]).toBe(1);
    expect(summary.lastTimestamp).toBeTypeOf("number");
  });

  it("getSummary with empty commands returns null lastTimestamp", () => {
    const inspector = new CommandInspector();
    const summary = inspector.getSummary();
    expect(summary.total).toBe(0);
    expect(summary.lastTimestamp).toBeNull();
  });

  it("getSummary with frameCount=0 returns avgCommandsPerFrame=0", () => {
    const inspector = new CommandInspector();
    inspector.record("CreateNode", { id: "1" });
    const summary = inspector.getSummary(0);
    expect(summary.total).toBe(1);
    expect(summary.avgCommandsPerFrame).toBe(0);
  });

  it("getSummary with frameCount omitted returns avgCommandsPerFrame=0", () => {
    const inspector = new CommandInspector();
    inspector.record("CreateNode", { id: "1" });
    const summary = inspector.getSummary();
    expect(summary.avgCommandsPerFrame).toBe(0);
  });

  it("getSummary with frameCount>0 returns avgCommandsPerFrame", () => {
    const inspector = new CommandInspector();
    inspector.record("CreateNode", { id: "1" });
    inspector.record("SetText", { id: "2" });
    const summary = inspector.getSummary(4);
    expect(summary.avgCommandsPerFrame).toBe(0.5);
  });

  it("clears all commands", () => {
    const inspector = new CommandInspector();
    inspector.record("CreateNode", { id: "1" });
    inspector.clear();
    expect(inspector.getTotalCount()).toBe(0);
  });

  it("records command with duration", () => {
    const inspector = new CommandInspector();
    const cmd = inspector.record("CreateNode", { id: "1" }, 5.5);
    expect(cmd.duration).toBe(5.5);
  });

  it("getCommandsInRange filters correctly", () => {
    const inspector = new CommandInspector();
    inspector.record("CreateNode", { id: "1" });
    inspector.record("SetText", { id: "2" });
    inspector.record("AppendChild", { id: "3" });
    const commands = inspector.getCommands();
    const mid = commands[1]?.timestamp ?? 0;
    const result = inspector.getCommandsInRange(0, mid);
    expect(result.length).toBeGreaterThanOrEqual(1);
  });

  it("calls onCommand callback", () => {
    let captured = false;
    const inspector = new CommandInspector({
      onCommand: () => {
        captured = true;
      },
    });
    inspector.record("CreateNode", { id: "1" });
    expect(captured).toBe(true);
  });

  it("getCommands returns readonly array", () => {
    const inspector = new CommandInspector();
    inspector.record("CreateNode", { id: "1" });
    const cmds = inspector.getCommands();
    expect(cmds).toHaveLength(1);
  });
});

// ─── EventInspector ──────────────────────────────────────────────────────────

describe("EventInspector", () => {
  it("records keyboard events", () => {
    const inspector = new EventInspector();
    inspector.recordKeyboard("a", { ctrl: false, shift: false, alt: false, meta: false });
    expect(inspector.count).toBe(1);
  });

  it("records mouse events", () => {
    const inspector = new EventInspector();
    inspector.recordMouse("click", 10, 20, "left");
    expect(inspector.count).toBe(1);
  });

  it("records focus events", () => {
    const inspector = new EventInspector();
    inspector.recordFocus("focus", "node-1");
    expect(inspector.count).toBe(1);
  });

  it("records resize events", () => {
    const inspector = new EventInspector();
    inspector.recordResize(120, 40, 80, 24);
    expect(inspector.count).toBe(1);
  });

  it("records lifecycle events", () => {
    const inspector = new EventInspector();
    inspector.recordLifecycle("mount");
    expect(inspector.count).toBe(1);
  });

  it("filters by category", () => {
    const inspector = new EventInspector();
    inspector.recordKeyboard("a", { ctrl: false, shift: false, alt: false, meta: false });
    inspector.recordMouse("click", 10, 20);
    expect(inspector.getEventsByCategory("keyboard")).toHaveLength(1);
    expect(inspector.getEventsByCategory("mouse")).toHaveLength(1);
  });

  it("filters by type", () => {
    const inspector = new EventInspector();
    inspector.recordKeyboard("a", { ctrl: false, shift: false, alt: false, meta: false });
    inspector.recordMouse("click", 10, 20);
    expect(inspector.getEventsByType("keydown")).toHaveLength(1);
  });

  it("gets recent events", () => {
    const inspector = new EventInspector();
    for (let i = 0; i < 10; i++) {
      inspector.recordKeyboard(String(i), { ctrl: false, shift: false, alt: false, meta: false });
    }
    expect(inspector.getRecent(3)).toHaveLength(3);
  });

  it("clears all events", () => {
    const inspector = new EventInspector();
    inspector.recordKeyboard("a", { ctrl: false, shift: false, alt: false, meta: false });
    inspector.clear();
    expect(inspector.count).toBe(0);
  });

  it("respects maxEvents", () => {
    const inspector = new EventInspector({ maxEvents: 3 });
    for (let i = 0; i < 5; i++) {
      inspector.recordKeyboard(String(i), { ctrl: false, shift: false, alt: false, meta: false });
    }
    expect(inspector.count).toBe(3);
  });

  it("getEventsInRange filters correctly", () => {
    const inspector = new EventInspector();
    inspector.recordKeyboard("a", { ctrl: false, shift: false, alt: false, meta: false });
    inspector.recordMouse("click", 10, 20);
    inspector.recordKeyboard("b", { ctrl: false, shift: false, alt: false, meta: false });
    const events = inspector.getEvents();
    const start = events[0]?.timestamp ?? 0;
    const end = events[1]?.timestamp ?? 0;
    const result = inspector.getEventsInRange(start, end);
    expect(result.length).toBeGreaterThanOrEqual(1);
  });

  it("getCategoryCounts returns correct counts", () => {
    const inspector = new EventInspector();
    inspector.recordKeyboard("a", { ctrl: false, shift: false, alt: false, meta: false });
    inspector.recordKeyboard("b", { ctrl: false, shift: false, alt: false, meta: false });
    inspector.recordMouse("click", 10, 20);
    const counts = inspector.getCategoryCounts();
    expect(counts.get("keyboard")).toBe(2);
    expect(counts.get("mouse")).toBe(1);
  });

  it("record with propagation parameter", () => {
    const inspector = new EventInspector();
    const event = inspector.record("keyboard", "keydown", "node-1", { key: "a" }, "bubbled");
    expect(event.propagation).toBe("bubbled");
    expect(event.target).toBe("node-1");
  });

  it("calls onEvent callback", () => {
    let captured = false;
    const inspector = new EventInspector({
      onEvent: () => {
        captured = true;
      },
    });
    inspector.recordKeyboard("a", { ctrl: false, shift: false, alt: false, meta: false });
    expect(captured).toBe(true);
  });

  it("getEvents returns readonly array", () => {
    const inspector = new EventInspector();
    inspector.recordKeyboard("a", { ctrl: false, shift: false, alt: false, meta: false });
    const events = inspector.getEvents();
    expect(events).toHaveLength(1);
  });
});

// ─── PerformanceTracker ──────────────────────────────────────────────────────

describe("PerformanceTracker", () => {
  it("records frame metrics via recordFrame", () => {
    const tracker = new PerformanceTracker();
    tracker.recordFrame({ duration: 16.5 });
    expect(tracker.count).toBe(1);
  });

  it("calculates FPS with multiple frames", () => {
    const tracker = new PerformanceTracker();
    const now = performance.now();
    for (let i = 0; i < 5; i++) {
      tracker.recordFrame({ duration: 16.67, timestamp: now + i * 16.67 });
    }
    const fps = tracker.getFps(5);
    expect(fps).toBeGreaterThan(0);
  });

  it("getFps returns 0 with 0 frames", () => {
    const tracker = new PerformanceTracker();
    expect(tracker.getFps()).toBe(0);
  });

  it("getFps returns 0 with 1 frame", () => {
    const tracker = new PerformanceTracker();
    tracker.recordFrame({ duration: 16 });
    expect(tracker.getFps()).toBe(0);
  });

  it("getFps with sampleSize smaller than total frames", () => {
    const tracker = new PerformanceTracker();
    for (let i = 0; i < 10; i++) {
      tracker.beginFrame(1);
      tracker.endFrame({});
    }
    const fps = tracker.getFps(3);
    expect(fps).toBeGreaterThanOrEqual(0);
    expect(typeof fps).toBe("number");
  });

  it("getFps calculates correctly with multiple frames", () => {
    const tracker = new PerformanceTracker();
    // Use beginFrame/endFrame which uses performance.now() internally
    for (let i = 0; i < 5; i++) {
      tracker.beginFrame(1);
      tracker.endFrame({});
    }
    const fps = tracker.getFps(5);
    expect(fps).toBeGreaterThan(0);
    expect(typeof fps).toBe("number");
  });

  it("returns snapshot with correct totals", () => {
    const tracker = new PerformanceTracker();
    tracker.recordFrame({ duration: 16, commandCount: 5, dirtyRegionCount: 2 });
    tracker.recordFrame({ duration: 20, commandCount: 3, dirtyRegionCount: 1 });
    const snapshot = tracker.getSnapshot();
    expect(snapshot.totalFrames).toBe(2);
    expect(snapshot.commandCount).toBe(8);
    expect(snapshot.dirtyNodeCount).toBe(3);
    expect(snapshot.droppedFrames).toBe(0);
  });

  it("tracks dropped frames (duration > 33.33ms)", () => {
    const tracker = new PerformanceTracker();
    tracker.recordFrame({ duration: 40 });
    tracker.recordFrame({ duration: 10 });
    const snapshot = tracker.getSnapshot();
    expect(snapshot.droppedFrames).toBe(1);
  });

  it("calculates min/max/avg frame times", () => {
    const tracker = new PerformanceTracker();
    tracker.recordFrame({ duration: 10 });
    tracker.recordFrame({ duration: 20 });
    tracker.recordFrame({ duration: 30 });
    const snapshot = tracker.getSnapshot();
    expect(snapshot.minFrameTime).toBe(10);
    expect(snapshot.maxFrameTime).toBe(30);
    expect(snapshot.avgFrameTime).toBeCloseTo(20);
  });

  it("respects maxFrames overflow", () => {
    const tracker = new PerformanceTracker({ maxFrames: 3 });
    for (let i = 0; i < 5; i++) {
      tracker.recordFrame({ duration: 16 });
    }
    expect(tracker.count).toBe(3);
  });

  it("clears all frames and resets frame number", () => {
    const tracker = new PerformanceTracker();
    tracker.recordFrame({ duration: 16 });
    tracker.recordFrame({ duration: 16 });
    tracker.clear();
    expect(tracker.count).toBe(0);
    const frame = tracker.recordFrame({ duration: 16 });
    expect(frame.frameNumber).toBe(0);
  });

  it("beginFrame/endFrame flow records correct duration", () => {
    const tracker = new PerformanceTracker();
    tracker.beginFrame(5);
    const frame = tracker.endFrame({ dirtyRegionCount: 2 });
    expect(frame.commandCount).toBe(5);
    expect(frame.dirtyRegionCount).toBe(2);
    expect(frame.duration).toBeGreaterThanOrEqual(0);
  });

  it("endFrame with all options", () => {
    const tracker = new PerformanceTracker();
    tracker.beginFrame(10);
    const frame = tracker.endFrame({
      dirtyRegionCount: 3,
      renderDuration: 5,
      layoutDuration: 3,
      paintDuration: 2,
      ffiDuration: 1,
    });
    expect(frame.renderDuration).toBe(5);
    expect(frame.layoutDuration).toBe(3);
    expect(frame.paintDuration).toBe(2);
    expect(frame.ffiDuration).toBe(1);
  });

  it("endFrame with no arguments uses defaults", () => {
    const tracker = new PerformanceTracker();
    tracker.beginFrame(3);
    const frame = tracker.endFrame({});
    expect(frame.dirtyRegionCount).toBe(0);
    expect(frame.commandCount).toBe(3);
  });

  it("calls onFrame callback via recordFrame", () => {
    let captured = false;
    const tracker = new PerformanceTracker({
      onFrame: () => {
        captured = true;
      },
    });
    tracker.recordFrame({ duration: 16 });
    expect(captured).toBe(true);
  });

  it("calls onFrame callback via beginFrame/endFrame", () => {
    let called = false;
    const tracker = new PerformanceTracker({
      onFrame: () => {
        called = true;
      },
    });
    tracker.beginFrame(5);
    tracker.endFrame({ dirtyRegionCount: 2 });
    expect(called).toBe(true);
    const frames = tracker.getFrames();
    expect(frames[0]?.commandCount).toBe(5);
    expect(frames[0]?.dirtyRegionCount).toBe(2);
  });

  it("getRecentFrames returns correct count", () => {
    const tracker = new PerformanceTracker();
    for (let i = 0; i < 10; i++) {
      tracker.recordFrame({ duration: 16 });
    }
    expect(tracker.getRecentFrames(3)).toHaveLength(3);
    expect(tracker.getRecentFrames(100)).toHaveLength(10);
  });

  it("getFrames returns readonly array", () => {
    const tracker = new PerformanceTracker();
    tracker.recordFrame({ duration: 16 });
    const frames = tracker.getFrames();
    expect(frames).toHaveLength(1);
  });

  it("getSnapshot with empty frames returns zeros", () => {
    const tracker = new PerformanceTracker();
    const snapshot = tracker.getSnapshot();
    expect(snapshot.fps).toBe(0);
    expect(snapshot.avgFrameTime).toBe(0);
    expect(snapshot.minFrameTime).toBe(0);
    expect(snapshot.maxFrameTime).toBe(0);
  });

  it("recordFrame ignores timestamp parameter", () => {
    const tracker = new PerformanceTracker();
    const before = performance.now();
    const frame = tracker.recordFrame({ duration: 16, timestamp: 999 });
    expect(frame.timestamp).toBeGreaterThanOrEqual(before);
    expect(frame.timestamp).not.toBe(999);
  });
});

// ─── TreeInspector ───────────────────────────────────────────────────────────

describe("TreeInspector", () => {
  it("builds a tree from flat nodes", () => {
    const inspector = new TreeInspector();
    const root = inspector.buildTree([
      { id: "root", type: "Box" },
      { id: "child1", type: "Text", parent: "root" },
      { id: "child2", type: "Text", parent: "root" },
    ]);
    expect(root.id).toBe("root");
    expect(root.children).toHaveLength(2);
    expect(inspector.countNodes()).toBe(3);
  });

  it("builds tree with all node properties", () => {
    const inspector = new TreeInspector();
    const root = inspector.buildTree([
      {
        id: "n1",
        type: "Box",
        props: { key: "val" },
        style: { bold: true },
        layout: { x: 0, y: 0, width: 100, height: 50 },
        dirty: true,
        visible: true,
        zIndex: 10,
      },
    ]);
    expect(root.props).toEqual({ key: "val" });
    expect(root.style).toEqual({ bold: true });
    expect(root.layout).toEqual({ x: 0, y: 0, width: 100, height: 50 });
    expect(root.dirty).toBe(true);
    expect(root.visible).toBe(true);
    expect(root.zIndex).toBe(10);
  });

  it("builds tree with empty array returns empty node", () => {
    const inspector = new TreeInspector();
    const root = inspector.buildTree([]);
    expect(root.id).toBe("empty");
    expect(root.type).toBe("Empty");
  });

  it("calls onTreeUpdate callback", () => {
    let called = false;
    const inspector = new TreeInspector({
      onTreeUpdate: () => {
        called = true;
      },
    });
    inspector.buildTree([{ id: "root", type: "Box" }]);
    expect(called).toBe(true);
  });

  it("handles orphan nodes (parent not found)", () => {
    const inspector = new TreeInspector();
    inspector.buildTree([{ id: "child", type: "Text", parent: "nonexistent" }]);
    expect(inspector.getNode("child")).toBeDefined();
  });

  it("updates node properties", () => {
    const inspector = new TreeInspector();
    inspector.buildTree([{ id: "n1", type: "Box" }]);
    inspector.updateNode("n1", { dirty: true });
    expect(inspector.getNode("n1")?.dirty).toBe(true);
  });

  it("updateNode for style field", () => {
    const inspector = new TreeInspector();
    inspector.buildTree([{ id: "n1", type: "Box" }]);
    inspector.updateNode("n1", { style: { bold: true } });
    expect(inspector.getNode("n1")?.style).toEqual({ bold: true });
  });

  it("updateNode for layout field", () => {
    const inspector = new TreeInspector();
    inspector.buildTree([{ id: "n1", type: "Box" }]);
    inspector.updateNode("n1", { layout: { x: 0, y: 0, width: 100, height: 50 } });
    expect(inspector.getNode("n1")?.layout).toEqual({ x: 0, y: 0, width: 100, height: 50 });
  });

  it("updateNode for visible field", () => {
    const inspector = new TreeInspector();
    inspector.buildTree([{ id: "n1", type: "Box" }]);
    inspector.updateNode("n1", { visible: false });
    expect(inspector.getNode("n1")?.visible).toBe(false);
  });

  it("updateNode for zIndex field", () => {
    const inspector = new TreeInspector();
    inspector.buildTree([{ id: "n1", type: "Box" }]);
    inspector.updateNode("n1", { zIndex: 100 });
    expect(inspector.getNode("n1")?.zIndex).toBe(100);
  });

  it("updateNode for props field", () => {
    const inspector = new TreeInspector();
    inspector.buildTree([{ id: "n1", type: "Box" }]);
    inspector.updateNode("n1", { props: { newProp: "value" } });
    expect(inspector.getNode("n1")?.props).toEqual({ newProp: "value" });
  });

  it("updateNode on non-existent node does not throw", () => {
    const inspector = new TreeInspector();
    inspector.buildTree([{ id: "n1", type: "Box" }]);
    expect(() => inspector.updateNode("nonexistent", { dirty: true })).not.toThrow();
  });

  it("updateNode dirty=false removes from dirty set", () => {
    const inspector = new TreeInspector();
    inspector.buildTree([{ id: "n1", type: "Box" }]);
    inspector.markDirty("n1");
    expect(inspector.getDirtyNodes()).toHaveLength(1);
    inspector.updateNode("n1", { dirty: false });
    expect(inspector.getDirtyNodes()).toHaveLength(0);
  });

  it("marks node as dirty", () => {
    const inspector = new TreeInspector();
    inspector.buildTree([
      { id: "root", type: "Box" },
      { id: "child", type: "Text", parent: "root" },
    ]);
    inspector.markDirty("child");
    expect(inspector.getDirtyNodes()).toHaveLength(1);
    expect(inspector.getNode("child")?.dirty).toBe(true);
  });

  it("clearDirty clears all dirty state", () => {
    const inspector = new TreeInspector();
    inspector.buildTree([
      { id: "n1", type: "Box", dirty: true },
      { id: "n2", type: "Box", dirty: true },
    ]);
    inspector.clearDirty();
    expect(inspector.getDirtyNodes()).toHaveLength(0);
    expect(inspector.getNode("n1")?.dirty).toBe(false);
  });

  it("finds nodes by predicate", () => {
    const inspector = new TreeInspector();
    inspector.buildTree([
      { id: "root", type: "Box" },
      { id: "child", type: "Text", parent: "root" },
    ]);
    const texts = inspector.findNodes((n) => n.type === "Text");
    expect(texts).toHaveLength(1);
  });

  it("findNodes with no matches returns empty", () => {
    const inspector = new TreeInspector();
    inspector.buildTree([{ id: "root", type: "Box" }]);
    const result = inspector.findNodes((n) => n.type === "Nonexistent");
    expect(result).toHaveLength(0);
  });

  it("findNodes works with empty tree", () => {
    const inspector = new TreeInspector();
    inspector.buildTree([]);
    const result = inspector.findNodes(() => true);
    expect(result).toHaveLength(0);
  });

  it("gets path from root", () => {
    const inspector = new TreeInspector();
    inspector.buildTree([
      { id: "root", type: "Box" },
      { id: "child", type: "Text", parent: "root" },
    ]);
    const path = inspector.getPath("child");
    expect(path).toHaveLength(2);
    expect(path[0]?.id).toBe("root");
    expect(path[1]?.id).toBe("child");
  });

  it("getPath for root node returns single element", () => {
    const inspector = new TreeInspector();
    inspector.buildTree([{ id: "root", type: "Box" }]);
    const path = inspector.getPath("root");
    expect(path).toHaveLength(1);
    expect(path[0]?.id).toBe("root");
  });

  it("getPath with 3+ levels deep tree", () => {
    const inspector = new TreeInspector();
    inspector.buildTree([
      { id: "root", type: "Box" },
      { id: "child", type: "Flex", parent: "root" },
      { id: "grandchild", type: "Text", parent: "child" },
      { id: "great-grandchild", type: "Text", parent: "grandchild" },
    ]);
    const path = inspector.getPath("great-grandchild");
    expect(path).toHaveLength(4);
    expect(path[0]?.id).toBe("root");
    expect(path[1]?.id).toBe("child");
    expect(path[2]?.id).toBe("grandchild");
    expect(path[3]?.id).toBe("great-grandchild");
  });

  it("getPath for non-existent node returns empty", () => {
    const inspector = new TreeInspector();
    inspector.buildTree([{ id: "root", type: "Box" }]);
    const path = inspector.getPath("nonexistent");
    expect(path).toHaveLength(0);
  });

  it("getAllNodes returns all nodes", () => {
    const inspector = new TreeInspector();
    inspector.buildTree([
      { id: "root", type: "Box" },
      { id: "child", type: "Text", parent: "root" },
    ]);
    const nodes = inspector.getAllNodes();
    expect(nodes).toHaveLength(2);
  });

  it("clears all data", () => {
    const inspector = new TreeInspector();
    inspector.buildTree([{ id: "root", type: "Box" }]);
    inspector.clear();
    expect(inspector.getRoot()).toBeNull();
    expect(inspector.getAllNodes()).toHaveLength(0);
  });

  it("getNode returns undefined for non-existent node", () => {
    const inspector = new TreeInspector();
    inspector.buildTree([{ id: "root", type: "Box" }]);
    expect(inspector.getNode("nonexistent")).toBeUndefined();
  });
});

// ─── SchedulerInspector ──────────────────────────────────────────────────────

describe("SchedulerInspector", () => {
  it("returns default snapshot", () => {
    const inspector = new SchedulerInspector();
    const snapshot = inspector.getSnapshot();
    expect(snapshot.frameCount).toBe(0);
    expect(snapshot.droppedFrames).toBe(0);
  });

  it("updates state", () => {
    const inspector = new SchedulerInspector();
    inspector.updateState({ frameCount: 42, droppedFrames: 2 });
    expect(inspector.getSnapshot().frameCount).toBe(42);
  });

  it("tracks frame drops", () => {
    const inspector = new SchedulerInspector();
    inspector.recordFrameDrop();
    inspector.recordFrameDrop();
    expect(inspector.getSnapshot().droppedFrames).toBe(2);
  });

  it("calculates drop rate", () => {
    const inspector = new SchedulerInspector();
    inspector.updateState({ frameCount: 100 });
    inspector.recordFrameDrop();
    inspector.recordFrameDrop();
    expect(inspector.getDropRate()).toBeCloseTo(0.02);
  });

  it("drop rate returns 0 with no frames", () => {
    const inspector = new SchedulerInspector();
    expect(inspector.getDropRate()).toBe(0);
  });

  it("clears all data", () => {
    const inspector = new SchedulerInspector();
    inspector.updateState({ frameCount: 10 });
    inspector.clear();
    expect(inspector.getSnapshot().frameCount).toBe(0);
  });

  it("calls onFrameDrop callback", () => {
    let captured = false;
    const inspector = new SchedulerInspector({
      onFrameDrop: () => {
        captured = true;
      },
    });
    inspector.recordFrameDrop();
    expect(captured).toBe(true);
  });

  it("incrementFrameCount increases frame count", () => {
    const inspector = new SchedulerInspector();
    inspector.incrementFrameCount();
    inspector.incrementFrameCount();
    expect(inspector.getSnapshot().frameCount).toBe(2);
  });

  it("updateState with all fields updates snapshot", () => {
    const inspector = new SchedulerInspector();
    inspector.updateState({
      isRunning: true,
      isRendering: true,
      hasScheduledRender: true,
      frameCount: 50,
      pendingFrames: 2,
      highestPriority: "high",
      idleCallbacksPending: 5,
      animationFramesPending: 1,
      frameBudgetMs: 8,
      utilization: 0.75,
    });
    inspector.recordFrameDrop();
    inspector.recordFrameDrop();
    inspector.recordFrameDrop();
    const s = inspector.getSnapshot();
    expect(s.isRunning).toBe(true);
    expect(s.isRendering).toBe(true);
    expect(s.hasScheduledRender).toBe(true);
    expect(s.frameCount).toBe(50);
    expect(s.droppedFrames).toBe(3);
    expect(s.pendingFrames).toBe(2);
    expect(s.highestPriority).toBe("high");
    expect(s.idleCallbacksPending).toBe(5);
    expect(s.animationFramesPending).toBe(1);
    expect(s.frameBudgetMs).toBe(8);
    expect(s.utilization).toBe(0.75);
  });
});

// ─── FocusInspector ──────────────────────────────────────────────────────────

describe("FocusInspector", () => {
  it("records focus", () => {
    const inspector = new FocusInspector();
    inspector.recordFocus("node-1");
    expect(inspector.getSnapshot().focusedNodeId).toBe("node-1");
  });

  it("records blur", () => {
    const inspector = new FocusInspector();
    inspector.recordFocus("node-1");
    inspector.recordBlur("node-1");
    expect(inspector.getSnapshot().focusedNodeId).toBeNull();
  });

  it("blur on non-focused node does not change focus", () => {
    const inspector = new FocusInspector();
    inspector.recordFocus("node-1");
    inspector.recordBlur("node-2");
    expect(inspector.getSnapshot().focusedNodeId).toBe("node-1");
  });

  it("tracks focus history", () => {
    const inspector = new FocusInspector();
    inspector.recordFocus("node-1");
    inspector.recordFocus("node-2");
    expect(inspector.getFocusHistory()).toHaveLength(2);
  });

  it("getRecentFocusChanges returns correct count", () => {
    const inspector = new FocusInspector();
    inspector.recordFocus("node-1");
    inspector.recordFocus("node-2");
    inspector.recordFocus("node-3");
    const recent = inspector.getRecentFocusChanges(2);
    expect(recent).toHaveLength(2);
  });

  it("sets focusable nodes", () => {
    const inspector = new FocusInspector();
    inspector.setFocusableNodes(["n1", "n2"]);
    expect(inspector.getSnapshot().focusableNodes).toHaveLength(2);
  });

  it("sets tab order", () => {
    const inspector = new FocusInspector();
    inspector.setTabOrder(["n2", "n1"]);
    expect(inspector.getSnapshot().tabOrder).toHaveLength(2);
  });

  it("sets scope", () => {
    const inspector = new FocusInspector();
    inspector.setScope("modal");
    expect(inspector.getSnapshot().currentScope).toBe("modal");
  });

  it("clears scope", () => {
    const inspector = new FocusInspector();
    inspector.setScope("modal");
    inspector.setScope(null);
    expect(inspector.getSnapshot().currentScope).toBeNull();
  });

  it("isFocused returns correct value", () => {
    const inspector = new FocusInspector();
    inspector.recordFocus("node-1");
    expect(inspector.isFocused("node-1")).toBe(true);
    expect(inspector.isFocused("node-2")).toBe(false);
  });

  it("calls onFocusChange callback", () => {
    let captured = false;
    const inspector = new FocusInspector({
      onFocusChange: () => {
        captured = true;
      },
    });
    inspector.recordFocus("node-1");
    expect(captured).toBe(true);
  });

  it("calls onFocusChange callback on blur", () => {
    let captureCount = 0;
    const inspector = new FocusInspector({
      onFocusChange: () => {
        captureCount++;
      },
    });
    inspector.recordFocus("node-1");
    inspector.recordBlur("node-1");
    expect(captureCount).toBe(2);
  });

  it("clears all data", () => {
    const inspector = new FocusInspector();
    inspector.recordFocus("node-1");
    inspector.clear();
    expect(inspector.getSnapshot().focusedNodeId).toBeNull();
  });
});

// ─── CapabilityInspector ─────────────────────────────────────────────────────

describe("CapabilityInspector", () => {
  it("returns default capabilities", () => {
    const inspector = new CapabilityInspector();
    expect(inspector.get().trueColor).toBe(false);
  });

  it("updates capabilities", () => {
    const inspector = new CapabilityInspector();
    inspector.update({ trueColor: true, terminalBrand: "kitty" });
    expect(inspector.get().trueColor).toBe(true);
    expect(inspector.get().terminalBrand).toBe("kitty");
  });

  it("has() returns boolean for boolean capabilities", () => {
    const inspector = new CapabilityInspector();
    inspector.update({ trueColor: true });
    expect(inspector.has("trueColor")).toBe(true);
    expect(inspector.has("kittyKeyboard")).toBe(false);
  });

  it("has() returns true for non-empty string capabilities", () => {
    const inspector = new CapabilityInspector();
    expect(inspector.has("terminalBrand")).toBe(false);
    inspector.update({ terminalBrand: "kitty" });
    expect(inspector.has("terminalBrand")).toBe(true);
  });

  it("has() returns true for object capabilities", () => {
    const inspector = new CapabilityInspector();
    expect(inspector.has("terminalSize")).toBe(true);
  });

  it("has() with non-existent capability returns false via fallthrough", () => {
    const inspector = new CapabilityInspector();
    // Cast needed: has() is typed to accept keyof TerminalCapabilities,
    // but at runtime we can pass any string to test the fallthrough path
    // biome-ignore lint/suspicious/noExplicitAny: testing runtime behavior beyond the type system
    expect(inspector.has("nonexistent" as any)).toBe(false);
  });

  it("returns summary of enabled features", () => {
    const inspector = new CapabilityInspector();
    inspector.update({ trueColor: true, mouseSupport: true });
    const summary = inspector.getSummary();
    expect(summary).toContain("trueColor");
    expect(summary).toContain("mouse");
  });

  it("getSummary with all capabilities enabled returns all features", () => {
    const inspector = new CapabilityInspector();
    inspector.update({
      trueColor: true,
      kittyKeyboard: true,
      mouseSupport: true,
      osc52: true,
      osc8: true,
      pixelSupport: true,
      alternateScreen: true,
      syncUpdate: true,
      bracketedPaste: true,
      focusEvents: true,
      strikethrough: true,
      underlineColor: true,
      cursorStyle: true,
      hyperlinks: true,
      inlineImages: true,
      sixel: true,
    });
    const summary = inspector.getSummary();
    expect(summary).toContain("trueColor");
    expect(summary).toContain("kittyKeyboard");
    expect(summary).toContain("mouse");
    expect(summary).toContain("osc52");
    expect(summary).toContain("osc8");
    expect(summary).toContain("pixel");
    expect(summary).toContain("altScreen");
    expect(summary).toContain("sync");
    expect(summary).toContain("bracketedPaste");
    expect(summary).toContain("focusEvents");
    expect(summary).toContain("strikethrough");
    expect(summary).toContain("underlineColor");
    expect(summary).toContain("cursorStyle");
    expect(summary).toContain("hyperlinks");
    expect(summary).toContain("inlineImages");
    expect(summary).toContain("sixel");
    expect(summary).toHaveLength(16);
  });

  it("updateFromNative parses JSON", () => {
    const inspector = new CapabilityInspector();
    inspector.updateFromNative(JSON.stringify({ trueColor: true }));
    expect(inspector.get().trueColor).toBe(true);
  });

  it("updateFromNative handles invalid JSON", () => {
    const inspector = new CapabilityInspector();
    expect(() => inspector.updateFromNative("invalid json")).not.toThrow();
  });

  it("calls onCapabilitiesDetected callback", () => {
    let captured = false;
    const inspector = new CapabilityInspector({
      onCapabilitiesDetected: () => {
        captured = true;
      },
    });
    inspector.update({ trueColor: true });
    expect(captured).toBe(true);
  });

  it("clears all data", () => {
    const inspector = new CapabilityInspector();
    inspector.update({ trueColor: true });
    inspector.clear();
    expect(inspector.get().trueColor).toBe(false);
  });
});

// ─── Timeline ────────────────────────────────────────────────────────────────

describe("Timeline", () => {
  it("records entries", () => {
    const timeline = new Timeline();
    timeline.record("render", "frame", 16);
    expect(timeline.count).toBe(1);
  });

  it("records render events", () => {
    const timeline = new Timeline();
    timeline.recordRender(16.5);
    expect(timeline.count).toBe(1);
  });

  it("records command events", () => {
    const timeline = new Timeline();
    timeline.recordCommand("CreateNode", 0.5);
    expect(timeline.count).toBe(1);
  });

  it("records event events", () => {
    const timeline = new Timeline();
    timeline.recordEvent("keyboard", "keydown", { key: "a" });
    expect(timeline.count).toBe(1);
  });

  it("filters by category", () => {
    const timeline = new Timeline();
    timeline.record("render", "frame", 16);
    timeline.record("command", "CreateNode");
    expect(timeline.getEntriesByCategory("render")).toHaveLength(1);
  });

  it("gets recent entries", () => {
    const timeline = new Timeline();
    for (let i = 0; i < 10; i++) {
      timeline.record("render", "frame", 16);
    }
    expect(timeline.getRecent(3)).toHaveLength(3);
  });

  it("groups by time window", () => {
    const timeline = new Timeline();
    timeline.record("render", "frame", 16);
    timeline.record("render", "frame", 16);
    const groups = timeline.getGroupedByWindow(1000);
    expect(groups.length).toBeGreaterThanOrEqual(1);
  });

  it("getGroupedByWindow returns empty for empty timeline", () => {
    const timeline = new Timeline();
    const groups = timeline.getGroupedByWindow(1000);
    expect(groups).toHaveLength(0);
  });

  it("getGroupedByWindow splits into multiple windows", () => {
    const timeline = new Timeline();
    for (let i = 0; i < 5; i++) {
      timeline.record("render", "frame", 16);
    }
    const groups = timeline.getGroupedByWindow(0);
    expect(groups.length).toBe(5);
  });

  it("respects maxEntries", () => {
    const timeline = new Timeline({ maxEntries: 3 });
    for (let i = 0; i < 5; i++) {
      timeline.record("render", "frame", 16);
    }
    expect(timeline.count).toBe(3);
  });

  it("getEntriesInRange filters correctly", () => {
    const timeline = new Timeline();
    timeline.record("render", "frame", 16);
    timeline.record("command", "CreateNode");
    timeline.record("render", "frame", 16);
    const entries = timeline.getEntries();
    const start = entries[0]?.timestamp ?? 0;
    const end = entries[1]?.timestamp ?? 0;
    const result = timeline.getEntriesInRange(start, end);
    expect(result.length).toBeGreaterThanOrEqual(1);
  });

  it("calls onEntry callback", () => {
    let captured = false;
    const timeline = new Timeline({
      onEntry: () => {
        captured = true;
      },
    });
    timeline.record("render", "frame", 16);
    expect(captured).toBe(true);
  });

  it("clears all entries", () => {
    const timeline = new Timeline();
    timeline.record("render", "frame", 16);
    timeline.clear();
    expect(timeline.count).toBe(0);
  });
});

// ─── SnapshotManager ─────────────────────────────────────────────────────────

describe("SnapshotManager", () => {
  it("captures a snapshot", () => {
    const manager = new SnapshotManager();
    const tree = { id: "root", type: "Box", props: {}, children: [] };
    const snapshot = manager.capture(tree);
    expect(snapshot.id).toBe(0);
    expect(snapshot.nodeCount).toBe(1);
  });

  it("captures snapshot with nested nodes", () => {
    const manager = new SnapshotManager();
    const tree = {
      id: "root",
      type: "Box",
      props: {},
      children: [{ id: "child", type: "Text", props: {}, children: [] }],
    };
    const snapshot = manager.capture(tree);
    expect(snapshot.nodeCount).toBe(2);
  });

  it("diffs two snapshots", () => {
    const manager = new SnapshotManager();
    const tree1 = { id: "root", type: "Box", props: {}, children: [] };
    const tree2 = { id: "root", type: "Box", props: { new: true }, children: [] };
    const s1 = manager.capture(tree1);
    const s2 = manager.capture(tree2);
    const diff = manager.diff(s1.id, s2.id);
    expect(diff).not.toBeNull();
    expect(diff?.changed.length).toBeGreaterThan(0);
  });

  it("diff returns null for non-existent snapshot", () => {
    const manager = new SnapshotManager();
    const tree = { id: "root", type: "Box", props: {}, children: [] };
    manager.capture(tree);
    const diff = manager.diff(999, 0);
    expect(diff).toBeNull();
  });

  it("diff returns null for both non-existent", () => {
    const manager = new SnapshotManager();
    const diff = manager.diff(999, 998);
    expect(diff).toBeNull();
  });

  it("diffTrees with identical style and layout does not report changes", () => {
    const manager = new SnapshotManager();
    const tree1 = {
      id: "root",
      type: "Box",
      props: {},
      style: { bold: true },
      layout: { x: 0, y: 0, width: 100, height: 50 },
      children: [],
    };
    const s1 = manager.capture(tree1);
    const s2 = manager.capture({ ...tree1 });
    const diff = manager.diff(s1.id, s2.id);
    expect(diff).not.toBeNull();
    expect(diff?.changed.some((c) => c.field === "style")).toBe(false);
    expect(diff?.changed.some((c) => c.field === "layout")).toBe(false);
  });

  it("detects added nodes", () => {
    const manager = new SnapshotManager();
    const tree1 = { id: "root", type: "Box", props: {}, children: [] };
    const tree2 = {
      id: "root",
      type: "Box",
      props: {},
      children: [{ id: "child", type: "Text", props: {}, children: [] }],
    };
    const s1 = manager.capture(tree1);
    const s2 = manager.capture(tree2);
    const diff = manager.diff(s1.id, s2.id);
    expect(diff?.added).toContain("child");
  });

  it("detects removed nodes", () => {
    const manager = new SnapshotManager();
    const tree1 = {
      id: "root",
      type: "Box",
      props: {},
      children: [{ id: "child", type: "Text", props: {}, children: [] }],
    };
    const tree2 = { id: "root", type: "Box", props: {}, children: [] };
    const s1 = manager.capture(tree1);
    const s2 = manager.capture(tree2);
    const diff = manager.diff(s1.id, s2.id);
    expect(diff?.removed).toContain("child");
  });

  it("detects style changes", () => {
    const manager = new SnapshotManager();
    const tree1 = { id: "root", type: "Box", props: {}, style: { bold: true }, children: [] };
    const tree2 = { id: "root", type: "Box", props: {}, style: { bold: false }, children: [] };
    const s1 = manager.capture(tree1);
    const s2 = manager.capture(tree2);
    const diff = manager.diff(s1.id, s2.id);
    expect(diff?.changed.some((c) => c.field === "style")).toBe(true);
  });

  it("detects layout changes", () => {
    const manager = new SnapshotManager();
    const tree1 = {
      id: "root",
      type: "Box",
      props: {},
      layout: { x: 0, y: 0, width: 100, height: 50 },
      children: [],
    };
    const tree2 = {
      id: "root",
      type: "Box",
      props: {},
      layout: { x: 0, y: 0, width: 200, height: 50 },
      children: [],
    };
    const s1 = manager.capture(tree1);
    const s2 = manager.capture(tree2);
    const diff = manager.diff(s1.id, s2.id);
    expect(diff?.changed.some((c) => c.field === "layout")).toBe(true);
  });

  it("getSnapshot by id returns correct snapshot", () => {
    const manager = new SnapshotManager();
    const tree = { id: "root", type: "Box", props: {}, children: [] };
    const snap = manager.capture(tree);
    expect(manager.getSnapshot(snap.id)).toBeDefined();
    expect(manager.getSnapshot(snap.id)?.id).toBe(snap.id);
  });

  it("getSnapshot by id returns undefined for non-existent", () => {
    const manager = new SnapshotManager();
    expect(manager.getSnapshot(999)).toBeUndefined();
  });

  it("respects maxSnapshots overflow", () => {
    const manager = new SnapshotManager({ maxSnapshots: 3 });
    for (let i = 0; i < 5; i++) {
      manager.capture({ id: `root-${i}`, type: "Box", props: {}, children: [] });
    }
    expect(manager.getSnapshots()).toHaveLength(3);
  });

  it("diffs identical trees returns empty diff", () => {
    const manager = new SnapshotManager();
    const tree = { id: "root", type: "Box", props: {}, children: [] };
    const s1 = manager.capture(tree);
    const s2 = manager.capture({ ...tree });
    const diff = manager.diff(s1.id, s2.id);
    expect(diff).not.toBeNull();
    expect(diff?.added).toHaveLength(0);
    expect(diff?.removed).toHaveLength(0);
    expect(diff?.changed).toHaveLength(0);
  });

  it("clears all snapshots", () => {
    const manager = new SnapshotManager();
    manager.capture({ id: "root", type: "Box", props: {}, children: [] });
    manager.clear();
    expect(manager.getSnapshots()).toHaveLength(0);
  });
});

// ─── Export utilities ────────────────────────────────────────────────────────

describe("export utilities", () => {
  it("createExport returns valid export", () => {
    const data = createExport({});
    expect(data.version).toBe("1.0.0");
    expect(data.logs).toHaveLength(0);
    expect(data.commands).toHaveLength(0);
  });

  it("createExport with includeLogs false", () => {
    const data = createExport({}, { includeLogs: false });
    expect(data.logs).toHaveLength(0);
  });

  it("createExport with includeCommands false", () => {
    const data = createExport({}, { includeCommands: false });
    expect(data.commands).toHaveLength(0);
  });

  it("createExport with includeEvents false", () => {
    const data = createExport({}, { includeEvents: false });
    expect(data.events).toHaveLength(0);
  });

  it("createExport with includeFrames false", () => {
    const data = createExport({}, { includeFrames: false });
    expect(data.frames).toHaveLength(0);
  });

  it("createExport with includeTimeline false", () => {
    const data = createExport({}, { includeTimeline: false });
    expect(data.timeline).toHaveLength(0);
  });

  it("createExport with includeSnapshots false", () => {
    const data = createExport({}, { includeSnapshots: false });
    expect(data.snapshots).toHaveLength(0);
  });

  it("createExport with tree data", () => {
    const tree = { id: "root", type: "Box", props: {}, children: [] };
    const data = createExport({ tree });
    expect(data.tree).toEqual(tree);
  });

  it("createExport with focus data", () => {
    const focus = {
      focusedNodeId: "n1",
      previousNodeId: null,
      focusableNodes: ["n1"],
      tabOrder: ["n1"],
      currentScope: null,
    };
    const data = createExport({ focus });
    expect(data.focus).toEqual(focus);
  });

  it("createExport with capabilities data", () => {
    const capabilities = {
      trueColor: true,
      kittyKeyboard: false,
      mouseSupport: true,
      osc52: false,
      osc8: false,
      pixelSupport: false,
      alternateScreen: false,
      terminalBrand: "kitty",
      terminalSize: { columns: 80, rows: 24 },
      syncUpdate: false,
      bracketedPaste: false,
      focusEvents: false,
      strikethrough: false,
      underlineColor: false,
      cursorStyle: false,
      hyperlinks: false,
      inlineImages: false,
      sixel: false,
    };
    const data = createExport({ capabilities });
    expect(data.capabilities).toEqual(capabilities);
  });

  it("exportToJson returns valid JSON", () => {
    const data = createExport({});
    const json = exportToJson(data);
    expect(() => JSON.parse(json)).not.toThrow();
  });

  it("createSummary returns string report", () => {
    const data = createExport({});
    const summary = createSummary(data);
    expect(summary).toContain("BetterTUI DevTools Diagnostic Report");
  });

  it("createSummary includes scheduler info", () => {
    const data = createExport({
      scheduler: {
        isRunning: true,
        isRendering: false,
        hasScheduledRender: false,
        frameCount: 100,
        droppedFrames: 5,
        pendingFrames: 0,
        highestPriority: "normal",
        idleCallbacksPending: 0,
        animationFramesPending: 0,
        frameBudgetMs: 16.67,
        utilization: 0.8,
      },
    });
    const summary = createSummary(data);
    expect(summary).toContain("Frame Count: 100");
  });

  it("createSummary includes focus info", () => {
    const data = createExport({
      focus: {
        focusedNodeId: "n1",
        previousNodeId: null,
        focusableNodes: ["n1"],
        tabOrder: ["n1"],
        currentScope: null,
      },
    });
    const summary = createSummary(data);
    expect(summary).toContain("Focused: n1");
    expect(summary).toContain("Focusable Nodes: 1");
  });

  it("createSummary with null focus shows none", () => {
    const data = createExport({
      focus: {
        focusedNodeId: null,
        previousNodeId: null,
        focusableNodes: [],
        tabOrder: [],
        currentScope: null,
      },
    });
    const summary = createSummary(data);
    expect(summary).toContain("Focused: none");
  });

  it("createSummary includes capabilities info", () => {
    const data = createExport({
      capabilities: {
        trueColor: true,
        kittyKeyboard: false,
        mouseSupport: false,
        osc52: false,
        osc8: false,
        pixelSupport: false,
        alternateScreen: false,
        terminalBrand: "kitty",
        terminalSize: { columns: 80, rows: 24 },
        syncUpdate: false,
        bracketedPaste: false,
        focusEvents: false,
        strikethrough: false,
        underlineColor: false,
        cursorStyle: false,
        hyperlinks: false,
        inlineImages: false,
        sixel: false,
      },
    });
    const summary = createSummary(data);
    expect(summary).toContain("Brand: kitty");
    expect(summary).toContain("Size: 80x24");
    expect(summary).toContain("True Color: true");
  });

  it("enabled DevTools exportJson returns valid JSON", () => {
    const devtools = createDevTools({ enabled: true });
    devtools.recordCommand("CreateNode", { id: "1" });
    const json = devtools.exportJson();
    const parsed = JSON.parse(json);
    expect(parsed.commands).toHaveLength(1);
  });

  it("enabled DevTools exportJson with options filters data", () => {
    const devtools = createDevTools({ enabled: true });
    devtools.recordCommand("CreateNode", { id: "1" });
    const json = devtools.exportJson({ includeCommands: false });
    const parsed = JSON.parse(json);
    expect(parsed.commands).toHaveLength(0);
  });

  it("enabled DevTools exportData with options filters data", () => {
    const devtools = createDevTools({ enabled: true });
    devtools.recordCommand("CreateNode", { id: "1" });
    devtools.recordKeyboard("a", { ctrl: false, shift: false, alt: false, meta: false });
    const data = devtools.exportData({ includeCommands: false });
    expect(data.commands).toHaveLength(0);
    expect(data.events).toHaveLength(1);
  });
});
