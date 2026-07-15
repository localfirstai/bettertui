# @bettertui/examples

Runnable example applications for BetterTUI. Every example is a small, self-contained
module under `src/examples/<category>/` that exports a `meta` descriptor,
`Example` component, `run`/`destroy` contracts, and an `import.meta.main` standalone guard.

The examples are the **living integration tests** of the framework. They prove that the
React reconciler, Rust engine, layout, widgets, events, keyboard, and runtime lifecycle
all work correctly together. If any example does not behave correctly, the framework is
considered broken.

## Overview

BetterTUI's example suite is organised **by capability category** (one folder per category)
rather than as a flat file dump, so it scales cleanly to hundreds of examples. Examples depend
on `@bettertui/react` → `@bettertui/core` → the Rust engine, and never bypass core.

All examples are wired to the real React reconciler and the native render loop. Interactive
keyboard input is handled by an internal `KeyInput` manager.

## Getting started

```bash
# from the repo root
pnpm install
```

### Zero-build usage

Every example runs directly from source — no build step required:

```bash
# interactive launcher (no build step)
pnpm --filter @bettertui/examples dev

# run one example directly
pnpm --filter @bettertui/examples dev hello-world
pnpm --filter @bettertui/examples dev --list

# or from any directory
tsx examples/src/index.tsx              # launcher
tsx examples/src/index.tsx hello-world  # single example
tsx examples/src/index.tsx --list       # catalogue
```

## Run modes

### 1. Standalone from source (primary — no build step)

Every example runs directly from source via its `import.meta.main` guard:

```bash
# from examples/
pnpm exec tsx src/examples/core/hello-world.tsx
pnpm exec tsx src/examples/containers/scroll-area-basics.tsx
pnpm exec tsx src/examples/layout/flex-layout.tsx

# or from repo root
tsx examples/src/examples/core/hello-world.tsx
```

This mirrors OpenTUI's per-file execution capability. No build step required.
Press `q` or `Escape` to quit.

### 2. Interactive launcher (the centrepiece)

```bash
pnpm --filter @bettertui/examples dev
```

Opens the **ExampleSelector** — a category-grouped, scrollable, filterable menu:

- `Tab`/`Esc` — switch focus between filter and list
- Type to filter by name, description, tags, or category
- `↑`/`↓` or `j`/`k` — navigate (`Shift` for fast-scroll)
- `Enter` — run selected example
- `Escape`/`q` inside an example — return to menu
- `t` — toggle dark/light theme
- `ctrl+c` — quit

### 3. Run one via launcher CLI

```bash
pnpm --filter @bettertui/examples dev hello-world   # run directly
pnpm --filter @bettertui/examples dev --list         # compact catalogue
```

### 4. Bundled standalone (single .mjs)

Each example also ships as its own runnable `.mjs` (requires a build step):

```bash
# build first
pnpm --filter @bettertui/examples build

node dist/examples/core/hello-world.mjs
node dist/examples/layout/flex-layout.mjs
```

## Directory guide

```
examples/
├── package.json
├── tsconfig.json
├── tsdown.config.ts          # bundler config (launcher + per-example entries)
├── scripts/
│   ├── build.ts              # standalone artifact build
│   ├── generate-docs.mjs     # per-example README from meta
│   └── run-example.mjs       # CLI helper
├── src/
│   ├── index.tsx             # launcher / ExampleSelector
│   ├── lib/                  # shared infra (internal only)
│   │   ├── meta.ts           # ExampleMeta, categories, labels
│   │   ├── registry.ts       # aggregated catalogue
│   │   ├── keyboard.ts       # KeyInput TTY keyboard manager
│   │   ├── keyboard-context.tsx
│   │   ├── runtime-keys.tsx  # shared dev keybindings
│   │   ├── tab-controller.tsx
│   │   ├── hex-list.tsx
│   │   ├── palette-grid.tsx
│   │   ├── theme.ts          # example themes (dark/light)
│   │   ├── standalone.tsx    # mount helpers
│   │   └── import-meta.d.ts  # import.meta.main type declaration
│   └── examples/
│       ├── core/             # hello-world, rendering-engine
│       ├── layout/           # flex-layout, grid-layout
│       ├── containers/       # scroll-area-basics, list-view
│       ├── navigation/       # tabs-navigation
│       ├── widgets/          # tree-view, data-table-basics
│       ├── typography/       # text-styles
│       ├── theming/          # theming
│       ├── animation/        # animation-basics
│       ├── performance/      # live-metrics, performance-stress-test
│       └── terminal/         # capabilities
├── docs/                     # generated per-example READMEs
└── test/                     # registry + keyboard tests
```

