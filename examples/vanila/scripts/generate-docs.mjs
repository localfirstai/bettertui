#!/usr/bin/env node

// Generates per-example README.md files from each example's meta export.
// Reads the meta from source and writes to docs/examples/<slug>.md.
// Usage: node scripts/generate-docs.mjs

import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const root = resolve(here, "..");

// Inline the META array to avoid a runtime import of registry.ts (which
// imports the entire React tree). This is a docs build script, not a framework
// bundler, and the META array changes infrequently enough that maintaining a
// local copy is simpler than resolving TypeScript at script time.
const META = [
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
    description: "Switchable tabs and expandable accordion sections.",
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
    description: "Bold, italic, dim, underline, and colour treatment.",
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
    description: "Measure FPS and render time under large workloads.",
    category: "performance",
    level: 5,
    tags: ["performance", "setInterval", "DataTable", "Tree", "metrics"],
    next: ["live-metrics", "data-table-basics"],
  },
  {
    slug: "capabilities",
    title: "Terminal Capabilities",
    description: "Detect and display the current terminal's feature set.",
    category: "terminal",
    level: 2,
    tags: ["detectCapabilities", "kittyKeyboard", "trueColor", "focusEvents"],
    next: ["theming", "live-metrics"],
    requires: ["native engine"],
  },
];

const CATEGORY_LABELS = {
  core: "Core",
  layout: "Layout",
  containers: "Containers",
  navigation: "Navigation",
  widgets: "Widgets",
  typography: "Typography",
  theming: "Theming",
  animation: "Animation",
  performance: "Performance",
  terminal: "Terminal",
};

const docsDir = resolve(root, "docs");
mkdirSync(docsDir, { recursive: true });

const indexLines = [
  "# @bettertui/examples — Generated Docs",
  "",
  "Auto-generated per-example documentation.",
  "",
  `Total: ${META.length} examples`,
  "",
  "## Index",
  "",
];

for (const meta of META) {
  const slug = meta.slug;
  const catLabel = CATEGORY_LABELS[meta.category] || meta.category;

  const md = [
    `# ${meta.title}`,
    "",
    `**Category:** ${catLabel}  `,
    `**Level:** ${meta.level}/5  `,
    `**Slug:** \`${slug}\`  `,
    `**Tags:** ${meta.tags.join(", ")}`,
    "",
    meta.description,
    "",
    "## How to run",
    "",
    "### From source (standalone)",
    "```bash",
    `bun examples/src/examples/${meta.category}/${slug}.tsx`,
    "```",
    "",
    "### Via launcher",
    "```bash",
    "node dist/index.mjs",
    "# then select from the interactive menu",
    "# or directly:",
    `node dist/index.mjs ${slug}`,
    "```",
    "",
    "### Bundled standalone",
    "```bash",
    `node dist/examples/${meta.category}/${slug}.mjs`,
    "```",
    "",
  ];

  if (meta.next && meta.next.length > 0) {
    md.push("## Next examples", "");
    for (const next of meta.next) {
      md.push(`- [${next}](./${next}.md)`);
    }
    md.push("");
  }

  md.push("---", "", "Generated by `scripts/generate-docs.mjs`");

  writeFileSync(resolve(docsDir, `${slug}.md`), md.join("\n"));

  indexLines.push(`- [${meta.title}](${slug}.md) — ${catLabel}, L${meta.level}`);
}

indexLines.push("");
indexLines.push("---");
indexLines.push("Generated by `scripts/generate-docs.mjs`");

writeFileSync(resolve(docsDir, "index.md"), indexLines.join("\n"));

console.log(`✅ Generated ${META.length} example docs + index in docs/`);
