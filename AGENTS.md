# AGENTS.md

## TurboRepo

- Build outputs must be `["dist/**"]`, not `[".next/**"]` — the default starter is Next.js-specific.
- `turbo.json` tasks: `build`, `dev`, `lint`, `format`, `format:check`, `typecheck`, `clean`. No `check-types` (renamed to `typecheck`).
- `format` and `format:check` tasks have `cache: false` since formatting is fast and should always run.

## pnpm Workspace

- `pnpm-workspace.yaml` must include `native/bindings` (has package.json) but NOT `native/engine` (pure Rust).
- Use `pnpm@9.15.0` (pinned in `packageManager` field).

## Biome

- Configured in `biome.json` at root. All packages run `biome check src/` for lint.
- Union types must be single-line if they fit — biome rejects multi-line unions that fit on one line.
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
- **Dead dependencies:** `@bettertui/reconciler` and `@bettertui/widgets` list `@bettertui/core` as a dependency in package.json but only import from `@bettertui/shared`. The `@bettertui/core` dep is unused.
- **`@bettertui/shared` is the true foundation:** All packages import types from `@bettertui/shared`. `@bettertui/core` is just a re-export layer plus a few extras (NodeType, NodeOptions, TreeDiff). New packages should depend on `@bettertui/shared` directly, not `@bettertui/core`, unless they need the core-specific types.
- **Proposed but not yet created packages:** The architecture documents reference packages that don't exist yet: `@bettertui/protocol`, `@bettertui/renderer`, `@bettertui/hooks`, `@bettertui/testing`, `@bettertui/animations`, `@bettertui/editor`, `@bettertui/graphics`.
- **Node model design:** The architecture specifies `slotmap`-based arena allocation with generational indices (`NodeId` = `slotmap::DefaultKey`, 8 bytes). The TypeScript `NodeId` is currently `string` — this will need to change when the Rust engine is implemented.
