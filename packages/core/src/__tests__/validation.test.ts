import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  isValidColor,
  validate,
  validateLayoutConstraints,
  validateStyle,
  warnIfInvalid,
} from "../index";

describe("isValidColor", () => {
  it("accepts named colors", () => {
    expect(isValidColor("black")).toBe(true);
    expect(isValidColor("white")).toBe(true);
    expect(isValidColor("red")).toBe(true);
    expect(isValidColor("green")).toBe(true);
    expect(isValidColor("blue")).toBe(true);
    expect(isValidColor("yellow")).toBe(true);
    expect(isValidColor("cyan")).toBe(true);
    expect(isValidColor("magenta")).toBe(true);
    expect(isValidColor("gray")).toBe(true);
    expect(isValidColor("grey")).toBe(true);
    expect(isValidColor("transparent")).toBe(true);
  });

  it("is case insensitive for named colors", () => {
    expect(isValidColor("RED")).toBe(true);
    expect(isValidColor("Blue")).toBe(true);
    expect(isValidColor("Transparent")).toBe(true);
  });

  it("accepts hex colors", () => {
    expect(isValidColor("#fff")).toBe(true);
    expect(isValidColor("#ffffff")).toBe(true);
    expect(isValidColor("#FF0000")).toBe(true);
    expect(isValidColor("#ffffffff")).toBe(true);
    expect(isValidColor("#00000000")).toBe(true);
  });

  it("accepts rgb colors", () => {
    expect(isValidColor("rgb(255, 0, 0)")).toBe(true);
    expect(isValidColor("rgb(0, 255, 0)")).toBe(true);
    expect(isValidColor("rgb(0, 0, 255)")).toBe(true);
  });

  it("accepts rgba colors", () => {
    expect(isValidColor("rgba(255, 0, 0, 0.5)")).toBe(true);
    expect(isValidColor("rgba(0, 255, 0, 1)")).toBe(true);
    expect(isValidColor("rgba(0, 0, 255, 0)")).toBe(true);
  });

  it("rejects invalid colors", () => {
    expect(isValidColor("")).toBe(false);
    expect(isValidColor("notacolor")).toBe(false);
    expect(isValidColor("#xyz")).toBe(false);
    expect(isValidColor("#ff")).toBe(false);
    expect(isValidColor("#ffff")).toBe(false);
    expect(isValidColor("rgba(0, 0, 0)")).toBe(false);
    expect(isValidColor("hsl(0, 0%, 0%)")).toBe(false);
  });
});

describe("validateLayoutConstraints", () => {
  it("returns no errors for empty layout", () => {
    const errors = validateLayoutConstraints({});
    expect(errors).toHaveLength(0);
  });

  it("validates numeric fields are finite", () => {
    const errors = validateLayoutConstraints({
      flexGrow: Number.NaN,
      flexShrink: Number.POSITIVE_INFINITY,
      padding: Number.NEGATIVE_INFINITY,
      width: 100,
    });
    const numericErrors = errors.filter((e) =>
      ["flexGrow", "flexShrink", "padding"].includes(e.field),
    );
    expect(numericErrors).toHaveLength(3);
  });

  it("accepts valid numeric values", () => {
    const errors = validateLayoutConstraints({
      flexGrow: 1,
      flexShrink: 0,
      padding: 10,
      margin: 5,
      width: 200,
      height: 100,
      zIndex: 10,
    });
    expect(errors).toHaveLength(0);
  });

  it("validates percentage strings", () => {
    const errors = validateLayoutConstraints({
      width: "50%",
      height: "100%",
      minWidth: "75%",
    });
    const percentErrors = errors.filter((e) => ["width", "height", "minWidth"].includes(e.field));
    expect(percentErrors).toHaveLength(0);
  });

  it("rejects invalid percentage strings", () => {
    const errors = validateLayoutConstraints({
      width: "notapercent",
      height: "150%",
      minWidth: "-10%",
    });
    expect(errors.some((e) => e.field === "width")).toBe(true);
    expect(errors.some((e) => e.field === "height")).toBe(true);
    expect(errors.some((e) => e.field === "minWidth")).toBe(true);
  });

  it("rejects invalid flexDirection", () => {
    const errors = validateLayoutConstraints({
      flexDirection: "diagonal",
    });
    expect(errors).toHaveLength(1);
    expect(errors[0]?.field).toBe("flexDirection");
  });

  it("accepts valid flexDirection", () => {
    const errors = validateLayoutConstraints({
      flexDirection: "row",
    });
    expect(errors).toHaveLength(0);
  });

  it("rejects invalid justifyContent", () => {
    const errors = validateLayoutConstraints({
      justifyContent: "invalid",
    });
    expect(errors).toHaveLength(1);
    expect(errors[0]?.field).toBe("justifyContent");
  });

  it("accepts valid justifyContent", () => {
    const errors = validateLayoutConstraints({
      justifyContent: "center",
    });
    expect(errors).toHaveLength(0);
  });

  it("rejects invalid alignItems", () => {
    const errors = validateLayoutConstraints({
      alignItems: "invalid",
    });
    expect(errors).toHaveLength(1);
    expect(errors[0]?.field).toBe("alignItems");
  });

  it("accepts valid alignItems", () => {
    const errors = validateLayoutConstraints({
      alignItems: "stretch",
    });
    expect(errors).toHaveLength(0);
  });

  it("rejects invalid alignSelf", () => {
    const errors = validateLayoutConstraints({
      alignSelf: "invalid",
    });
    expect(errors).toHaveLength(1);
    expect(errors[0]?.field).toBe("alignSelf");
  });

  it("accepts valid alignSelf", () => {
    const errors = validateLayoutConstraints({
      alignSelf: "flex-end",
    });
    expect(errors).toHaveLength(0);
  });

  it("rejects invalid position", () => {
    const errors = validateLayoutConstraints({
      position: "fixed",
    });
    expect(errors).toHaveLength(1);
    expect(errors[0]?.field).toBe("position");
  });

  it("accepts valid position", () => {
    const errors = validateLayoutConstraints({
      position: "absolute",
    });
    expect(errors).toHaveLength(0);
  });

  it("rejects invalid overflow", () => {
    const errors = validateLayoutConstraints({
      overflow: "auto",
    });
    expect(errors).toHaveLength(1);
    expect(errors[0]?.field).toBe("overflow");
  });

  it("accepts valid overflow", () => {
    const errors = validateLayoutConstraints({
      overflow: "hidden",
    });
    expect(errors).toHaveLength(0);
  });

  it("rejects invalid flexWrap", () => {
    const errors = validateLayoutConstraints({
      flexWrap: "invalid",
    });
    expect(errors).toHaveLength(1);
    expect(errors[0]?.field).toBe("flexWrap");
  });

  it("accepts valid flexWrap", () => {
    const errors = validateLayoutConstraints({
      flexWrap: "wrap",
    });
    expect(errors).toHaveLength(0);
  });
});

