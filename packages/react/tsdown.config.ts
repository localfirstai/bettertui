import { defineConfig } from "tsdown";

export default defineConfig([
  {
    entry: ["src/index.ts"],
    format: ["esm"],
    dts: true,
    clean: true,
    sourcemap: true,
    deps: {
      neverBundle: ["react", "react-reconciler", "bettertui_engine", "react-devtools-core", "ws"],
    },
  },
  {
    entry: ["src/jsx-runtime.ts"],
    format: ["esm"],
    dts: true,
    deps: {
      neverBundle: ["react"],
    },
  },
  {
    entry: ["src/jsx-dev-runtime.ts"],
    format: ["esm"],
    dts: true,
    deps: {
      neverBundle: ["react"],
    },
  },
]);
