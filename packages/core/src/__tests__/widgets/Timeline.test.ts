import { describe, expect, it } from "vitest";
import { Timeline } from "../../widgets/Timeline";

describe("Timeline", () => {
  it("constructs with default options", () => {
    const tl = new Timeline();
    expect(tl.isPlaying()).toBe(true);
  });

  it("constructs stopped when autoPlay false", () => {
    const tl = new Timeline({ autoPlay: false });
    expect(tl.isPlaying()).toBe(false);
  });

  it("addTween returns numeric index", () => {
    const tl = new Timeline();
    const idx = tl.addTween({ from: 0, to: 100, duration: 1.0 });
    expect(typeof idx).toBe("number");
  });

  it("animationValue returns number after tick", () => {
    const tl = new Timeline({ duration: 1.0 });
    tl.addTween({ from: 0, to: 100, duration: 1.0 });
    tl.tick(0.5);
    const val = tl.animationValue(0);
    expect(val).not.toBeNull();
    if (val !== null) {
      expect(val).toBeGreaterThanOrEqual(0);
      expect(val).toBeLessThanOrEqual(100);
    }
  });

  it("progress returns 0..1 when duration set", () => {
    const tl = new Timeline({ duration: 2.0 });
    tl.tick(1.0);
    const p = tl.progress();
    expect(p).not.toBeNull();
    if (p !== null) {
      expect(p).toBeGreaterThanOrEqual(0);
      expect(p).toBeLessThanOrEqual(1);
    }
  });

  it("play/pause toggle isPlaying", () => {
    const tl = new Timeline();
    tl.pause();
    expect(tl.isPlaying()).toBe(false);
    tl.play();
    expect(tl.isPlaying()).toBe(true);
  });

  it("onComplete fires when timeline finishes", () => {
    let done = false;
    const tl = new Timeline({
      duration: 0.1,
      onComplete: () => {
        done = true;
      },
    });
    tl.tick(0.2);
    expect(done).toBe(true);
  });

  it("renderCommands creates zero-size Box", () => {
    const tl = new Timeline();
    const cmds = tl.renderCommands("tl1");
    expect(cmds[0]?.type).toBe("CreateNode");
    const w = cmds.find((c) => c.type === "SetWidth");
    const h = cmds.find((c) => c.type === "SetHeight");
    expect(w).toBeDefined();
    expect(h).toBeDefined();
  });

  it("setSpeed changes playback rate", () => {
    const tl = new Timeline({ duration: 10.0 });
    tl.setSpeed(2.0);
    tl.tick(1.0); // at 2x speed, 1s = 2s elapsed
    const p = tl.progress();
    // progress should be roughly 0.2 (2s / 10s)
    expect(p).not.toBeNull();
    if (p !== null) expect(p).toBeCloseTo(0.2, 1);
  });

  it("restart resets timeline", () => {
    const tl = new Timeline({ duration: 2.0 });
    tl.tick(1.0);
    tl.restart();
    expect(tl.currentTime()).toBeCloseTo(0, 1);
  });
});
