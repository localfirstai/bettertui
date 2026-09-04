# Changelog

All notable changes to `bettertui_engine` are documented here. This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] — 2026-08-31

### Fixed
- `measure_text` now returns zero width for empty strings instead of 1, so blank text nodes no longer push sibling content sideways during layout.

### Added
- `KeyInput::to_key_event()` converts raw terminal input into `event_bus::KeyEvent` for the keybinding system. Returns `None` for unrecognised keys.
- `EditBuffer::delete_word_forward()` deletes from the cursor through the end of the next word via `TextBuffer::word_boundary_right`.
- `EditBuffer::move_word_backward()` and `move_word_forward()` for word-granularity cursor navigation.
- Tests covering `delete_word_backward`, `delete_word_forward`, and `move_word_*` operations.

### Changed
- `EditBuffer::delete_word_backward()` reimplemented to use `TextBuffer::word_boundary_left` instead of manual character counting, fixing incorrect boundary detection.

## [0.1.0] — 2026-08-18

Initial crates.io release.
