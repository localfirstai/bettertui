import { describe, expect, it } from "vitest";
import { CATEGORY_LABELS } from "../src/lib/meta";
import {
  CATEGORY_ORDER,
  META,
  exampleBySlug,
  examplesByCategory,
  loadExampleModule,
} from "../src/lib/registry";

describe("example registry", () => {
  it("contains exactly the catalogued examples", () => {
    expect(META.length).toBe(15);
  });

  it("has a unique slug per example", () => {
    const slugs = META.map((m) => m.slug);
    expect(new Set(slugs).size).toBe(slugs.length);
  });

  it("covers every supported category", () => {
    const byCat = examplesByCategory();
    for (const cat of CATEGORY_ORDER) {
      expect(byCat.get(cat), `category ${cat} should have examples`).toBeDefined();
      expect(byCat.get(cat)?.length ?? 0).toBeGreaterThan(0);
    }
  });

  it("only uses known categories", () => {
    const known = new Set(Object.keys(CATEGORY_LABELS));
    for (const m of META) {
      expect(known.has(m.category), `unknown category ${m.category}`).toBe(true);
    }
  });

  it("every example has a runnable module export (meta/Example/run/destroy)", async () => {
    for (const meta of META) {
      const mod = await loadExampleModule(meta.slug);
      expect(mod.meta.slug).toBe(meta.slug);
      expect(typeof mod.Example).toBe("function");
      expect(typeof mod.run).toBe("function");
      expect(typeof mod.destroy).toBe("function");
    }
  });

  it("exampleBySlug resolves a known slug and is undefined for unknown", () => {
    expect(exampleBySlug("hello-world")?.slug).toBe("hello-world");
    expect(exampleBySlug("does-not-exist")).toBeUndefined();
  });
});
