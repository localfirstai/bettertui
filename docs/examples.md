# Examples

Runnable examples live in the **`@bettertui/examples`** package (`examples/`). Each example is a small, self-contained `.tsx` file in `examples/src/` that imports from `@bettertui/react`, exports a `meta` descriptor, and renders to a terminal via `render()`.

There are **8 examples**, grouped into categories by their `meta.category` value. They are launched through an example browser, not a per-example directory.

## Running an example

```bash
pnpm --filter @bettertui/examples build
node dist/index.mjs <slug>
```

Or browse the catalogue:

```bash
node dist/index.mjs        # interactive menu
node dist/index.mjs --list # compact catalogue
```

## Examples

| Slug | Title | Category | Demonstrates |
|------|-------|----------|--------------|
| `rendering-engine` | Rendering & Engine | rendering | `CommandBuffer`, `createReconciler`, `Runtime`, engine layering |
| `animation-basics` | Animation & Motion | animation | `useAnimation`, `easings`, `useTimeline`, `Progress` |
| `live-metrics` | Live Metrics | performance | `setInterval`, `DataTable`, `Progress`, simulated live data |
| `performance-stress-test` | Performance Stress Test | performance | FPS/render-time metrics, large `DataTable`/`Tree` workloads |
| `scroll-area-basics` | Scroll Area | containers | `ScrollArea`, keyboard scrolling |
| `tabs-navigation` | Tabs & Accordion | navigation | `Tabs`, `Accordion` |
| `theming` | Theming | theming | `Provider` `theme` prop, `useTheme`, live theme switching |
| `tree-view` | Tree View | data-display | `Tree`, `TreeNode`, expand/collapse navigation |

## Status

All 8 examples are wired to the real reconciler. The React component functions are thin wrappers that emit element descriptors; the live native render loop is not yet connected, so examples exercise the full API surface rather than painting pixels to the terminal.

## Other apps

`apps/website` is an Astro/Starlight docs + landing site (`@bettertui/website`). It does **not** depend on the engine packages — it is the documentation portal, not a TUI demo. Benchmarking lives in `packages/benchmark` (Vitest `bench` harness).

## Documentation per example

Generated per-example docs live in `examples/docs/`. They are produced by `examples/scripts/generate-docs.mjs` after a build and cover the subset of examples that have committed `meta` descriptors. Per-example docs that reference slugs without a corresponding `examples/src/*.tsx` source are stale and should be regenerated.
