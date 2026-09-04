# @bettertui/examples

> **Website:** [bettertui.dev](https://bettertui.dev) | **Source:** [github.com/localfirstai/bettertui](https://github.com/localfirstai/bettertui)

BetterTUI vanilla TypeScript examples and standalone executable.

64 examples covering rendering, layout, input, widgets, animation, performance, terminal features, and more. Each example is a self-contained module under `src/examples/` that exports `meta` + `Example` + `run(keyInput)` + `destroy(keyInput)`.

## Acknowledgements

Several examples and benchmarks in this package are inspired by [OpenTUI](https://github.com/anomalyco/opentui). We use their example patterns as a reference for our performance benchmarks. Thanks to the OpenTUI team for their excellent work on terminal UI primitives.

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
