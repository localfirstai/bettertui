// Aggregated example catalogue consumed by the launcher and docs tooling.
// Explicit (not auto-discovered) so types stay sound and the menu mirrors
// OpenTUI's static section list. Internal to the examples package.

import type { ExampleCategory, ExampleMeta } from "./meta";
import type { ExampleEntry, ExampleModule } from "./meta";

import * as animationBasics from "~/examples/animation/animation-basics";
import * as listView from "~/examples/containers/list-view";
import * as scrollAreaBasics from "~/examples/containers/scroll-area-basics";
// Statically imported example modules. This is bundler-safe: the examples are
// inlined into the launcher bundle, so their import.meta.url matches the
// launcher and the per-file import.meta.main guard must NOT auto-run them on
// import. The launcher mounts a chosen example via its run() export, which is
// the standalone execution path (node dist/index.mjs <slug>).
import * as helloWorld from "~/examples/core/hello-world";
import * as renderingEngine from "~/examples/core/rendering-engine";
import * as flexLayout from "~/examples/layout/flex-layout";
import * as gridLayout from "~/examples/layout/grid-layout";
import * as tabsNavigation from "~/examples/navigation/tabs-navigation";
import * as liveMetrics from "~/examples/performance/live-metrics";
import * as performanceStressTest from "~/examples/performance/performance-stress-test";
import * as capabilities from "~/examples/terminal/capabilities";
import * as theming from "~/examples/theming/theming";
import * as textStyles from "~/examples/typography/text-styles";
import * as dataTableBasics from "~/examples/widgets/data-table-basics";
import * as treeView from "~/examples/widgets/tree-view";

const MODULES: Record<string, ExampleModule> = {
  "hello-world": helloWorld as unknown as ExampleModule,
  "rendering-engine": renderingEngine as unknown as ExampleModule,
  "flex-layout": flexLayout as unknown as ExampleModule,
  "grid-layout": gridLayout as unknown as ExampleModule,
  "scroll-area-basics": scrollAreaBasics as unknown as ExampleModule,
  "list-view": listView as unknown as ExampleModule,
  "tabs-navigation": tabsNavigation as unknown as ExampleModule,
  "tree-view": treeView as unknown as ExampleModule,
  "data-table-basics": dataTableBasics as unknown as ExampleModule,
  "text-styles": textStyles as unknown as ExampleModule,
  theming: theming as unknown as ExampleModule,
  "animation-basics": animationBasics as unknown as ExampleModule,
  "live-metrics": liveMetrics as unknown as ExampleModule,
  "performance-stress-test": performanceStressTest as unknown as ExampleModule,
  capabilities: capabilities as unknown as ExampleModule,
};

export function loadExampleModule(slug: string): Promise<ExampleModule> {
  const module = MODULES[slug];
  if (!module) throw new Error(`Unknown example slug: ${slug}`);
  return Promise.resolve(module);
}

