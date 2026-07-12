import { describe, expect, it } from "vitest";
import { easings } from "../hooks";

describe("easings", () => {
  describe("linear", () => {
    it("returns t unchanged", () => {
      expect(easings.linear(0)).toBe(0);
      expect(easings.linear(0.5)).toBe(0.5);
      expect(easings.linear(1)).toBe(1);
    });
  });

  describe("inQuad", () => {
    it("computes quadratic easing", () => {
      expect(easings.inQuad(0)).toBe(0);
      expect(easings.inQuad(0.5)).toBe(0.25);
      expect(easings.inQuad(1)).toBe(1);
    });
  });

  describe("outQuad", () => {
    it("computes out-quad easing", () => {
      expect(easings.outQuad(0)).toBe(0);
      expect(easings.outQuad(0.5)).toBe(0.75);
      expect(easings.outQuad(1)).toBe(1);
    });
  });

  describe("inOutQuad", () => {
    it("computes in-out-quad easing", () => {
      expect(easings.inOutQuad(0)).toBe(0);
      expect(easings.inOutQuad(0.5)).toBe(0.5);
      expect(easings.inOutQuad(1)).toBe(1);
    });
  });

  describe("inCubic", () => {
    it("computes cubic easing", () => {
      expect(easings.inCubic(0)).toBe(0);
      expect(easings.inCubic(0.5)).toBe(0.125);
      expect(easings.inCubic(1)).toBe(1);
    });
  });

  describe("outCubic", () => {
    it("computes out-cubic easing", () => {
      expect(easings.outCubic(0)).toBe(0);
      expect(easings.outCubic(0.5)).toBeCloseTo(0.875);
      expect(easings.outCubic(1)).toBe(1);
    });
  });

  describe("inOutCubic", () => {
    it("computes in-out-cubic easing", () => {
      expect(easings.inOutCubic(0)).toBe(0);
      expect(easings.inOutCubic(0.5)).toBe(0.5);
      expect(easings.inOutCubic(1)).toBe(1);
    });
  });

  describe("inExpo", () => {
    it("computes exponential easing", () => {
      expect(easings.inExpo(0)).toBe(0);
      expect(easings.inExpo(0.5)).toBeCloseTo(0.03125);
      expect(easings.inExpo(1)).toBe(1);
    });
  });

  describe("outExpo", () => {
    it("computes out-expo easing", () => {
      expect(easings.outExpo(0)).toBe(0);
      expect(easings.outExpo(0.5)).toBeCloseTo(0.96875);
      expect(easings.outExpo(1)).toBe(1);
    });
  });

  describe("inOutExpo", () => {
    it("computes in-out-expo easing", () => {
      expect(easings.inOutExpo(0)).toBe(0);
      expect(easings.inOutExpo(1)).toBe(1);
    });
  });

  describe("inSine", () => {
    it("computes sine easing", () => {
      expect(easings.inSine(0)).toBe(0);
      expect(easings.inSine(0.5)).toBeCloseTo(0.292893);
      expect(easings.inSine(1)).toBeCloseTo(1);
    });
  });

  describe("outSine", () => {
    it("computes out-sine easing", () => {
      expect(easings.outSine(0)).toBe(0);
      expect(easings.outSine(0.5)).toBeCloseTo(Math.SQRT1_2);
      expect(easings.outSine(1)).toBeCloseTo(1);
    });
  });

  describe("inOutSine", () => {
    it("computes in-out-sine easing", () => {
      expect(easings.inOutSine(0)).toBeCloseTo(0);
      expect(easings.inOutSine(0.5)).toBeCloseTo(0.5);
      expect(easings.inOutSine(1)).toBeCloseTo(1);
    });
  });

  describe("inBounce", () => {
    it("computes in-bounce easing", () => {
      expect(easings.inBounce(0)).toBe(0);
      expect(easings.inBounce(1)).toBe(1);
    });
  });

  describe("outBounce", () => {
    it("computes out-bounce easing", () => {
      expect(easings.outBounce(0)).toBe(0);
      expect(easings.outBounce(1)).toBe(1);
    });
  });

  describe("inOutBounce", () => {
    it("computes in-out-bounce easing", () => {
      expect(easings.inOutBounce(0)).toBe(0);
      expect(easings.inOutBounce(1)).toBe(1);
    });
  });

  describe("inElastic", () => {
    it("computes elastic easing", () => {
      expect(easings.inElastic(0)).toBe(0);
      expect(easings.inElastic(1)).toBe(1);
    });
  });

  describe("outElastic", () => {
    it("computes out-elastic easing", () => {
      expect(easings.outElastic(0)).toBe(0);
      expect(easings.outElastic(1)).toBe(1);
    });
  });

  describe("inOutElastic", () => {
    it("computes in-out-elastic easing", () => {
      expect(easings.inOutElastic(0)).toBe(0);
      expect(easings.inOutElastic(1)).toBe(1);
    });
  });

  describe("inBack", () => {
    it("computes back-easing", () => {
      expect(easings.inBack(0)).toBeCloseTo(0);
      expect(easings.inBack(1)).toBeCloseTo(1);
    });
  });

  describe("outBack", () => {
    it("computes out-back easing", () => {
      expect(easings.outBack(0)).toBeCloseTo(0);
      expect(easings.outBack(1)).toBe(1);
    });
  });

  describe("inOutBack", () => {
    it("computes in-out-back easing", () => {
      expect(easings.inOutBack(0)).toBeCloseTo(0);
      expect(easings.inOutBack(1)).toBe(1);
    });
  });

  it("all easings return 0 at t=0 and 1 at t=1 with close precision", () => {
    for (const [name, fn] of Object.entries(easings)) {
      expect(fn(0), `${name} at t=0`).toBeCloseTo(0, 5);
      expect(fn(1), `${name} at t=1`).toBeCloseTo(1, 5);
    }
  });

  it("all easings return values between -2 and 2 for t in [0,1]", () => {
    for (const fn of Object.values(easings)) {
      for (let i = 0; i <= 10; i++) {
        const t = i / 10;
        const result = fn(t);
        expect(result >= -2 && result <= 2, `t=${t} result=${result}`).toBe(true);
      }
    }
  });

  it("outputs 22 easing functions", () => {
    expect(Object.keys(easings)).toHaveLength(22);
  });
});
