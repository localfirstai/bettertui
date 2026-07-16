# BetterTUI Vanilla Examples

Runnable example applications for BetterTUI built directly on `@bettertui/core` and the native
Rust engine — no React. They demonstrate the framework-agnostic API: command buffer, native
bridge, `CliRenderer`, and the `KeyInput` keyboard manager.

## Layout

```
examples/vanila/
├── package.json
├── src/
│   ├── index.ts            # Launcher (ExampleSelector) + single-example runner
│   ├── selector.ts         # Interactive example browser
│   ├── lib/                # Shared infra (internal only)
│   │   └── types.ts        # defineExample(), Example contract
│   └── examples/           # One file per example (vanilla .ts)
│       ├── hello-world.ts
│       ├── flex-layout.ts
│       ├── colors.ts
│       ├── capabilities.ts
│       ├── input-demo.ts
│       ├── keyboard.ts
│       ├── performance.ts
│       ├── select-demo.ts
│       └── index.ts        # Registry of all examples
└── README.md
```

(`examples/react`, `examples/rust`, and `examples/solid` exist as reserved, empty directories.)

## Getting started

```bash
# from the repository root — build the native addon first
pnpm --filter @bettertui/core build:native

# run the interactive launcher
pnpm --filter @bettertui/examples-vanila dev

# run a single example by slug
pnpm --filter @bettertui/examples-vanila dev hello-world
pnpm --filter @bettertui/examples-vanila dev keyboard
```

Each example calls `defineExample(category, { name, slug, description, run, destroy })`. `run`
receives a `CliRenderer` (from `@bettertui/core`) and drives the native engine directly; `destroy`
cleans up. The launcher (`index.ts`) wires an `ExampleSelector` that lists examples, runs one on
`Enter`, and returns to the menu on `Escape`.

## Examples

| Slug | Demonstrates |
|------|--------------|
| `hello-world` | Basic text rendering with the native engine |
| `flex-layout` | Taffy flexbox layout in the native renderer |
| `colors` | Colour/style rendering |
| `capabilities` | `detectCapabilities()` terminal feature detection |
| `input-demo` | Native input handling |
| `keyboard` | `KeyInput` keyboard events |
| `performance` | Render-loop performance |
| `select-demo` | Selection / picker UI |

## Adding an example

1. Create `src/examples/<slug>.ts`.
2. Use `defineExample(category, { name, slug, description, run(renderer), destroy(renderer) })`.
3. Register it in `src/examples/index.ts`.
4. Run `pnpm typecheck` and `biome check src/`.

## Status

These examples exercise the `@bettertui/core` API and the native render loop directly. They are
the living integration tests for the vanilla / native TypeScript path.
