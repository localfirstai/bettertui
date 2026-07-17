# AGENTS.md

## Naming Conventions

**When creating any new file, always follow `.claude/rules/naming-convension.md`.** Summary:

- **Rust:** standard Rust nomenclature — `snake_case` files/modules, `PascalCase` types, `SCREAMING_SNAKE_CASE` constants.
- **TypeScript:** camelCase identifiers and camelCase file names for services, utils, and plain modules (e.g. `userService.ts`).
  - Types: `*.types.ts` (e.g. `demo.types.ts`).
  - Examples: `*.example.ts` (e.g. `demo.example.ts`).
  - TS widgets (non-React): `PascalCase.ts` (e.g. `Button.ts`).
- **React:** components are `kebab-case.tsx` (e.g. `text-input.tsx`); hooks are `useHookName.ts` (e.g. `useFocus.ts`).

## TurboRepo

- Build outputs must be `["dist/**"]`, not `[".next/**"]` — the default starter is Next.js-specific.
- `turbo.json` tasks: `build`, `dev`, `lint`, `format`, `format:check`, `typecheck`, `clean`. No `check-types` (renamed to `typecheck`).
- `format` and `format:check` tasks have `cache: false` since formatting is fast and should always run.
- **Cache cascading**: Modifying a shared dep (like `@bettertui/core`) triggers cache misses for all downstream packages. Running individual package builds masks broken dependents — always run `pnpm build` from root after a refactor.

## pnpm Workspace

- `pnpm-workspace.yaml` currently includes all packages under `packages/*`. The Rust engine lives in `packages/core/crates/engine/` and is built as a `cdylib` with the `napi` feature to produce the `bettertui_engine.node` addon (no separate `crates/bindings` directory).
- Use `pnpm@9.15.0` (pinned in `packageManager` field).
- **Package deletion order**: Update all dependents' package.json FIRST (remove the deleted package from their deps), THEN run `pnpm install -r`. The lockfile auto-regenerates. Deleting the package dir before updating dependents causes `pnpm install` to fail with unresolved workspace deps.

## Biome

- Configured in `biome.json` at root. All packages run `biome check src/` for lint.
- Union types must be single-line if they fit — biome rejects multi-line unions that fit on one line.
- Function signatures: biome formats single-line params when they fit on one line (e.g., `function foo(a: string, b: number): void` not multi-line).
- Biome is the **only** formatter and linter for TypeScript/JavaScript/JSON. No Prettier, no ESLint.
- VCS integration is enabled: `biome.json` has `vcs.enabled = true` with `useIgnoreFile = true`, so `.gitignore` is respected automatically.
- The `.husky/pre-commit` hook runs `biome check --write --staged .` on staged files only.

## Rust + TypeScript Interop

- The native addon is built from `packages/core/crates/engine/` with the `napi` feature via `napi build` (see `packages/core/package.json` `build:native` script); the engine crate's `build.rs` and `napi` feature handle `napi_build::setup()`.
- Rust structs with `new()` must also implement `Default` or clippy will error.
- `impl Into<String>` in function signatures creates monomorphizations — prefer accepting `String` directly or use a different pattern.

## Rust 2024 Edition

- **Unsafe blocks inside unsafe fns required.** Even inside an `unsafe fn`, you need an explicit `unsafe { }` block for unsafe operations. This is a 2024 edition change.
- **`div_ceil` is stable since 1.73.** No need to manually reimplement `(a + b - 1) / b` — use `a.div_ceil(b)`.
- **`sort_by_key` requires `Ord`.** The key function must return a type that implements `Ord`.

## Husky

- Installed as a dev dependency. Initialized via `pnpm exec husky init`.
- `prepare` script in root `package.json` runs `husky` to install git hooks.
- Pre-commit hook runs `biome check --write --staged` then `pnpm run cargo:fmt` (which passes `--manifest-path packages/core/Cargo.toml`) then `git update-index --again`.
- **Pre-commit requires node in PATH.** The shell doesn't have node unless configured. Prepend `$HOME/.nvm/versions/node/v24.15.0/bin` before git commands that trigger hooks, or commit will fail with `exec: node: not found`.

## GitHub Actions

- CI workflow in `.github/workflows/ci.yml`.
- Runs Biome CI, Rust formatting check, Clippy with `-D warnings`, TypeScript typecheck, and workspace build.
- Separate job for `cargo test --manifest-path packages/core/Cargo.toml`.

