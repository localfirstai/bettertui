import { afterEach, beforeEach, describe, expect, it } from "vitest";
import {
  clearEnvCache,
  env,
  generateEnvColored,
  generateEnvMarkdown,
  getAllEnvVarConfigs,
  getEnvVarConfig,
  registerEnvVar,
} from "../lib/env";

describe("env registry and store", () => {
  const originalEnv = { ...process.env };

  beforeEach(() => {
    clearEnvCache();
  });

  afterEach(() => {
    process.env = { ...originalEnv };
    clearEnvCache();
  });

  it("registers and reads default boolean env vars", () => {
    registerEnvVar({
      name: "TEST_BOOL_VAR",
      description: "Test boolean var",
      type: "boolean",
      default: false,
    });

    expect(getEnvVarConfig("TEST_BOOL_VAR")?.name).toBe("TEST_BOOL_VAR");
    expect(env.TEST_BOOL_VAR).toBe(false);
  });

  it("coerces string process.env values to boolean", () => {
    registerEnvVar({
      name: "TEST_BOOL_COERCE",
      description: "Test boolean coercion",
      type: "boolean",
      default: false,
    });

    process.env.TEST_BOOL_COERCE = "true";
    clearEnvCache();
    expect(env.TEST_BOOL_COERCE).toBe(true);

    process.env.TEST_BOOL_COERCE = "1";
    clearEnvCache();
    expect(env.TEST_BOOL_COERCE).toBe(true);

    process.env.TEST_BOOL_COERCE = "yes";
    clearEnvCache();
    expect(env.TEST_BOOL_COERCE).toBe(true);

    process.env.TEST_BOOL_COERCE = "false";
    clearEnvCache();
    expect(env.TEST_BOOL_COERCE).toBe(false);
  });

  it("coerces number process.env values and validates numbers", () => {
    registerEnvVar({
      name: "TEST_NUM_VAR",
      description: "Test number var",
      type: "number",
      default: 42,
    });

    expect(env.TEST_NUM_VAR).toBe(42);

    process.env.TEST_NUM_VAR = "100";
    clearEnvCache();
    expect(env.TEST_NUM_VAR).toBe(100);

    process.env.TEST_NUM_VAR = "not-a-number";
    clearEnvCache();
    expect(() => env.TEST_NUM_VAR).toThrow(/must be a valid number/);
  });

  it("generates env markdown and colored outputs", () => {
    const markdown = generateEnvMarkdown();
    expect(markdown).toContain("# Environment Variables");
    expect(markdown).toContain("BTUI_DEBUG");

    const colored = generateEnvColored();
    expect(colored).toContain("BetterTUI Environment Variables");
    expect(colored).toContain("BTUI_DEBUG");
  });

  it("returns all registered configs", () => {
    const configs = getAllEnvVarConfigs();
    expect(configs.length).toBeGreaterThan(0);
    expect(configs.some((c) => c.name === "BTUI_DEBUG")).toBe(true);
  });
});
