# AGENTS.md

## TurboRepo

- Build outputs must be `["dist/**"]`, not `[".next/**"]` — the default starter is Next.js-specific.
- `turbo.json` tasks: `build`, `dev`, `lint`, `typecheck`, `clean`. No `check-types` (renamed to `typecheck`).

## pnpm Workspace

- `pnpm-workspace.yaml` must include `native/bindings` (has package.json) but NOT `native/engine` (pure Rust).
- Use `pnpm@9.15.0` (pinned in `packageManager` field).

## Biome

- Configured in `biome.json` at root. All packages run `biome check src/` for lint.
- Union types must be single-line if they fit — biome rejects multi-line unions that fit on one line.

## Rust + TypeScript Interop

- `native/bindings/build.rs` must exist and call `napi_build::setup()` for napi-rs to work.
- Rust structs with `new()` must also implement `Default` or clippy will error.
