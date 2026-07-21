import { describe, expect, it } from "vitest";
import { Slider } from "../../widgets/Slider";

describe("Slider", () => {
  it("constructs with default options", () => {
    const slider = new Slider();
    expect(slider.options.value).toBeUndefined();
    expect(slider.percent).toBe(0);
  });

  it("constructs with value", () => {
    const slider = new Slider({ value: 50, min: 0, max: 100 });
    expect(slider.options.value).toBe(50);
    expect(slider.percent).toBe(50);
  });

  it("percent returns 0 for zero-range", () => {
    const slider = new Slider({ value: 5, min: 5, max: 5 });
    expect(slider.percent).toBe(0);
  });

  it("percent returns value above max (no clamp)", () => {
    const slider = new Slider({ value: 150, min: 0, max: 100 });
    expect(slider.percent).toBe(150);
  });

  it("renderCommands creates Box node and sets width", () => {
    const slider = new Slider({ value: 50 });
    const cmds = slider.renderCommands("s1");
    expect(cmds.some((c) => c.type === "CreateNode")).toBe(true);
    expect(cmds.some((c) => c.type === "SetWidth")).toBe(true);
  });

  it("handleKey increases value on right", () => {
    let val = 0;
    const slider = new Slider({
      value: 5,
      min: 0,
      max: 10,
      onChange: (v) => {
        val = v;
      },
    });
    slider.handleKey({
      key: "right",
      code: "ArrowRight",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(val).toBe(6);
  });

  it("handleKey decreases value on left", () => {
    let val = 0;
    const slider = new Slider({
      value: 5,
      min: 0,
      max: 10,
      onChange: (v) => {
        val = v;
      },
    });
    slider.handleKey({
      key: "left",
      code: "ArrowLeft",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(val).toBe(4);
  });

  it("handleKey respects min boundary", () => {
    let val = 0;
    const slider = new Slider({
      value: 0,
      min: 0,
      max: 10,
      onChange: (v) => {
        val = v;
      },
    });
    slider.handleKey({
      key: "left",
      code: "ArrowLeft",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(val).toBe(0);
  });

  it("handleKey respects max boundary", () => {
    let val = 0;
    const slider = new Slider({
      value: 10,
      min: 0,
      max: 10,
      onChange: (v) => {
        val = v;
      },
    });
    slider.handleKey({
      key: "right",
      code: "ArrowRight",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(val).toBe(10);
  });

  it("handleKey does nothing when disabled", () => {
    const slider = new Slider({ value: 5, disabled: true });
    expect(
      slider.handleKey({
        key: "right",
        code: "ArrowRight",
        ctrl: false,
        shift: false,
        alt: false,
        meta: false,
        eventType: "press",
        source: "raw",
      }),
    ).toBe(false);
  });

  it("handleKey respects custom step", () => {
    let val = 0;
    const slider = new Slider({
      value: 5,
      min: 0,
      max: 100,
      step: 10,
      onChange: (v) => {
        val = v;
      },
    });
    slider.handleKey({
      key: "right",
      code: "ArrowRight",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
      eventType: "press",
      source: "raw",
    });
    expect(val).toBe(15);
  });
});
