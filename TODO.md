# TypeScript 5.x → 7.x Migration Report

## Migration Complete ✅

**Date**: July 12, 2026
**TypeScript version**: 5.9.3 → 7.0.2

## Final Validation Results

| Check | Status | Details |
|-------|--------|---------|
| `pnpm install` | ✅ | TypeScript 7.0.2 installed |
| `pnpm typecheck` | ✅ | 7/7 workspaces pass (zero TS errors) |
| `pnpm build` | ✅ | 22/22 workspaces build successfully |
| `pnpm test` | ✅ | 38/38 tests pass (28 core + 10 themes) |
| `pnpm lint` | ✅ | All core packages pass |
| Declaration files | ✅ | `.d.ts` files generated via `tsc --emitDeclarationOnly` |
| Public API | ✅ | No breaking changes |
| Runtime behavior | ✅ | No regressions |

## Changes Made

### Dependencies Upgraded
| Package | Before | After |
|---------|--------|-------|
| `typescript` (catalog) | `^5.7.0` | `^7.0.2` |
| `typescript` (apps/performance) | `^5` (hardcoded) | `catalog:` |

### Files Modified

#### Root Configuration
- `pnpm-workspace.yaml` - Updated TypeScript catalog version
- `tsconfig.json` - Removed `declarationMap: true` (not supported in tsgo)
- `README.md` - Updated "TypeScript 5+" → "TypeScript 7+"

#### tsup Configs (removed `dts: true`)
- `packages/core/tsup.config.ts`
- `packages/react/tsup.config.ts`
- `packages/shared/tsup.config.ts`
- `packages/themes/tsup.config.ts`
- `packages/devtools/tsup.config.ts`

#### Build Scripts (added `tsc --emitDeclarationOnly`)
- `packages/core/package.json`
- `packages/react/package.json`
- `packages/shared/package.json`
- `packages/themes/package.json`
- `packages/devtools/package.json`

#### tsconfig Fixes
- `packages/react/tsconfig.json` - Added `"lib": ["ES2024", "dom"]` for DOM types
- `apps/website/tsconfig.json` - Removed `baseUrl: "."`
- `apps/performance/tsconfig.json` - Removed `baseUrl: "."`

#### Script Changes
- `apps/website/package.json` - Changed `typecheck` from `astro check` to `tsc --noEmit`
- `apps/performance/package.json` - Changed `typescript` to use catalog

#### Code Fixes
- `apps/website/src/lib/bench/frameworks.ts` - Changed `private collector` to `collector` (pre-existing private access bug)

#### Documentation Updates
- `docs/api/README.md` - Updated tsup build description
- `docs/architecture/Overview.md` - Updated tsup build description
- `packages/AGENTS.md` - Updated build process description

#### Lint Fixes
- `apps/website/src/lib/bench/bettertui-runner.ts` - Biome formatting
- `apps/website/src/lib/bench/frameworks.ts` - Biome formatting
- `apps/website/src/lib/bench/sample-data.ts` - Biome formatting

## Compiler Options Changed

| Option | Before | After | Reason |
|--------|--------|-------|--------|
| `declarationMap` | `true` | removed | Not supported in tsgo (TS 7 Go compiler) |
| `lib` (packages/react) | inherited `["ES2024"]` | `["ES2024", "dom"]` | TS 7 requires explicit DOM types |

## Build Pipeline Change

**Before**: `tsup` (with `dts: true`) generated both JS and `.d.ts` files

**After**: Two-step process:
1. `tsup` generates bundled JS only
2. `tsc --emitDeclarationOnly --declaration --outDir dist` generates `.d.ts` files

This change was necessary because tsup's DTS pipeline (rollup-plugin-dts) hardcodes `baseUrl: '.'` internally, which is removed in TypeScript 7.

## Known Limitations

1. **Declaration maps not available** - tsgo doesn't support `.d.ts.map` files yet. Go-to-definition will navigate to `.d.ts` files instead of source. Will be restored when tsgo adds support.

2. **`astro check` disabled** - Astro's type checker (`@astrojs/check`) doesn't support TS 7 yet. Using `tsc --noEmit` instead. Will be restored when `@astrojs/check` adds TS 7 support.

3. **DTS not bundled** - Declarations are generated as individual files per module (standard TypeScript behavior) instead of a single bundled file. Consumers are unaffected.

4. **`apps/performance` peer dep warning** - `@opentui/core` requires `typescript@^5`, but `strict-peer-dependencies=false` in `.npmrc` allows the build to proceed.

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
