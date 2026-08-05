import { defineConfig } from "tsdown";

export default defineConfig({
  entry: ["src/index.ts"],
  format: ["esm"],
  dts: true,
  // Clean only TypeScript outputs; exclude the native .node addon so that
  // running tsdown alone does not delete bettertui_engine.node.
  clean: ["dist/*.mjs", "dist/*.mts", "dist/*.d.ts", "dist/*.map"],
  sourcemap: true,
  deps: {
    neverBundle: ["bettertui_engine"],
  },
});