## Learning path

A progressive path through the catalogue:

1. **Core** — `hello-world`, `rendering-engine`
2. **Layout** — `flex-layout`, `grid-layout`
3. **Containers** — `scroll-area-basics`, `list-view`
4. **Widgets** — `tree-view`, `data-table-basics`
5. **Typography** — `text-styles`
6. **Navigation** — `tabs-navigation`
7. **Theming** — `theming`
8. **Animation** — `animation-basics`
9. **Performance** — `live-metrics`, `performance-stress-test`
10. **Terminal** — `capabilities`

Each example's `meta.next` array points to sensible follow-ups.

## Coverage Matrix

Every example validates one or more framework capabilities:

| Capability | Example | Status |
|---|---|---|
| Renderer | hello-world | ✅ |
| Text Rendering | hello-world, text-styles | ✅ |
| Commit Pipeline | hello-world | ✅ |
| CommandBuffer | rendering-engine | ✅ |
| createReconciler | rendering-engine | ✅ |
| Runtime | rendering-engine | ✅ |
| Flex Layout | flex-layout | ✅ |
| Gap | flex-layout, grid-layout | ✅ |
| Alignment | flex-layout | ✅ |
| Sizing | flex-layout | ✅ |
| Grid Layout | grid-layout | ✅ |
| Scroll Area | scroll-area-basics | ✅ |
| Scrollbar | scroll-area-basics | ✅ |
| List Selection | list-view | ✅ |
| Keyboard Navigation | list-view, tree-view | ✅ |
| Tabs | tabs-navigation | ✅ |
| Accordion | tabs-navigation | ✅ |
| Tree Expansion | tree-view | ✅ |
| Tree Selection | tree-view | ✅ |
| Data Table | data-table-basics, live-metrics | ✅ |
| Text Styles | text-styles | ✅ |
| Theming | theming | ✅ |
| Theme Switching | theming | ✅ |
| Animation | animation-basics | ✅ |
| Easing | animation-basics | ✅ |
| Progress | animation-basics | ✅ |
| Scheduler | animation-basics | ✅ |
| Live Data | live-metrics | ✅ |
| setInterval | live-metrics, performance-stress-test | ✅ |
| FPS Metrics | performance-stress-test | ✅ |
| detectCapabilities | capabilities | ✅ |
| Focus Events | capabilities | ✅ |
| True Color | capabilities | ✅ |
| Provider | hello-world, flex-layout, ... | ✅ |
| Separator | hello-world, flex-layout, ... | ✅ |
| StatusLine | scroll-area-basics, list-view, ... | ✅ |
| Badge | tree-view, live-metrics, ... | ✅ |
| Spacer | tree-view, live-metrics, ... | ✅ |

## Categories

| Category | Examples |
|---|---|
| `core` | hello-world, rendering-engine |
| `layout` | flex-layout, grid-layout |
| `containers` | scroll-area-basics, list-view |
| `navigation` | tabs-navigation |
| `widgets` | tree-view, data-table-basics |
| `typography` | text-styles |
| `theming` | theming |
| `animation` | animation-basics |
| `performance` | live-metrics, performance-stress-test |
| `terminal` | capabilities |

## Adding an example

1. Create `src/examples/<category>/<slug>.tsx`.
2. Export `meta`, `Example`, `run(keyInput)`, `destroy(keyInput)`.
3. End with an `import.meta.main` guard for standalone execution.
4. Add a `META` entry in `src/lib/registry.ts` and its import.
5. Add the entry to `tsdown.config.ts` for the bundled artifact.
6. Run `pnpm typecheck` and `biome check src/`.

## Status

These examples exercise the full reconciler and native render loop. Every example can run
standalone from source with `bun`, via the launcher, or as a bundled `.mjs`. Interactive
keyboard input uses the internal `KeyInput` manager so examples behave correctly in a real
terminal. See `docs/architecture/example-parity.md` for the OpenTUI gap report.
