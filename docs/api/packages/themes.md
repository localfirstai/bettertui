# @bettertui/themes — REMOVED

The standalone `@bettertui/themes` package has been removed.

Theme types (`Theme`, `ThemeColors`, `ThemeSpacing`) now live in `@bettertui/shared` (internal — re-exported by `@bettertui/core` and `@bettertui/react`). The canonical Theme definition lives in the Rust engine (`packages/core/crates/engine/src/theme.rs`). The React `Provider` accepts `Partial<Theme>` directly.

See [Theming guide](../../guides/theming.md) and [themes doc](../../themes.md).
