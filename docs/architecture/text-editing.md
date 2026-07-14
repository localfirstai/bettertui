# Text Editing

BetterTUI ships a rope-based text editor engine, exposed directly to TypeScript as `NapiTextEngine` (no arena dependency). Core text primitives live in `packages/core/crates/engine/src/text/`; the `text` module (in `text.rs`) exposes `TextEngine` with cursor, selection, undo/redo, and line numbers built on top of the `text/` submodules.

## Engine

```mermaid
classDiagram
    class TextEngine {
        +insert_char(ch)
        +insert_str(s)
        +delete_char()
        +delete_range(start, end)
        +undo()
        +redo()
        +search(pattern, options) Vec~SearchResult~
        +replace(...)
        +line_count() usize
        +char_count() usize
        +word_count() usize
        +line_to_char(line) usize
        +char_to_line(char) usize
    }
    class TextBuffer {
        +ropey::Rope rope
        +insert_char/insert_str/delete_char/delete_range
        +char_at/substring/line/line_to_char/char_to_line
        +word_boundary_left/right
    }
    TextEngine *-- TextBuffer
    TextEngine *-- Cursor
    TextEngine *-- Selection
    TextEngine *-- UndoManager
    TextEngine *-- SearchEngine
```

- `TextBuffer` wraps `ropey::Rope` — efficient editing at any position.
- `Cursor` tracks the caret position.
- `SearchEngine` / `SearchOptions` / `SearchResult` handle find.
- `Selection` / `SelectionRange` handle ranges.
- `UndoManager` with `enum UndoAction`.

## TypeScript surface

`@bettertui/core` exposes `NapiTextEngine` (~25 methods):

| Group | Methods |
|-------|---------|
| Insert | `insertChar`, `insertStr`, `insertText`, `insertAt` |
| Cursor | `cursorLeft/Right/Up/Down`, `cursorLineStart/End`, `cursorPosition`, `setCursorPosition` |
| Delete | `deleteChar`, `deleteCharForward`, `deleteWordBackward/Forward`, `deleteLineBackward/Forward` |
| Query | `charAt`, `substring`, `find`, `replaceAll`, `canUndo`, `canRedo`, `length`, `text`, `lines`, `lineCount`, `isEmpty`, `clear` |

> The selection concept also exists at the screen level (terminal crate's `screen.rs` `selection_*`) for terminal text selection, separate from the `text/selection.rs` submodule.
