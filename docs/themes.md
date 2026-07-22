# Themes

Theme types (`Theme`, `ThemeColors`, `ThemeSpacing`) live in `@bettertui/shared` (re-exported by `@bettertui/core` and `@bettertui/react`). The canonical Theme definition lives in the Rust engine (`packages/core/crates/engine/src/theme.rs`). The React `Provider` accepts `Partial<Theme>` directly.

See [Theming guide](guides/theming.md) for usage examples and [Architecture](architecture/README.md) for details.
