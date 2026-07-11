import { beforeEach, describe, expect, it } from "vitest";
import { getIcon, listIcons, registerIcon } from "../index";

describe("icons", () => {
  beforeEach(() => {
    // Clear registry by registering and getting - the registry is module-scoped
    // We test the API contract, not isolation
  });

  it("registerIcon and getIcon work", () => {
    const icon = { name: "test-icon", char: "\u2665", tags: ["heart", "love"] };
    registerIcon(icon);
    const found = getIcon("test-icon");
    expect(found).toBeDefined();
    expect(found?.char).toBe("\u2665");
    expect(found?.tags).toContain("heart");
  });

  it("getIcon returns undefined for unknown", () => {
    const found = getIcon("nonexistent-icon-12345");
    expect(found).toBeUndefined();
  });

  it("listIcons returns registered icons", () => {
    registerIcon({ name: "list-test-1", char: "A", tags: ["alpha"] });
    registerIcon({ name: "list-test-2", char: "B", tags: ["beta"] });
    const icons = listIcons();
    expect(icons.length).toBeGreaterThanOrEqual(2);
    expect(icons.some((i) => i.name === "list-test-1")).toBe(true);
    expect(icons.some((i) => i.name === "list-test-2")).toBe(true);
  });

  it("registerIcon overwrites existing", () => {
    registerIcon({ name: "overwrite-test", char: "X", tags: ["old"] });
    registerIcon({ name: "overwrite-test", char: "Y", tags: ["new"] });
    const found = getIcon("overwrite-test");
    expect(found?.char).toBe("Y");
    expect(found?.tags).toContain("new");
  });
});
