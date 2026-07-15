import { defineConfig } from "tsdown";

export default defineConfig({
  entry: [
    "src/index.ts",
    "src/renderer/index.ts",
    "src/selector.ts",
    "src/examples/index.ts",
    "src/examples/hello-world.ts",
    "src/examples/colors.ts",
    "src/examples/keyboard.ts",
    "src/examples/capabilities.ts",
    "src/examples/flex-layout.ts",
    "src/examples/input-demo.ts",
    "src/examples/select-demo.ts",
    "src/examples/performance.ts",
  ],
  format: ["esm"],
  clean: true,
  sourcemap: true,
  platform: "node",
  target: "node22",
  dts: false,
});
