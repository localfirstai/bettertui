# @bettertui/themes — REMOVED

The `@bettertui/themes` package has been removed.

Theme types (`Theme`, `ThemeColors`, `ThemeSpacing`) now live in `@bettertui/shared` (internal — re-exported by `@bettertui/react`).
The canonical Theme definition lives in the Rust engine (`packages/core/crates/engine/src/theme.rs`).
The React `Provider` in `@bettertui/react` accepts `Partial<Theme>` directly.
