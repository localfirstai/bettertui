import { defineConfig } from "tsdown";

export default defineConfig({
  entry: ["src/index.tsx"],
  format: ["esm"],
  clean: true,
  sourcemap: true,
  deps: {
    neverBundle: ["react", "react-reconciler"],
  },
});
