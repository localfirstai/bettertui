# @bettertui/examples

Runnable example applications for BetterTUI. Every example is a small, self-contained
module under `src/examples/<category>/` that exports a `meta` descriptor plus `run`/`destroy`
contracts, and renders through the React `render()` API. The package is launched by an
interactive browser that mirrors the experience of OpenTUI's example launcher.

## Overview

BetterTUI's example suite is organised **by capability category** (one folder per category)
rather than as a flat file dump, so it scales cleanly to hundreds of examples. Examples depend
on `@bettertui/react` → `@bettertui/core` → the Rust engine, and never bypass core.

All examples are wired to the real React reconciler and the native render loop. Interactive
keyboard input is handled by an internal `KeyInput` manager (the public `useKeyboard` hook only
fires on DOM `keydown` events, which never occur in a Node TTY), so each example reads keypresses
through `useExampleKey`.

## Getting started

```bash
# from the repo root
pnpm install

# build the example executable
pnpm --filter @bettertui/examples build

# run one example directly
node dist/index.mjs hello-world

# compact catalogue
node dist/index.mjs --list

# interactive browser (run in a real terminal)
pnpm --filter @bettertui/examples dev
# or:
node dist/index.mjs
```

## Running one example

```bash
node dist/index.mjs <slug>
```

Slugs are listed by `node dist/index.mjs --list`. Each example quits on `q` (or `Escape`).

## The interactive browser

Running `node dist/index.mjs` (or `pnpm dev`) with a TTY opens the **ExampleSelector**:

- A **category-grouped, scrollable, filterable** menu.
- A live **filter input** — type to search by name, description, tags, or category; press
  `Tab`/`Esc` to switch focus between filter and list; `/` jumps straight to the filter.
- `↑`/`↓` (or `j`/`k`, `Shift` for fast-scroll) move the selection; `Enter` runs the example.
- Inside an example, `Esc`/`q` returns to the menu and calls `destroy`.
- `t` toggles the **dark/light theme** live; `ctrl+c` quits.
- An **instructions bar** shows the key bindings.

## Directory guide

```
examples/
├── package.json
├── tsconfig.json
├── tsdown.config.ts          # bundler config (one entry per example for standalone .mjs)
├── vitest.config.ts
├── scripts/
│   ├── build.ts              # standalone artifact build (tsdown)
│   ├── generate-docs.mjs     # per-example README from meta
│   └── navigate.mjs          # CLI catalogue / search
├── src/
│   ├── index.tsx             # launcher / ExampleSelector (mirrors OpenTUI index.ts)
│   ├── lib/                  # shared infra (internal only)
│   │   ├── meta.ts           # ExampleMeta, categories, labels
│   │   ├── registry.ts       # aggregated catalogue + lazy loaders
│   │   ├── keyboard.ts       # KeyInput TTY keyboard manager
│   │   ├── keyboard-context.tsx
│   │   ├── runtime-keys.tsx  # shared dev keybindings
│   │   ├── tab-controller.tsx
│   │   ├── hex-list.tsx
│   │   ├── palette-grid.tsx
│   │   ├── theme.ts          # example themes (dark/light)
│   │   └── standalone.tsx    # mount helpers
│   └── examples/             # one folder per category
│       ├── core/  layout/  containers/  navigation/  widgets/
│       └── typography/  theming/  animation/  performance/  terminal/
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

## Categories

| Category | Examples |
|----------|----------|
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
2. Export `meta` (slug, title, description, category, level 1–5, tags, next?).
3. Export a component and `run(keyInput)` / `destroy(keyInput)` that mount and tear down.
4. Add a `LOADERS[slug]` entry in `src/lib/registry.ts` and the `META` entry.
5. Run `pnpm --filter @bettertui/examples test` and `biome check src/`.

## Contributing

Keep examples small, focused, and self-contained. Use BetterTUI's own component and API names
(never copy OpenTUI strings/branding). Ensure `biome`, `tsc`, and `vitest` stay clean.

## Status

The examples exercise the full reconciler and native render loop. Interactive keyboard input uses
the internal `KeyInput` manager so examples behave correctly in a real terminal. See
[`docs/EXAMPLES-PARITY.md`](../docs/EXAMPLES-PARITY.md) for the OpenTUI parity and gap report.