export const META: ExampleMeta[] = [
  {
    slug: "hello-world",
    title: "Hello World",
    description: "The smallest possible BetterTUI screen rendered through React.",
    category: "core",
    level: 1,
    tags: ["render", "Box", "Text", "Provider"],
    next: ["rendering-engine", "flex-layout"],
  },
  {
    slug: "rendering-engine",
    title: "Rendering & Engine",
    description: "The CommandBuffer + reconciler layer the React API builds on.",
    category: "core",
    level: 4,
    tags: ["CommandBuffer", "createReconciler", "Runtime", "engine"],
    next: ["hello-world", "flex-layout"],
  },
  {
    slug: "flex-layout",
    title: "Flex Layout",
    description: "Row/column flexbox, alignment, and gaps for responsive composition.",
    category: "layout",
    level: 2,
    tags: ["Flex", "alignItems", "justifyContent", "gap"],
    next: ["grid-layout", "scroll-area-basics"],
  },
  {
    slug: "grid-layout",
    title: "Grid Layout",
    description: "Fixed-column grids for dashboards and tabular content.",
    category: "layout",
    level: 2,
    tags: ["Grid", "columns", "gap"],
    next: ["flex-layout", "list-view"],
  },
  {
    slug: "scroll-area-basics",
    title: "Scroll Area",
    description: "Scroll long content vertically with a visible scrollbar.",
    category: "containers",
    level: 2,
    tags: ["ScrollArea", "scrolling"],
    next: ["list-view", "tree-view"],
  },
  {
    slug: "list-view",
    title: "List View",
    description: "Selectable, keyboard-navigable item lists.",
    category: "containers",
    level: 2,
    tags: ["List", "selection", "navigation"],
    next: ["scroll-area-basics", "tree-view"],
  },
  {
    slug: "tabs-navigation",
    title: "Tabs & Accordion",
    description: "Switchable tabs and expandable accordion sections for content organization.",
    category: "navigation",
    level: 2,
    tags: ["Tabs", "TabItem", "Accordion"],
    next: ["list-view", "data-table-basics"],
  },
  {
    slug: "tree-view",
    title: "Tree View",
    description: "Expand/collapse a file-tree with keyboard navigation and selection.",
    category: "widgets",
    level: 2,
    tags: ["Tree", "TreeNode", "navigation"],
    next: ["data-table-basics", "list-view"],
  },
  {
    slug: "data-table-basics",
    title: "Data Table",
    description: "Tabular data with headers, columns, and a selected row.",
    category: "widgets",
    level: 3,
    tags: ["DataTable", "columns", "data"],
    next: ["tree-view", "live-metrics"],
  },
  {
    slug: "text-styles",
    title: "Text Styles",
    description: "Bold, italic, dim, underline, and colour treatment for legible typography.",
    category: "typography",
    level: 1,
    tags: ["Text", "bold", "dim", "color", "underline"],
    next: ["flex-layout", "theming"],
  },
  {
    slug: "theming",
    title: "Theming",
    description: "Apply and switch themes through the Provider's theme prop.",
    category: "theming",
    level: 2,
    tags: ["Provider", "Theme", "useTheme"],
    next: ["text-styles", "animation-basics"],
  },
  {
    slug: "animation-basics",
    title: "Animation & Motion",
    description: "Drive values over time with useAnimation, easings, and useTimeline.",
    category: "animation",
    level: 3,
    tags: ["useAnimation", "easings", "useTimeline", "motion"],
    next: ["theming", "live-metrics"],
  },
  {
    slug: "live-metrics",
    title: "Live Metrics",
    description: "A simulated real-time system dashboard with auto-updating metrics.",
    category: "performance",
    level: 4,
    tags: ["setInterval", "DataTable", "Progress", "live data"],
    next: ["data-table-basics", "performance-stress-test"],
  },
  {
    slug: "performance-stress-test",
    title: "Performance Stress Test",
    description: "Measure FPS and render time under large-table / large-tree workloads.",
    category: "performance",
    level: 5,
    tags: ["performance", "setInterval", "DataTable", "Tree", "metrics"],
    next: ["live-metrics", "data-table-basics"],
  },
  {
    slug: "capabilities",
    title: "Terminal Capabilities",
    description: "Detect and display the current terminal's feature set via the native engine.",
    category: "terminal",
    level: 2,
    tags: ["detectCapabilities", "kittyKeyboard", "trueColor", "focusEvents"],
    next: ["theming", "live-metrics"],
    requires: ["native engine"],
  },
];

export const META_BY_SLUG: Record<string, ExampleMeta> = Object.fromEntries(
  META.map((m) => [m.slug, m]),
);

export const examples: ExampleMeta[] = META;

export function exampleBySlug(slug: string): ExampleMeta | undefined {
  return META_BY_SLUG[slug];
}

export function examplesByCategory(): Map<ExampleCategory, ExampleMeta[]> {
  const map = new Map<ExampleCategory, ExampleMeta[]>();
  for (const meta of META) {
    const list = map.get(meta.category) ?? [];
    list.push(meta);
    map.set(meta.category, list);
  }
  return map;
}

export function exampleEntries(): ExampleEntry[] {
  return META.map((meta) => ({
    meta,
    load: () => loadExampleModule(meta.slug),
  }));
}

export { CATEGORY_ORDER, CATEGORY_LABELS } from "./meta";