describe("validateStyle", () => {
  it("returns no errors for valid colors", () => {
    const errors = validateStyle({ fg: "red", bg: "#ffffff" });
    expect(errors).toHaveLength(0);
  });

  it("returns no errors when colors are undefined", () => {
    const errors = validateStyle({ bold: true });
    expect(errors).toHaveLength(0);
  });

  it("rejects invalid foreground color", () => {
    const errors = validateStyle({ fg: "notacolor" });
    expect(errors).toHaveLength(1);
    expect(errors[0]?.field).toBe("fg");
  });

  it("rejects invalid background color", () => {
    const errors = validateStyle({ bg: "notacolor" });
    expect(errors).toHaveLength(1);
    expect(errors[0]?.field).toBe("bg");
  });

  it("rejects both invalid colors", () => {
    const errors = validateStyle({ fg: "bad", bg: "worse" });
    expect(errors).toHaveLength(2);
  });
});

describe("validate", () => {
  it("returns valid for no args", () => {
    const result = validate();
    expect(result.valid).toBe(true);
    expect(result.errors).toHaveLength(0);
  });

  it("returns valid for valid layout and style", () => {
    const result = validate({ width: 100 }, { fg: "red" });
    expect(result.valid).toBe(true);
    expect(result.errors).toHaveLength(0);
  });

  it("returns invalid for invalid layout", () => {
    const result = validate({ flexDirection: "diagonal" });
    expect(result.valid).toBe(false);
    expect(result.errors.length).toBeGreaterThan(0);
  });

  it("returns invalid for invalid style", () => {
    const result = validate(undefined, { fg: "bad" });
    expect(result.valid).toBe(false);
    expect(result.errors.length).toBeGreaterThan(0);
  });

  it("returns multiple errors for invalid layout and style", () => {
    const result = validate(
      { flexDirection: "diagonal", position: "fixed" },
      { fg: "bad", bg: "worse" },
    );
    expect(result.valid).toBe(false);
    expect(result.errors.length).toBe(4);
  });
});

describe("warnIfInvalid", () => {
  beforeEach(() => {
    vi.stubEnv("NODE_ENV", "development");
  });

  afterEach(() => {
    vi.unstubAllEnvs();
  });

  it("warns in development mode for invalid props", () => {
    const warnSpy = vi.spyOn(console, "warn").mockImplementation(() => {});
    warnIfInvalid({ flexDirection: "diagonal" }, { fg: "bad" }, "TestComponent");
    expect(warnSpy).toHaveBeenCalledWith("[TestComponent] Invalid props:", expect.any(Array));
  });

  it("uses default component name when not provided", () => {
    const warnSpy = vi.spyOn(console, "warn").mockImplementation(() => {});
    warnIfInvalid({ flexDirection: "diagonal" });
    expect(warnSpy).toHaveBeenCalledWith("[Component] Invalid props:", expect.any(Array));
  });

  it("does not warn for valid props", () => {
    const warnSpy = vi.spyOn(console, "warn").mockImplementation(() => {});
    warnIfInvalid({ width: 100 }, { fg: "red" });
    expect(warnSpy).not.toHaveBeenCalled();
  });

  it("does not warn in production mode", () => {
    vi.stubEnv("NODE_ENV", "production");
    const warnSpy = vi.spyOn(console, "warn").mockImplementation(() => {});
    warnIfInvalid({ flexDirection: "diagonal" });
    expect(warnSpy).not.toHaveBeenCalled();
  });
});
