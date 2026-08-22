# @bettertui/examples

> **Website:** [bettertui.dev](https://bettertui.dev)

BetterTUI vanilla TypeScript examples and standalone executable.

64 examples covering rendering, layout, input, widgets, animation, performance, terminal features, and more. Each example is a self-contained module under `src/examples/` that exports `meta` + `Example` + `run(keyInput)` + `destroy(keyInput)`.

## Running

```bash
pnpm --filter @bettertui/core build:native
pnpm dev                    # interactive launcher
pnpm dev <slug>             # run a single example
pnpm dev --list             # print catalogue
```

## Structure

```
src/
├── examples/           # 64 .example.ts files
│   ├── core/           # basic rendering, engine
│   ├── layout/         # flexbox, grid
│   ├── input/          # keyboard, mouse
│   ├── widgets/        # tree, table, select, slider
│   ├── animation/      # sprite, timeline
│   ├── performance/    # metrics, stress test
│   ├── terminal/       # capabilities, PTY, VT
│   └── ...             # graphics, fonts, markdown, etc.
├── lib/                # shared infrastructure
├── assets/             # images, textures
└── xterm-web-demo/     # browser-based demo
```

Dependencies: `@bettertui/core` (workspace).

## Related Documentation

- [Examples guide](../../docs/examples.md)
- [@bettertui/core API](../../docs/api/packages/core.md)
