import { describe, expect, it } from "vitest";
import { createDevTools } from "../index";

describe("createDevTools", () => {
  it("returns null when called with no options", () => {
    const result = createDevTools();
    expect(result).toBeNull();
  });

  it("returns null when called with empty options", () => {
    const result = createDevTools({});
    expect(result).toBeNull();
  });

  it("returns null when called with partial options", () => {
    const result = createDevTools({ enabled: true });
    expect(result).toBeNull();
  });

  it("returns null when called with full options", () => {
    const result = createDevTools({ enabled: true, port: 8080 });
    expect(result).toBeNull();
  });
});
