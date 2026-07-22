import { defineConfig } from "tsdown";

export default defineConfig([
  {
    entry: ["src/index.ts"],
    format: ["esm"],
    dts: true,
    clean: true,
    sourcemap: true,
    external: ["react", "react-reconciler", "bettertui_engine"],
  },
  {
    entry: ["src/jsx-runtime.ts"],
    format: ["esm"],
    dts: true,
    external: ["react"],
  },
  {
    entry: ["src/jsx-dev-runtime.ts"],
    format: ["esm"],
    dts: true,
    external: ["react"],
  },
]);
