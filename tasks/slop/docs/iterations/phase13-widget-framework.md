# Widget Framework & AI UI Foundation

## Overview

Phase 13 implements a complete widget architecture for powering AI coding applications. The framework provides lightweight widgets that produce arena nodes, integrating with the existing rendering pipeline.

## Architecture

```
Widget → Node Arena → Layout → Render Objects → Painter → ANSI
```

### Core Principles

1. **Widgets are factories, not owners**: Widgets create arena nodes via `WidgetContext`
2. **Object safety**: Widget trait uses `kind()` for identification
3. **Event propagation**: DOM-style capture→target→bubble with `EventDispatcher`
4. **Theme tokens**: Semantic enum lookup, not hardcoded colors

## Module Structure

### Events (`events/`)

- **types.rs**: Event, KeyEvent, MouseEvent, PasteEvent, FocusEvent, BlurEvent, ResizeEvent, LifecycleEvent, EventResult
- **dispatch.rs**: EventDispatcher with capture→target→bubble propagation
- **bus.rs**: EventBus with coalescing, queue management

### Widgets (`widgets/`)

- **mod.rs**: Widget trait, WidgetHost, foundation widget re-exports
- **context.rs**: WidgetContext for widget creation
- **tree.rs**: WidgetTree (WidgetId↔NodeId mapping)
- **registry.rs**: WidgetRegistry factory pattern
- **reconcile.rs**: Reconciler for widget tree diffing
- **theme.rs**: Theme/ThemeToken/SpacingToken system

### Foundation Widgets

- **box_widget.rs**: BoxWidget - basic container
- **text_widget.rs**: TextWidget - text display
- **spacer_widget.rs**: SpacerWidget - flexible spacing
- **flex_widget.rs**: FlexWidget - flexbox layout
- **scroll_area.rs**: ScrollAreaWidget with scroll handling
- **container.rs**: ContainerWidget with title/padding

### Markdown (`widgets/markdown/`)

- **ast.rs**: MarkdownNode, InlineNode, ListItem, TaskItem
- **parser.rs**: Recursive descent parser
- **renderer.rs**: MarkdownRenderer converts AST to widgets

### Prompt Composer (`widgets/prompt_composer.rs`)

- **ComposerState**: Text editing state with cursor, selection, undo/redo, clipboard
- **PromptComposer**: Widget for 3-line editor

### Chat (`widgets/chat/`)

- **types.rs**: Message, Role, ChatStatus, ChatState
- **view.rs**: ChatView widget
- **status.rs**: StatusBar, ThinkingIndicator

### Integration (`widgets/`)

- **app.rs**: AppState with tree, arena, focus, scheduler, theme
- **pipeline.rs**: Pipeline for render tree building

## Usage Examples

### Creating a Widget

```rust
use bettertui_engine::widgets::{BoxWidget, Widget, WidgetContext, WidgetId};
use bettertui_engine::tree::layout::LayoutProps;
use bettertui_engine::tree::style::Style;

struct MyWidget;

impl Widget for MyWidget {
    fn kind(&self) -> &'static str {
        "MyWidget"
    }

    fn create(&self, ctx: &mut WidgetContext) -> WidgetId {
        let layout = LayoutProps::default();
        let style = Style::default();
        let id = ctx.make_box(layout, style);
        WidgetId(id)
    }
}
```

### Using AppState

```rust
use bettertui_engine::widgets::{AppState, WidgetId};
use bettertui_engine::tree::render_node::RenderNode;
use bettertui_engine::tree::node_kind::NodeKind;

let mut state = AppState::new();
let root_nid = state.arena.insert(RenderNode::new(NodeKind::Box));
let root_id = WidgetId(root_nid);
state.set_root(root_id);
```

### Rendering Markdown

```rust
use bettertui_engine::widgets::{MarkdownRenderer, MarkdownParser};

let markdown = "# Hello\n\nThis is **bold** and *italic*.";
let mut parser = MarkdownParser::new(markdown);
let ast = parser.parse();

let renderer = MarkdownRenderer::new();
let widget_id = renderer.render(&ast, &mut ctx);
```

### Using ChatView

```rust
use bettertui_engine::widgets::{ChatView, Message, ChatState};

let view = ChatView::new();
let mut state = ChatState::new();
state.add_message(Message::user("Hello", 100));
state.add_message(Message::assistant("Hi there!", 200));

let widget_id = view.render_message(&state.messages[0], &mut ctx);
```

## Test Results

- **Total tests**: 777
- **All passing**: ✓
- **Clippy clean**: ✓

## Files Modified

- `native/engine/src/lib.rs`: Module declarations
- `native/engine/src/tree/color.rs`: Added `Color::rgb()` constructor
- `native/engine/src/widgets/`: All widget modules (new)
- `native/engine/src/events/`: All event modules (new)

## Next Steps

- Integration with existing terminal rendering
- Real-time streaming support
- Performance optimization
- Accessibility features