## Architecture

- **Canonical design docs:** `docs/architecture/` contains the definitive architecture. Root `ARCHITECTURE.md`, `ROADMAP.md`, `CONTRIBUTING.md` are older summaries — prefer docs/architecture/ for design decisions.
- **Two first-class packages.** `@bettertui/core` is the **public, first-class package for vanilla / native TypeScript** (framework-agnostic: CommandBuffer, Command protocol, Runtime class, tree manipulation, createReconciler(), native bridge). `@bettertui/react` is the **public, first-class React adapter** — React apps install only `@bettertui/react`, which depends on core and pulls it in automatically. Nothing may bypass Core.
- **`@bettertui/core` is the framework-agnostic foundation:** Contains the CommandBuffer, Command protocol, Runtime class, tree manipulation (Instance, TextInstance), HostConfig types, and the framework-agnostic createReconciler(). Zero React dependency. Future adapters (Vue, Solid, Svelte) depend on core directly.
- **`@bettertui/react` is the React adapter:** Absorbs the old `@bettertui/reconciler` and `@bettertui/runtime` packages. Contains the react-reconciler HostConfig, createRenderer, render(), RuntimeProvider, useRuntime, hooks (useTheme, useFocus, useKeyboard, etc.), and component stubs. Internal implementation details are not exported.
- **`@bettertui/reconciler` and `@bettertui/runtime` removed:** Absorbed into `@bettertui/react` (React-specific parts) and `@bettertui/core` (framework-agnostic parts). Do not reference these packages.
- **`@bettertui/core` owns both TypeScript runtime and Rust engine:** The engine bridge (internal to core as `src/platform/`) imports `Command` and `CommandBuffer` from core. The Rust crates live in `packages/core/crates/` — `engine`, `widgets`, `terminal`, and `bindings`. Everything from the Rust crates is exposed through `@bettertui/core` only.
- **`@bettertui/shared` is the type foundation:** Pure type definitions, zero runtime dependencies. Both core and react re-export shared types.
- **No `@bettertui/testing` package:** Testing is done with per-package Vitest suites (e.g. `*.test.ts` next to source). There is no separate testing package or headless harness — React output is asserted via `renderToStringAsync` in `packages/react/src/testing.ts`. Do not create `@bettertui/testing`.
- **Proposed but not yet created packages:** The architecture documents reference packages that don't exist yet: `@bettertui/protocol`, `@bettertui/renderer`, `@bettertui/hooks`, `@bettertui/animations`, `@bettertui/editor`, `@bettertui/graphics`.
- **Node model design:** The architecture specifies `slotmap`-based arena allocation with generational indices (`NodeId` = `slotmap::DefaultKey`, 8 bytes). The TypeScript `NodeId` is currently `string` — this will need to change when the Rust engine is implemented.

## Rust Workspace

- **There is no root `Cargo.toml`.** The Rust workspace root is `packages/core/Cargo.toml`. All cargo commands from the project root must pass `--manifest-path packages/core/Cargo.toml`. Root `package.json` scripts (`cargo:*`) include this flag automatically.
- **Pre-commit runs `pnpm run cargo:fmt` (not bare `cargo fmt`)** because there's no root Cargo.toml. The script passes `--manifest-path packages/core/Cargo.toml`.

## Rust Engine Testing

- Run engine tests: `cargo test --manifest-path packages/core/Cargo.toml --lib`
- Run all tests: `cargo test --manifest-path packages/core/Cargo.toml`
- Run clippy: `cargo clippy --manifest-path packages/core/Cargo.toml --lib -- -D warnings`
- All structs with `new()` must have `#[derive(Default)]` or manual Default impl.
- Module inception lint: `foo/foo.rs` triggers it — rename inner file (e.g., `foo/core.rs`).
- The only Rust crate with a unit-test suite co-located in `#[cfg(test)]` blocks is `bettertui-engine`. (The engine test build currently has a compilation issue in `terminal/vt.rs` that must be fixed before the suite is green.)
- **Orphaned `tests.rs` files** — if `mod.rs` already has `#[cfg(test)] mod tests { ... }` with inline tests AND a separate `tests.rs` file exists, delete the `tests.rs`. Rustc fails with duplicate module definitions.
