# Examples

Runnable examples live in `examples/vanila/` — built directly on `@bettertui/core` and the native
Rust engine (no React). Each example is a small, self-contained module under
`examples/vanila/src/examples/` that calls `defineExample(category, { name, slug, description, run, destroy })`
and renders through the native `CliRenderer`.

(`examples/react`, `examples/rust`, and `examples/solid` exist as reserved, empty directories.)

## Running an example

```bash
# build the native addon first
pnpm --filter @bettertui/core build:native

# interactive launcher
pnpm --filter @bettertui/examples-vanila dev

# run a single example by slug
pnpm --filter @bettertui/examples-vanila dev hello-world
pnpm --filter @bettertui/examples-vanila dev keyboard
```

## Example catalogue

| Slug | Category | Demonstrates |
|------|----------|--------------|
| `hello-world` | Core | Basic text rendering with the native engine |
| `flex-layout` | Layout | Taffy flexbox layout in the native renderer |
| `colors` | Styling | Colour/style rendering |
| `capabilities` | Terminal | `detectCapabilities()` terminal feature detection |
| `input-demo` | Input | Native input handling |
| `keyboard` | Input | `KeyInput` keyboard events |
| `performance` | Performance | Render-loop performance |
| `select-demo` | UI | Selection / picker UI |

## Status

All examples run through the `@bettertui/core` native bridge (`CliRenderer` + `KeyInput`), driving
the Rust engine directly. They are the living integration tests for the vanilla / native
TypeScript path.

## Other apps

`apps/website` is an Astro/Starlight docs + landing site (`@bettertui/website`). It does **not**
depend on the engine packages — it is the documentation portal, not a TUI demo. Benchmarking
lives in `packages/benchmark` (Vitest `bench` harness).
