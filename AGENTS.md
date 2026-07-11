# AGENTS.md

## TurboRepo

- Build outputs must be `["dist/**"]`, not `[".next/**"]` — the default starter is Next.js-specific.
- `turbo.json` tasks: `build`, `dev`, `lint`, `format`, `format:check`, `typecheck`, `clean`. No `check-types` (renamed to `typecheck`).
- `format` and `format:check` tasks have `cache: false` since formatting is fast and should always run.
- **Cache cascading**: Modifying a shared dep (like `@bettertui/core`) triggers cache misses for all downstream packages. Running individual package builds masks broken dependents — always run `pnpm build` from root after a refactor.

## pnpm Workspace

- `pnpm-workspace.yaml` must include `native/bindings` (has package.json) but NOT `native/engine` (pure Rust).
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

- `native/bindings/build.rs` must exist and call `napi_build::setup()` for napi-rs to work.
- Rust structs with `new()` must also implement `Default` or clippy will error.

## Husky

- Installed as a dev dependency. Initialized via `pnpm exec husky init`.
- `prepare` script in root `package.json` runs `husky` to install git hooks.
- Pre-commit hook runs `biome check --write --staged` then `cargo fmt --all` then `git update-index --again`.

## GitHub Actions

- CI workflow in `.github/workflows/ci.yml`.
- Runs Biome CI, Rust formatting check, Clippy with `-D warnings`, TypeScript typecheck, and workspace build.
- Separate job for `cargo test --workspace`.

## Architecture

- **Canonical design docs:** `docs/architecture/` contains the definitive architecture. Root `ARCHITECTURE.md`, `ROADMAP.md`, `CONTRIBUTING.md` are older summaries — prefer docs/architecture/ for design decisions.
- **Package layering:** Application → `@bettertui/react` → `@bettertui/core` → `@bettertui/native` → Rust Engine. React may depend on Core. Core may depend on Native. Nothing may bypass Core.
- **`@bettertui/core` is the framework-agnostic foundation:** Contains the CommandBuffer, Command protocol, Runtime class, tree manipulation (Instance, TextInstance), HostConfig types, and the framework-agnostic createReconciler(). Zero React dependency. Future adapters (Vue, Solid, Svelte) depend on core directly.
- **`@bettertui/react` is the React adapter:** Absorbs the old `@bettertui/reconciler` and `@bettertui/runtime` packages. Contains the react-reconciler HostConfig, createRenderer, render(), RuntimeProvider, useRuntime, hooks (useTheme, useFocus, useKeyboard, etc.), and component stubs. Internal implementation details are not exported.
- **`@bettertui/reconciler` and `@bettertui/runtime` removed:** Absorbed into `@bettertui/react` (React-specific parts) and `@bettertui/core` (framework-agnostic parts). Do not reference these packages.
- **`@bettertui/native` depends on `@bettertui/core`**: The native bridge imports `Command` and `CommandBuffer` from core rather than the old reconciler.
- **`@bettertui/shared` is the type foundation:** Pure type definitions, zero runtime dependencies. Both core and react re-export shared types.
- **`@bettertui/widgets`** provides the Widget interface and version constant. Depends on `@bettertui/core`.
- **Proposed but not yet created packages:** The architecture documents reference packages that don't exist yet: `@bettertui/protocol`, `@bettertui/renderer`, `@bettertui/hooks`, `@bettertui/testing`, `@bettertui/animations`, `@bettertui/editor`, `@bettertui/graphics`.
- **Node model design:** The architecture specifies `slotmap`-based arena allocation with generational indices (`NodeId` = `slotmap::DefaultKey`, 8 bytes). The TypeScript `NodeId` is currently `string` — this will need to change when the Rust engine is implemented.

## Rust Engine Testing

- Run tests: `cargo test -p bettertui-engine --lib` — the `--lib` flag excludes integration tests (`tests/` dir). Without it, pre-existing integration failures block CI.
- Run clippy: `cargo clippy -p bettertui-engine --lib -- -D warnings`
- All structs with `new()` must have `#[derive(Default)]` or manual Default impl.
- Module inception lint: `foo/foo.rs` triggers it — rename inner file (e.g., `foo/core.rs`).
- Widget framework has ~100 tests (total engine: ~1071).
