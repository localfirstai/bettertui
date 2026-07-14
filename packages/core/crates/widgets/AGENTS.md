# AGENTS.md

## Crate Structure

- **`bettertui-widgets`** is a separate Rust crate. It depends on `bettertui-engine` for layout, tree, input, and rendering primitives.
- Import engine types via `bettertui_engine::tree::`, `bettertui_engine::input::`, etc.
- Import widget types via `crate::` (e.g., `crate::Widget`, `crate::WidgetContext`).

## Widget Test Imports

- **`super::*` only imports the current file.** Test modules in `pipeline.rs`, `app.rs`, etc. cannot access `WidgetId` or other types from `lib.rs` via `super::*`. Add explicit `use crate::WidgetId;` (and any other needed types) in `#[cfg(test)] mod tests`.

## Widget Trait

- Widget::create returns `WidgetId`. Construct with `WidgetId(node_id)` (tuple struct).
- WidgetContext requires lifetime: `WidgetContext<'_>` in return types.

## NodeArena

- `append_child()` returns `Result<(), TreeError>`. Handle with `let _ = ...` or propagate.
- No `insert_at()` method — use `insert()` then `append_child()`.

## Flex Widget

- Use `FlexDirection::Column` (not `Direction::Vertical`).
- No child API on FlexWidget itself — attach children via `ctx.append_child(parent, child)` after creation.

## Theme

- No `is_dark()` method. Store mode separately or check `theme.colors.is_empty()`.
- ThemeToken is an enum (21 variants), not a string key.

## Key Handling

- `Key::Character(char)` is the variant name (not `Key::Char`).
- Key enum variants are PascalCase: `Key::Enter`, `Key::Esc`, `Key::Backspace`.

## Derived Defaults

- BoxWidget and ContainerWidget need `#[derive(Default)]` — clippy fires on manual impl that matches derived behavior.
