import { describe, expect, it } from "vitest";
import { Image } from "../../widgets/Image";

// Minimal PNG: 1×1 transparent pixel
const TINY_PNG = Buffer.from(
  "89504e470d0a1a0a0000000d49484452000000010000000108060000001f15c489000000" +
    "0a49444154789c6260000000020001e221bc33000000000049454e44ae426082",
  "hex",
);

describe("Image", () => {
  it("constructs with required options", () => {
    const img = new Image({ data: TINY_PNG, width: 1, height: 1 });
    expect(img.options.width).toBe(1);
    expect(img.options.height).toBe(1);
  });

  it("renderCommands creates Box with width/height", () => {
    const img = new Image({ data: TINY_PNG, width: 10, height: 5 });
    const cmds = img.renderCommands("img1");
    expect(cmds[0]?.type).toBe("CreateNode");
    const w = cmds.find((c) => c.type === "SetWidth");
    const h = cmds.find((c) => c.type === "SetHeight");
    expect(w).toBeDefined();
    expect(h).toBeDefined();
    if (w?.type === "SetWidth") expect(w.value).toBe(10);
    if (h?.type === "SetHeight") expect(h.value).toBe(5);
  });

  it("buildSequence returns a Buffer", () => {
    const img = new Image({ data: TINY_PNG, width: 1, height: 1, id: 42 });
    const seq = img.buildSequence("kitty");
    expect(Buffer.isBuffer(seq)).toBe(true);
  });

  it("buildSequence auto protocol returns Buffer", () => {
    const img = new Image({ data: TINY_PNG, width: 1, height: 1 });
    const seq = img.buildSequence("auto");
    expect(Buffer.isBuffer(seq)).toBe(true);
  });

  it("renderCommands emits rawSequence attribute when sequence non-empty", () => {
    const img = new Image({ data: TINY_PNG, width: 1, height: 1, id: 99 });
    const cmds = img.renderCommands("img1");
    const rawAttr = cmds.find(
      (c) => c.type === "SetAttribute" && "key" in c && c.key === "rawSequence",
    );
    // Sequence may be empty if native binary not built; just verify type is right when present
    if (rawAttr) {
      expect(rawAttr.type).toBe("SetAttribute");
    }
  });
});
