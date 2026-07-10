# AGENTS.md

## Rust Engine

- Edition 2024. All workspace deps defined in root `Cargo.toml` `[workspace.dependencies]`.
- Every engine module (`src/*/mod.rs`) exposes a public struct with `new()` + `Default` impl.
- Clippy: `-D warnings` — no warnings allowed. Structs with `new()` must have `Default`.

## macOS Compatibility

- BSD `sed` does not support `\n` in replacement strings. Use `write` tool or Python for multi-line edits.
- `cargo fmt --all` sorts `pub mod` declarations alphabetically — don't fight it.

## napi-rs Bindings

- `native/bindings/Cargo.toml` uses `crate-type = ["cdylib"]` for Node.js addon.
- `build.rs` must call `napi_build::setup()`.
- The `#[napi_derive::napi]` attribute exposes functions to Node.js.
