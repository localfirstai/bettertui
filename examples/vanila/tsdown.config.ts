import { defineConfig } from "tsdown";

export default defineConfig({
  entry: [
    "src/index.tsx",
    "src/examples/core/hello-world.tsx",
    "src/examples/core/rendering-engine.tsx",
    "src/examples/layout/flex-layout.tsx",
    "src/examples/layout/grid-layout.tsx",
    "src/examples/containers/scroll-area-basics.tsx",
    "src/examples/containers/list-view.tsx",
    "src/examples/navigation/tabs-navigation.tsx",
    "src/examples/widgets/tree-view.tsx",
    "src/examples/widgets/data-table-basics.tsx",
    "src/examples/typography/text-styles.tsx",
    "src/examples/theming/theming.tsx",
    "src/examples/animation/animation-basics.tsx",
    "src/examples/performance/live-metrics.tsx",
    "src/examples/performance/performance-stress-test.tsx",
    "src/examples/terminal/capabilities.tsx",
  ],
  format: ["esm"],
  clean: true,
  sourcemap: true,
  deps: {
    neverBundle: ["react", "react-reconciler"],
  },
});
