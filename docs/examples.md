# Examples

Examples live under `examples/`. Most are placeholder stubs; only one has real wiring.

| Example | Path | Status |
|---------|------|--------|
| `counter` | `examples/counter/src/index.tsx` | **Real** implementation using `@bettertui/core` (`CommandBuffer`, `createReconciler`) + `@bettertui/react` (`Box`, `Flex`, `Provider`, `Text`); raw-mode keyboard counter. **But** `package.json` build entry is `src/index.ts` (the stub), not `index.tsx`. |
| `counter` | `examples/counter/src/index.ts` | Stub: `console.log("Counter example — coming soon")` |
| `dashboard` | `examples/dashboard/src/index.ts` | Stub |
| `mouse` | `examples/mouse/src/index.ts` | Stub |
| `table` | `examples/table/src/index.ts` | Stub |
| `text-editor` | `examples/text-editor/src/index.ts` | Stub |
| `tree` | `examples/tree/src/index.ts` | Stub |

All declare `@bettertui/react` (and `core`/`shared`) as `workspace:*` deps, but five are not implemented.

## Running the wired example

```bash
cd examples/counter
pnpm exec tsup src/index.tsx --format esm
node dist/index.js
```

## Other apps

`apps/website` is an Astro/Starlight docs + landing site (`@bettertui/website`). It does **not** depend on the engine packages — it is the documentation portal, not a TUI demo.

## Status

Examples are scaffolded. Only `counter/index.tsx` demonstrates real engine usage. Document them as *planned*, not functional.
