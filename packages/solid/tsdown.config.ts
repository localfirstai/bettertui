import { defineConfig } from "tsdown";

export default defineConfig({
  entry: ["src/index.ts"],
  format: ["esm"],
  dts: true,
  clean: true,
  sourcemap: true,
  deps: {
    neverBundle: ["solid-js", "solid-js/web", "solid-js/store", "bettertui_engine"],
  },
});
