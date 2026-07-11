# AGENTS.md

## TypeScript Packages

- All packages use `tsup` for build, `biome check src/` for lint, `tsc --noEmit` for typecheck.
- tsup config: ESM only, `dts: true`, `clean: true`, `sourcemap: true`.
- Packages with React as a dep must set `external: ["react"]` in tsup config.
- Packages with `react-reconciler` as a dep must also set `external: ["react-reconciler"]`.

## Biome Formatting

- Function signatures: biome formats single-line params when they fit on one line (e.g., `function foo(a: string, b: number): void` not multi-line).
- Union types must be single-line if they fit — biome rejects multi-line unions that fit on one line.

## Common Errors

- **Unused type imports**: TypeScript `noUnusedLocals` catches unused type imports in `.ts` files. This causes DTS generation to fail even though the JS build succeeds. Always check `pnpm build` output for DTS errors.
- **Export type conflicts**: Cannot `export interface Foo` inline AND `export type { Foo }` in the same file — TypeScript errors on conflicting declarations. Pick one export style.
- **Unused imports in host config**: `Command` was imported but unused in the react renderer — TS rejected it. Remove unused imports before committing.

## Package Dependencies

- `@bettertui/react` depends on `@bettertui/core`, `@bettertui/shared`, and `react-reconciler`. Peers `react@^19.0.0`.
- `@bettertui/core` depends on `@bettertui/shared`. No React dependency. Framework-agnostic.
- `@bettertui/native` depends on `@bettertui/core`. No React dependency.
- `@bettertui/themes` depends on `@bettertui/shared`. No React dependency.
- `@bettertui/widgets` depends on `@bettertui/core` and `@bettertui/shared`. No React dependency.
- `@bettertui/devtools` and `@bettertui/icons` have no dependencies.
- All example projects depend on `@bettertui/react` and `@bettertui/core`.

## Removed Packages

- `@bettertui/reconciler` — absorbed into `@bettertui/react` (internal host config)
- `@bettertui/runtime` — split: `Runtime` class -> `@bettertui/core`, wrappers -> `@bettertui/react`

## Placeholder Examples

- `example-table`, `example-dashboard`, `example-text-editor`, `example-mouse`, `example-tree` have `export {}` as their entire source — they are placeholders with no real code. Their `package.json` lists dependencies (`@bettertui/react`, etc.) that are NOT actually imported. Don't waste time debugging build issues in these — `tsup` tolerates empty entries. Real example code lives in `examples/counter` only.
