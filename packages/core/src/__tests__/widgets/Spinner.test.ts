import { describe, expect, it } from "vitest";
import { Spinner } from "../../widgets/Spinner";

describe("Spinner", () => {
  it("constructs with default options", () => {
    const s = new Spinner();
    expect(s.currentFrame).toBeTruthy();
  });

  it("constructs with variant", () => {
    const s = new Spinner({ variant: "line" });
    expect(s.options.variant).toBe("line");
  });

  it("tick advances the frame", () => {
    const s = new Spinner({ variant: "line" });
    const frame0 = s.currentFrame;
    s.tick();
    const frame1 = s.currentFrame;
    // after one tick the frame should cycle
    expect(frame1).not.toBe(frame0);
  });

  it("renderCommands creates Box node", () => {
    const s = new Spinner();
    const cmds = s.renderCommands("sp1");
    expect(cmds[0]?.type).toBe("CreateNode");
    if (cmds[0]?.type === "CreateNode") {
      expect(cmds[0].kind).toBe("Box");
    }
  });

  it("renderCommands includes frame text node", () => {
    const s = new Spinner();
    const cmds = s.renderCommands("sp1");
    const frameCreate = cmds.find(
      (c) => c.type === "CreateNode" && "id" in c && c.id === "sp1-frame",
    );
    expect(frameCreate).toBeDefined();
  });

  it("renderCommands includes label when set", () => {
    const s = new Spinner({ label: "Loading" });
    const cmds = s.renderCommands("sp1");
    const labelCreate = cmds.find(
      (c) => c.type === "CreateNode" && "id" in c && c.id === "sp1-label",
    );
    expect(labelCreate).toBeDefined();
  });

  it("renderCommands omits label when not set", () => {
    const s = new Spinner();
    const cmds = s.renderCommands("sp1");
    const labelCreate = cmds.find(
      (c) => c.type === "CreateNode" && "id" in c && c.id === "sp1-label",
    );
    expect(labelCreate).toBeUndefined();
  });

  it("applies color to frame node", () => {
    const s = new Spinner({ color: "blue" });
    const cmds = s.renderCommands("sp1");
    const setFg = cmds.find((c) => c.type === "SetForeground" && "id" in c && c.id === "sp1-frame");
    expect(setFg).toBeDefined();
  });

  it("start with custom scheduler calls tick without real timer", () => {
    const s = new Spinner({ variant: "line" });
    const frame0 = s.currentFrame;
    let tickCb: (() => void) | null = null;
    const stop = s.start(80, (cb) => {
      tickCb = cb;
      return () => {
        tickCb = null;
      };
    });
    expect(tickCb).not.toBeNull();
    // Local alias needed: TS 5.4 closure-assignment narrowing widens `tickCb` to `never` at call site
    const tick = tickCb as unknown as () => void;
    tick();
    expect(s.currentFrame).not.toBe(frame0);
    stop();
    expect(tickCb).toBeNull();
  });

  it("stop clears custom scheduler", () => {
    const s = new Spinner();
    let stopped = false;
    s.start(80, (_cb) => () => {
      stopped = true;
    });
    s.stop();
    expect(stopped).toBe(true);
  });
});
