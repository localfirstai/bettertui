import { describe, expect, it } from "vitest";
import { ProgressBar } from "../../widgets/ProgressBar";

describe("ProgressBar", () => {
  it("constructs with default options", () => {
    const pb = new ProgressBar();
    expect(pb.percent).toBe(0);
  });

  it("calculates percent correctly", () => {
    const pb = new ProgressBar({ value: 50, min: 0, max: 100 });
    expect(pb.percent).toBe(50);
  });

  it("clamps percent to 0-100", () => {
    const pbLow = new ProgressBar({ value: -10, min: 0, max: 100 });
    expect(pbLow.percent).toBe(0);

    const pbHigh = new ProgressBar({ value: 200, min: 0, max: 100 });
    expect(pbHigh.percent).toBe(100);
  });

  it("handles zero range gracefully", () => {
    const pb = new ProgressBar({ value: 5, min: 5, max: 5 });
    expect(pb.percent).toBe(0);
  });

  it("renderCommands creates Box node", () => {
    const pb = new ProgressBar({ value: 50 });
    const cmds = pb.renderCommands("pb1");
    expect(cmds[0]?.type).toBe("CreateNode");
    if (cmds[0]?.type === "CreateNode") {
      expect(cmds[0].kind).toBe("Box");
    }
  });

  it("renderCommands creates track text node", () => {
    const pb = new ProgressBar({ value: 50 });
    const cmds = pb.renderCommands("pb1");
    const trackCreate = cmds.find(
      (c) => c.type === "CreateNode" && "id" in c && c.id === "pb1-track",
    );
    expect(trackCreate).toBeDefined();
  });

  it("renderCommands includes percent label by default", () => {
    const pb = new ProgressBar({ value: 50 });
    const cmds = pb.renderCommands("pb1");
    const labelCreate = cmds.find(
      (c) => c.type === "CreateNode" && "id" in c && c.id === "pb1-label",
    );
    expect(labelCreate).toBeDefined();
  });

  it("renderCommands omits percent label when showPercent=false", () => {
    const pb = new ProgressBar({ value: 50, showPercent: false });
    const cmds = pb.renderCommands("pb1");
    const labelCreate = cmds.find(
      (c) => c.type === "CreateNode" && "id" in c && c.id === "pb1-label",
    );
    expect(labelCreate).toBeUndefined();
  });

  it("update changes value", () => {
    const pb = new ProgressBar({ value: 0 });
    pb.update({ value: 75 });
    expect(pb.percent).toBe(75);
  });
});
