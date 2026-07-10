# AGENTS.md

## Rust Engine

- Edition 2024. All workspace deps defined in root `Cargo.toml` `[workspace.dependencies]`.
- Every engine module (`src/*/mod.rs`) exposes a public struct with `new()` + `Default` impl.
- Clippy: `-D warnings` — no warnings allowed. Structs with `new()` must have `Default`.
- Module inception lint: `engine/engine.rs` triggers `clippy::module-inception` — rename inner file (e.g., `core.rs`) to avoid same name as parent module.

## macOS Compatibility

- BSD `sed` does not support `\n` in replacement strings. Use `write` tool or Python for multi-line edits.
- `cargo fmt --all` sorts `pub mod` declarations alphabetically — don't fight it.

## napi-rs Bindings

- `native/bindings/Cargo.toml` uses `crate-type = ["cdylib"]` for Node.js addon.
- `build.rs` must call `napi_build::setup()`.
- The `#[napi_derive::napi]` attribute exposes functions to Node.js.

## Tree Model Pitfalls

- `NodeId::default()` creates a null key that fails slotmap operations — always use `arena.insert()` to get a valid ID.
- `mark_dirty()` is on `node.state`, not `node` directly — call `node.state.mark_dirty()`.
- RenderNode field names differ from command names: `inverse` (not `inverted`), `strikethrough` (not `crossed_out`), `direction` (not `flex_direction`), `justify` (not `justify_content`), `align` (not `align_items`).
- `overflow` and `transform` are direct fields on `RenderNode`, not on `Visibility`.
- For generic key-value attributes, add `attributes: HashMap<String, String>` to `RenderNode`.
