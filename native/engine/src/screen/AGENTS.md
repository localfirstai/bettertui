# AGENTS.md

## ScreenState

- Ties together viewport, scrollback, cursor, alternate screen tracking, and selection into one unified state.
- `CursorState.style` is an enum (Block, Line, Bar) — not a u8 or string.
- `AlternateScreen::is_active()` checks if the alternate screen buffer is in use (e.g., by nvim's `:TUI`).
- Selection tracks both start/end `(row, col)` and direction (`is_forward`).
