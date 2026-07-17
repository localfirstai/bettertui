# @bettertui/examples

Vanilla TypeScript examples for BetterTUI — a runnable launcher that exercises `@bettertui/core` directly (no React). It is a fully working example terminal UI: a filterable menu that runs each demo against a live `CliRenderer`.

For React examples, see `@bettertui/react` instead. This package targets the framework-agnostic `@bettertui/core` API.

## What's inside

- `src/index.ts` — the example launcher (menu + example registry) built on `CliRenderer`, `BoxRenderable`, `TextRenderable`, `InputRenderable`, `SelectRenderable`, and friends from `@bettertui/core`.
- `src/*.ts` — one file per demo, grouped into categories (Layout, Input, Scroll, Text, Rendering, Runtime, Terminal, 3D & Physics).

## Usage

```bash
pnpm --filter @bettertui/examples dev      # node --experimental-strip-types src/index.ts
pnpm --filter @bettertui/examples typecheck
```

`Tab`/`Esc` switch focus between filter and list; type to filter; `↑↓`/`j`/`k` move; `Enter` runs; `ctrl+c` quits.

## Status

The launcher and core-backed demos run against `@bettertui/core`. A subset of demos (`keymap`, `qrcode`, and the 3D/physics set) import packages (`@bettertui/keymap`, `@bettertui/qrcode`, `@bettertui/three`) that may not yet be part of the BetterTUI workspace — they need to be created or ported before those examples resolve and run.

See [`packages/core/README.md`](../core/README.md) and [`docs/architecture/overview.md`](../../docs/architecture/overview.md).
