# Examples

Example applications live under `examples/`. There are **14 examples** across two groups, all with real implementations in `src/index.tsx` built on `@bettertui/core` and `@bettertui/react`.

## Fundamentals (`examples/fundamentals/`)

| Example | Path | Demonstrates |
|---------|------|--------------|
| `hello-world` | `examples/fundamentals/hello-world` | Basic `Box`/`Flex`/`Text` layout, `q` to quit |
| `counter` | `examples/fundamentals/counter` | State management, keyboard input, `Badge`/`StatusLine` |
| `layouts` | `examples/fundamentals/layouts` | Flex-direction, padding/margin toggles, `Grid` |
| `forms` | `examples/fundamentals/forms` | `Input`/`Textarea`/`Checkbox`/`Radio`/`Switch`/`Slider` |
| `tables` | `examples/fundamentals/tables` | `Table`/`DataTable` with sample data |
| `tree` | `examples/fundamentals/tree` | `Tree` component with nested nodes |
| `terminal` | `examples/fundamentals/terminal` | Unicode, box-drawing, and Nerd Font glyph rendering |

## Showcases (`examples/showcase/`)

| Example | Path | Demonstrates |
|---------|------|--------------|
| `dashboard` | `examples/showcase/dashboard` | Multi-panel `Grid` layout, `StatCard` composition, `Progress` |
| `widget-gallery` | `examples/showcase/widget-gallery` | Catalogue of available components |
| `markdown-viewer` | `examples/showcase/markdown-viewer` | Markdown rendering pipeline |
| `system-monitor` | `examples/showcase/system-monitor` | Live-updating stats with `StatusLine` |
| `capability-inspector` | `examples/showcase/capability-inspector` | Terminal capability detection |
| `performance-lab` | `examples/showcase/performance-lab` | Layout/render timing experiments |
| `terminal-showcase` | `examples/showcase/terminal-showcase` | Terminal runtime features |

## Running an example

```bash
cd examples/counter
pnpm exec tsdown src/index.tsx --format esm
node dist/index.mjs
```

## Status

All 14 examples are wired to the real reconciler. The React component functions are thin wrappers that emit element descriptors; the live native render loop is not yet connected, so examples exercise the full API surface rather than painting pixels to the terminal.

## Other apps

`apps/website` is an Astro/Starlight docs + landing site (`@bettertui/website`). It does **not** depend on the engine packages — it is the documentation portal, not a TUI demo. Benchmarking lives in `packages/benchmark` (Vitest `bench` harness).
