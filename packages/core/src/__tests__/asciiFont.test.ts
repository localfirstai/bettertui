import { describe, expect, it } from "vitest";
import { measureFontText, renderFontToText } from "../lib/asciiFont";
import type { CliRenderer } from "../platform/cliRenderer";
import { ASCIIFontRenderable } from "../renderables/Stubs";

describe("cfonts ASCII Font Rendering", () => {
  it("renders normal text with tiny font", () => {
    const rendered = renderFontToText("BETTERTUI EXAMPLES", "tiny");
    expect(rendered).toBeDefined();
    const lines = rendered.split("\n");
    expect(lines.length).toBe(2);
    expect(lines[0]).toContain("▄");
    expect(lines[1]).toContain("█");
  });

  it("measures normal text with tiny font correctly", () => {
    const measurements = measureFontText("BETTERTUI EXAMPLES", "tiny");
    expect(measurements.height).toBe(2);
    expect(measurements.width).toBeGreaterThan(50);
  });

  it("renders normal text across all supported cfonts styles", () => {
    const fontNames = ["tiny", "block", "shade", "slick", "huge", "grid", "pallet"];
    for (const font of fontNames) {
      const rendered = renderFontToText("TEST TEXT 123", font, "#60A5FA");
      expect(rendered).toBeTruthy();
      const lines = rendered.split("\n");
      expect(lines.length).toBeGreaterThan(1);
    }
  });

  it("ASCIIFontRenderable handles arbitrary normal text", () => {
    const mockRenderer = {
      createNode: () => 1,
      appendChild: () => {},
      removeNode: () => {},
      setNodeLayout: () => {},
      setNodeStyle: () => {},
      setText: () => {},
    };

    const renderable = new ASCIIFontRenderable(mockRenderer as unknown as CliRenderer, {
      text: "CUSTOM USER INPUT",
      font: "tiny",
      color: "#38BDF8",
    });

    expect(renderable.text).toBe("CUSTOM USER INPUT");
    expect(renderable.font).toBe("tiny");
  });
});
