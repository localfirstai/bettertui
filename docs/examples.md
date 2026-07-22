# Examples

Runnable examples live in `packages/examples/typescript/` — built directly on `@bettertui/core` and the native Rust engine (no React). 64 examples across categories: rendering, layout, input, widgets, animation, performance, terminal, and more.

Each example exports `meta` + `Example` + `run(keyInput)` + `destroy(keyInput)`.

## Running

```bash
pnpm --filter @bettertui/core build:native
pnpm --filter @bettertui/examples dev            # interactive launcher
pnpm --filter @bettertui/examples dev <slug>     # single example
```

## Other apps

`apps/website` is an Astro/Starlight docs + landing site. It does not depend on engine packages.

## See also

- [@bettertui/core API](api/packages/core.md)
- [Getting started guide](guides/getting-started.md)
