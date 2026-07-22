import { describe, expect, it, vi } from "vitest";
import { Spring, Tween, clamp, easing, inverseLerp, lerp, smoothstep } from "../animations";

// ── easing ────────────────────────────────────────────────────────────────────

describe("easing functions", () => {
  it("linear returns t unchanged", () => {
    expect(easing.linear(0)).toBe(0);
    expect(easing.linear(0.5)).toBe(0.5);
    expect(easing.linear(1)).toBe(1);
  });

  it("all ease functions return 0 at t=0 and 1 at t=1", () => {
    const fns = [
      "easeInQuad",
      "easeOutQuad",
      "easeInOutQuad",
      "easeInCubic",
      "easeOutCubic",
      "easeInOutCubic",
      "easeInSine",
      "easeOutSine",
      "easeInOutSine",
      "easeInExpo",
      "easeOutExpo",
      "easeInCirc",
      "easeOutCirc",
      "easeOutBounce",
      "easeInBounce",
    ] as const;
    for (const name of fns) {
      expect(easing[name](0)).toBeCloseTo(0, 5);
      expect(easing[name](1)).toBeCloseTo(1, 5);
    }
  });
});

// ── math helpers ──────────────────────────────────────────────────────────────

describe("lerp", () => {
  it("interpolates correctly at midpoint", () => {
    expect(lerp(0, 100, 0.5)).toBe(50);
  });
  it("clamps t to [0,1]", () => {
    expect(lerp(0, 100, -1)).toBe(0);
    expect(lerp(0, 100, 2)).toBe(100);
  });
});

describe("inverseLerp", () => {
  it("returns 0 at a", () => expect(inverseLerp(0, 100, 0)).toBe(0));
  it("returns 1 at b", () => expect(inverseLerp(0, 100, 100)).toBe(1));
  it("returns 0.5 at midpoint", () => expect(inverseLerp(0, 100, 50)).toBe(0.5));
  it("handles a === b (returns 0)", () => expect(inverseLerp(5, 5, 5)).toBe(0));
});

describe("smoothstep", () => {
  it("returns 0 at edge a and 1 at edge b", () => {
    expect(smoothstep(0, 1, 0)).toBe(0);
    expect(smoothstep(0, 1, 1)).toBe(1);
  });
  it("returns 0.5 at midpoint", () => {
    expect(smoothstep(0, 1, 0.5)).toBe(0.5);
  });
});

describe("clamp", () => {
  it("clamps below min", () => expect(clamp(-5, 0, 10)).toBe(0));
  it("clamps above max", () => expect(clamp(15, 0, 10)).toBe(10));
  it("passes through values in range", () => expect(clamp(5, 0, 10)).toBe(5));
});

// ── Tween ─────────────────────────────────────────────────────────────────────

describe("Tween", () => {
  it("starts at from value", () => {
    const tw = new Tween({ from: 0, to: 100, duration: 1 });
    expect(tw.value).toBe(0);
    expect(tw.isComplete).toBe(false);
  });

  it("reaches to value after full duration tick", () => {
    const tw = new Tween({ from: 0, to: 100, duration: 1 });
    tw.play();
    tw.tick(1);
    expect(tw.value).toBe(100);
    expect(tw.isComplete).toBe(true);
  });

  it("calls onUpdate on each tick", () => {
    const onUpdate = vi.fn();
    const tw = new Tween({ from: 0, to: 10, duration: 1, onUpdate });
    tw.play();
    tw.tick(0.5);
    expect(onUpdate).toHaveBeenCalledWith(5);
  });

  it("calls onComplete when done", () => {
    const onComplete = vi.fn();
    const tw = new Tween({ from: 0, to: 1, duration: 1, onComplete });
    tw.play();
    tw.tick(2);
    expect(onComplete).toHaveBeenCalledOnce();
  });

  it("does not tick when paused", () => {
    const tw = new Tween({ from: 0, to: 100, duration: 1 });
    tw.tick(0.5);
    expect(tw.value).toBe(0); // not playing
  });

  it("reset returns to start", () => {
    const tw = new Tween({ from: 0, to: 100, duration: 1 });
    tw.play();
    tw.tick(0.5);
    tw.reset();
    expect(tw.value).toBe(0);
    expect(tw.isComplete).toBe(false);
  });
});

// ── Spring ────────────────────────────────────────────────────────────────────

describe("Spring", () => {
  it("starts at initial position", () => {
    const s = new Spring({ target: 100, initial: 0 });
    expect(s.position).toBe(0);
  });

  it("moves toward target on tick", () => {
    const s = new Spring({ target: 100, initial: 0, frequency: 5, damping: 1 });
    s.tick(0.016);
    expect(s.position).toBeGreaterThan(0);
    expect(s.position).toBeLessThan(100);
  });

  it("settles after many ticks", () => {
    const s = new Spring({ target: 50, initial: 0, frequency: 2, damping: 1 });
    for (let i = 0; i < 312; i++) s.tick(0.016);
    expect(s.isSettled(0.5)).toBe(true);
  });

  it("snaps to target instantly", () => {
    const s = new Spring({ target: 100, initial: 0 });
    s.snap();
    expect(s.position).toBe(100);
    expect(s.velocity).toBe(0);
    expect(s.isSettled()).toBe(true);
  });

  it("settles after many ticks (critically damped)", () => {
    const s = new Spring({ target: 50, initial: 0, frequency: 10, damping: 1 });
    for (let i = 0; i < 200; i++) s.tick(0.016);
    expect(s.isSettled()).toBe(true);
  });

  it("target setter moves target", () => {
    const s = new Spring({ target: 0, initial: 0 });
    s.target = 100;
    s.tick(0.1);
    expect(s.position).toBeGreaterThan(0);
  });
});
