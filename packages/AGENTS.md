# AGENTS.md

## TypeScript Packages

- All packages use `tsup` for build, `biome check src/` for lint, `tsc --noEmit` for typecheck.
- tsup config: ESM only, `dts: true`, `clean: true`, `sourcemap: true`.
- Packages with React as a dep must set `external: ["react"]` in tsup config.

## Common Errors

- **Unused type imports**: TypeScript `noUnusedLocals` catches unused type imports in `.ts` files. This causes DTS generation to fail even though the JS build succeeds. Always check `pnpm build` output for DTS errors.
- **Export type conflicts**: Cannot `export interface Foo` inline AND `export type { Foo }` in the same file — TypeScript errors on conflicting declarations. Pick one export style.
- **Unused imports in reconciler**: `RenderNode` was imported but unused — TS rejected it. Remove unused imports before committing.

## Package Dependencies

- `@bettertui/reconciler` and `@bettertui/react` peer-depend on `react@^19.0.0`.
- `@bettertui/core` depends on `@bettertui/shared`.
- All example projects depend on `@bettertui/react` and `@bettertui/shared`.
