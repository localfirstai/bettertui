# Migration Report

## Migrations Complete ✅

**Date**: July 12, 2026
**TypeScript**: 5.9.3 → 7.0.2
**Build tool**: tsup 8.5.1 → tsdown 0.22.5

## Final Validation Results

| Check | Status | Details |
|-------|--------|---------|
| `pnpm install` | ✅ | TypeScript 7.0.2, tsdown 0.22.5 installed |
| `pnpm build` | ✅ | 21/21 workspaces build successfully |
| `pnpm typecheck` | ✅ | 7/7 workspaces pass (zero TS errors) |
| `pnpm test` | ✅ | 38/38 tests pass (28 core + 10 themes) |
| `pnpm lint` | ✅ | All core packages pass |
| Public API | ✅ | No breaking changes |
| Runtime behavior | ✅ | No regressions |

## Changes Made

### Dependencies
| Package | Before | After |
|---------|--------|-------|
| `typescript` (catalog) | `^5.7.0` | `^7.0.2` |
| `typescript` (apps/performance) | `^5` (hardcoded) | `catalog:` |
| `tsdown` (catalog) | — | `^0.22.5` |

### Root Configuration
- `pnpm-workspace.yaml` - Updated TypeScript catalog version, added tsdown to catalog
- `tsconfig.json` - Removed `declarationMap: true` (not supported in tsgo)
- `turbo.json` - Updated inputs from `tsup.config.ts` to `tsdown.config.ts`
- `README.md` - Updated "TypeScript 5+" → "TypeScript 7+"

### Build Configs (tsup → tsdown)
- Deleted `tsup.config.ts` for all 5 packages
- Created `tsdown.config.ts` for all 5 packages
- Migrated `external` → `deps.neverBundle` (tsdown API)
- Enabled built-in `dts: true` (replaces separate `tsc --emitDeclarationOnly` step)

### Build Scripts
- **packages**: `"tsup && tsc --emitDeclarationOnly --declaration --outDir dist"` → `"tsdown"`
- **examples**: `tsup --watch` → `tsdown --watch`

### Package Exports (ESM declarations)
- `types` field: `./dist/index.d.ts` → `./dist/index.d.mts`
- `exports` field: `./dist/index.d.ts` → `./dist/index.d.mts`

### tsconfig Fixes
- `packages/react/tsconfig.json` - Added `"lib": ["ES2024", "dom"]` for DOM types
- `apps/website/tsconfig.json` - Removed `baseUrl: "."`
- `apps/performance/tsconfig.json` - Removed `baseUrl: "."`

### Code Fixes
- `apps/website/src/lib/bench/frameworks.ts` - Changed `private collector` to `collector` (pre-existing private access bug)

### Documentation Updates
- `docs/api/README.md`, `docs/architecture/Overview.md`, `docs/examples.md`
- `docs/guides/testing.md`, `docs/guides/getting-started.md`
- `packages/AGENTS.md`, `packages/core/AGENTS.md`, `packages/react/AGENTS.md`
- `packages/core/crates/bindings/README.md`

## Compiler Options Changed

| Option | Before | After | Reason |
|--------|--------|-------|--------|
| `declarationMap` | `true` | removed | Not supported in tsgo (TS 7 Go compiler) |
| `lib` (packages/react) | inherited `["ES2024"]` | `["ES2024", "dom"]` | TS 7 requires explicit DOM types |

## Build Pipeline Change

**Before (tsup)**: `tsup` with `dts: true` — broken in TS 7 because rollup-plugin-dts hardcodes `baseUrl: '.'` internally, which conflicts with TS 7's removal of `baseUrl`.

**After (tsdown)**: `tsdown` with `dts: true` — uses Rolldown's built-in DTS generation, no `baseUrl` injection. Single command generates both JS and `.d.mts` files.

## Known Limitations

1. **Declaration maps not available** - tsgo doesn't support `.d.ts.map` files yet. Go-to-definition will navigate to `.d.ts` files instead of source. Will be restored when tsgo adds support.

2. **`astro check` disabled** - Astro's type checker (`@astrojs/check`) doesn't support TS 7 yet. Using `tsc --noEmit` instead. Will be restored when `@astrojs/check` adds TS 7 support.

3. **`apps/performance` peer dep warning** - `@opentui/core` requires `typescript@^5`, but `strict-peer-dependencies=false` in `.npmrc` allows the build to proceed.

## Workspace Status

| Workspace | Typecheck | Build | Lint | Test |
|-----------|-----------|-------|------|------|
| `@bettertui/shared` | ✅ | ✅ | ✅ | - |
| `@bettertui/core` | ✅ | ✅ | ✅ | ✅ (28 tests) |
| `@bettertui/react` | ✅ | ✅ | ✅ | - |
| `@bettertui/themes` | ✅ | ✅ | ✅ | ✅ (10 tests) |
| `@bettertui/devtools` | ✅ | ✅ | ✅ | - |
| `@bettertui/benchmark` | ✅ | - | ✅ | - |
| `@bettertui/website` | ✅ | ✅ | ⚠️ 2 pre-existing | - |
| `@bettertui/performance` | - | ✅ | - | - |
| 14 examples | - | ✅ | - | - |
