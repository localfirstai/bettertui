# Changelog

All notable changes to `@bettertui/core` are documented here. This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] — 2026-08-31

### Added
- Word-level editing in the `Input` widget: Alt+Left / Alt+B (move word backward), Alt+Right / Alt+F (move word forward), Ctrl+W / Alt+Backspace (delete word backward), Alt+D / Alt+Delete (delete word forward).
- `wordBoundaryLeft()` and `wordBoundaryRight()` utility functions exported from `@bettertui/core/lib` — TypeScript ports of the Rust `TextBuffer` word-boundary algorithms.
- `measureText` empty-text test locking in the `{ width: 0, height: 1 }` contract.
- Word boundary test suite (13 tests).

### Fixed
- Rust engine: empty text layout measurement returns zero width instead of 1, preventing blank text nodes from occupying a cell column during Taffy layout.

### Changed
- Rust engine: `EditBuffer::delete_word_backward` reimplemented with proper `word_boundary_left` instead of buggy manual character counting.
- Rust engine: `KeyInput::to_key_event()` added for bridging raw terminal input to the keybinding system.

## [0.1.0] — 2026-08-18

Initial npm release.
