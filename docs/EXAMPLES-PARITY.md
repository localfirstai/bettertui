# Examples — OpenTUI Parity & Gap Report

This document records how the rebuilt `@bettertui/examples` suite compares to OpenTUI's
`packages/examples`, and the honoured gaps. It is the deliverable parity report for the
example-suite restructure task.

## What was rebuilt

The previous `@bettertui/examples` was **broken**: `src/index.tsx` imported `./lib/meta` and
`./registry`, both missing, and there was no `tsconfig.json`, no `theme.ts`, and no `lib/`
folder. None of the 9 flat example files built or ran.

The suite was rebuilt from the OpenTUI reference as the source of truth:

- **Interactive launcher** (`src/index.tsx`) mirrors OpenTUI's `ExampleSelector`:
  category-grouped, scrollable, filterable menu; live filter with `Tab`/`Esc` focus toggle;
  `Enter` to run, `Esc`/`q` to return-to-menu (calling `destroy`); `t` dark/light theme toggle;
  instructions bar; `ctrl+c` quit.
- **Example contract** standardised to `meta` + `Example` + `run(keyInput)` + `destroy(keyInput)`.
- **Shared `src/lib/`** infrastructure: `meta`, `registry`, `keyboard` (`KeyInput` TTY manager),
  `keyboard-context`, `runtime-keys`, `tab-controller`, `hex-list`, `palette-grid`, `theme`,
  `standalone`. All internal to the package.
- **Category-grouped folder structure** (`src/examples/<category>/`) — strictly better than
  OpenTUI's flat `src/*.ts` dump.
- **Standalone build** via `scripts/build.ts` (tsdown) producing `dist/index.mjs`.
- **9 migrated + 6 new examples = 15**, across 10 categories.
- **Docs**: per-example READMEs (`generate-docs.mjs`), landing-page `README.md`, this report.

## Parity matrix

| OpenTUI behaviour | BetterTUI | Status |
|-------------------|-----------|--------|
| Category-grouped menu | `CATEGORY_ORDER` sections in `List` | ✅ |
| Live filter input (name/desc/category) | `KeyInput` + `recompute()` | ✅ |
| Filter ⇄ list focus toggle (`Tab`/`Esc`) | `setFocus("filter"\|"list")` | ✅ |
| `↑↓`/`j`/`k` navigation (shift = fast) | `moveSelection` | ✅ |
| `Enter` runs selected | `runSelected` → `mountExample` | ✅ |
| `Esc`/`q` returns to menu + `destroy` | `returnToMenu` | ✅ |
| Theme switch (dark/light) | `t` → `Provider theme` | ✅ |
| Instructions bar | `INSTRUCTIONS` constant | ✅ |
| `run`/`destroy` + `import.meta.main` | `run`/`destroy` + `isMainModule` guard | ✅ (adapted, see below) |
| `--list` catalogue mode | `listExamples()` | ✅ |
| Standalone executable (`bun build`) | `scripts/build.ts` (tsdown) | ✅ |
| Time-to-first-draw readout | not reproduced | ⚠️ gap (see below) |

## Honest gaps (not faked)

- **Time-to-first-draw readout.** OpenTUI renders a `TimeToFirstDrawRenderable`. BetterTUI's
  React `render()` path does not expose a first-paint timer; this was omitted rather than faked.
- **3D & Physics category.** OpenTUI ships ~11 three.js/rapier demos. BetterTUI has no 3D/WebGPU
  support, so this category is **omitted entirely** (not stubbed).
- **Input / forms category.** OpenTUI has `input-demo`, `editor-demo`, `select-demo`, etc.
  BetterTUI's `Input`/`Textarea` host components exist but are not yet wired to a TTY editor
  (no cursor/insertion in the native engine). This category is **not added** until the framework
  supports terminal text editing — documented as a gap, not faked.
- **Per-file `import.meta.main` under a bundle.** OpenTUI runs each example as a separate Bun
  entry, so its `if (import.meta.main)` guard fires per file. BetterTUI bundles the launcher into
  a single `dist/index.mjs`; running one example is done via `node dist/index.mjs <slug>` (the
  launcher calls `run()`), which is the documented standalone path. The per-file guard is kept
  in source for clarity but is dormant in the bundle.

## Coverage by category

| BetterTUI category | Examples | OpenTUI category |
|--------------------|----------|------------------|
| `core` | hello-world, rendering-engine | Runtime & Tooling |
| `layout` | flex-layout, grid-layout | Layout & Composition |
| `containers` | scroll-area-basics, list-view | Scroll & Navigation |
| `navigation` | tabs-navigation | Scroll & Navigation / Input |
| `widgets` | tree-view, data-table-basics | Text & Documents / Layout |
| `typography` | text-styles | Text & Documents |
| `theming` | theming | (theme toggle in launcher) |
| `animation` | animation-basics | Rendering & Effects |
| `performance` | live-metrics, performance-stress-test | Runtime & Tooling |
| `terminal` | capabilities | Terminal & Native |

## Verification

- `pnpm --filter @bettertui/examples typecheck` — clean
- `pnpm --filter @bettertui/examples test` — 12 vitest cases pass (registry integrity + KeyInput)
- `biome check src/` — clean
- `pnpm --filter @bettertui/examples build` — produces `dist/index.mjs`
- `node dist/index.mjs --list` — prints the 15-example catalogue
- `node dist/index.mjs <slug>` — runs and quits cleanly on `q` for every example
- `node scripts/generate-docs.mjs` — regenerates 15 per-example READMEs

## Branding

All component names, strings, and labels are BetterTUI's own (`ExampleSelector`, `KeyInput`,
`useExampleKey`, `BetterTUI Examples`). No OpenTUI strings or branding were copied.
