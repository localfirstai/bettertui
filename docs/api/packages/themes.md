# @bettertui/themes — REMOVED

The `@bettertui/themes` package has been removed.

Theme types (`Theme`, `ThemeColors`, `ThemeSpacing`) now live in `@bettertui/shared`.
The canonical Theme definition lives in the Rust engine (`packages/core/crates/widgets/src/theme.rs`).
The React `Provider` in `@bettertui/react` accepts `Partial<Theme>` directly.
