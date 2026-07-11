# Widget Model

> Widgets are the building blocks of BetterTUI applications.
> They compose nodes into reusable, interactive components.

## 1. Overview

A widget is a **reusable, composable unit of UI** that produces a tree of nodes. Widgets are not nodes themselves — they are factories that create and manage nodes.

```
Widget (factory)
    ↓ (creates)
Node Tree (arena-allocated)
    ↓ (rendered by)
Rendering Pipeline
```

### 1.1 Why Widgets, Not Just Nodes?

Nodes are the low-level rendering primitives. Widgets provide:

- **Encapsulation:** A widget manages its own state and behavior.
- **Reusability:** The same widget can be used multiple times in a tree.
- **Composition:** Widgets can contain other widgets.
- **Abstraction:** Widget users don't need to know about nodes.

### 1.2 Widget vs Component

In React terminology, "component" means a function that returns JSX. In BetterTUI, "widget" means a class or factory that manages nodes. The terms are used interchangeably in framework adapters.

## 2. Widget Structure

### 2.1 Widget Trait (Rust)

```rust
pub trait Widget: Send + Sync {
    /// The type of node this widget produces.
    type Node: WidgetNode;

    /// Create the initial node tree.
    fn create(&self, ctx: &WidgetContext) -> Self::Node;

    /// Update the node tree in response to state changes.
    fn update(&self, node: &mut Self::Node, ctx: &WidgetContext, state: &WidgetState);

    /// Handle events.
    fn handle_event(&self, node: &mut Self::Node, ctx: &WidgetContext, event: &Event) -> EventResult;

    /// Destroy the widget and clean up resources.
    fn destroy(&self, node: &mut Self::Node, ctx: &WidgetContext);
}
```

### 2.2 Widget Context

```rust
pub struct WidgetContext {
    pub arena: &mut NodeArena,
    pub animation_engine: &mut AnimationEngine,
    pub focus_manager: &mut FocusManager,
    pub clipboard: &ClipboardManager,
    pub terminal_size: (u16, u16),
}
```

### 2.3 Widget State

```rust
pub struct WidgetState {
    pub local: Box<dyn Any>,
    pub dirty: bool,
}
```

### 2.4 Widget Node

```rust
pub trait WidgetNode {
    fn root(&self) -> NodeId;
    fn children(&self) -> &[NodeId];
}
```

## 3. Built-in Widgets

### 3.1 Box Widget

A flex container that arranges children.

```rust
pub struct BoxWidget {
    pub direction: FlexDirection,
    pub justify: JustifyContent,
    pub align: AlignItems,
    pub padding: Option<RectValues>,
    pub margin: Option<RectValues>,
    pub gap: Option<Gap>,
    pub style: Style,
    pub children: Vec<Box<dyn Widget>>,
}

impl Widget for BoxWidget {
    type Node = BoxNode;

    fn create(&self, ctx: &WidgetContext) -> BoxNode {
        let root = ctx.arena.insert(RenderNode {
            kind: NodeKind::Box,
            layout: LayoutProps {
                direction: self.direction,
                justify: self.justify,
                align: self.align,
                padding: self.padding,
                margin: self.margin,
                gap: self.gap,
                ..Default::default()
            },
            style: self.style.clone(),
            ..RenderNode::default()
        });

        let children: Vec<NodeId> = self.children.iter()
            .map(|child| {
                let child_node = child.create(ctx);
                let child_root = child_node.root();
                ctx.arena.append_child(root, child_root).unwrap();
                child_root
            })
            .collect();

        BoxNode { root, children }
    }
}
```

### 3.2 Text Widget

Displays styled text.

```rust
pub struct TextWidget {
    pub content: String,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub color: Option<Color>,
    pub bg_color: Option<Color>,
}

impl Widget for TextWidget {
    type Node = TextNode;

    fn create(&self, ctx: &WidgetContext) -> TextNode {
        let root = ctx.arena.insert(RenderNode {
            kind: NodeKind::Text,
            text: Some(self.content.clone().into_boxed_str()),
            style: Style {
                fg: self.color,
                bg: self.bg_color,
                bold: Some(self.bold),
                italic: Some(self.italic),
                underline: Some(self.underline),
                ..Default::default()
            },
            ..RenderNode::default()
        });

        TextNode { root }
    }
}
```

### 3.3 Input Widget

Single-line text input.

```rust
pub struct InputWidget {
    pub placeholder: String,
    pub value: String,
    pub cursor_position: usize,
    pub style: Style,
    pub focus_style: Style,
}

impl Widget for InputWidget {
    type Node = InputNode;

    fn create(&self, ctx: &WidgetContext) -> InputNode {
        let root = ctx.arena.insert(RenderNode {
            kind: NodeKind::Input,
            text: Some(self.value.clone().into_boxed_str()),
            focus: FocusProps {
                focusable: true,
                ..Default::default()
            },
            cursor: Some(CursorProps {
                style: CursorStyle::Bar,
                blink: true,
                position: Some(Point { x: self.cursor_position as u16, y: 0 }),
            }),
            style: self.style.clone(),
            ..RenderNode::default()
        });

        InputNode { root }
    }

    fn handle_event(&self, node: &mut InputNode, ctx: &WidgetContext, event: &Event) -> EventResult {
        match event {
            Event::Key(key_event) => {
                match key_event.key {
                    Key::Character(ch) => {
                        // Insert character at cursor position
                        EventResult::Consumed
                    }
                    Key::Backspace => {
                        // Delete character before cursor
                        EventResult::Consumed
                    }
                    Key::ArrowLeft => {
                        // Move cursor left
                        EventResult::Consumed
                    }
                    _ => EventResult::Ignored,
                }
            }
            _ => EventResult::Ignored,
        }
    }
}
```

### 3.4 List Widget

Scrollable list with selection.

```rust
pub struct ListWidget {
    pub items: Vec<ListItem>,
    pub selected: usize,
    pub style: Style,
    pub selected_style: Style,
}

pub struct ListItem {
    pub label: String,
    pub value: Box<dyn Any>,
}

impl Widget for ListWidget {
    type Node = ListNode;

    fn create(&self, ctx: &WidgetContext) -> ListNode {
        let root = ctx.arena.insert(RenderNode {
            kind: NodeKind::List,
            overflow: Overflow::Scroll,
            focus: FocusProps {
                focusable: true,
                ..Default::default()
            },
            style: self.style.clone(),
            ..RenderNode::default()
        });

        let children: Vec<NodeId> = self.items.iter().enumerate().map(|(i, item)| {
            let child = ctx.arena.insert(RenderNode {
                kind: NodeKind::Text,
                text: Some(item.label.clone().into_boxed_str()),
                style: if i == self.selected {
                    self.selected_style.clone()
                } else {
                    Style::default()
                },
                ..RenderNode::default()
            });
            ctx.arena.append_child(root, child).unwrap();
            child
        }).collect();

        ListNode { root, children }
    }
}
```

### 3.5 Table Widget

Multi-column table with header.

```rust
pub struct TableWidget {
    pub columns: Vec<Column>,
    pub rows: Vec<Vec<String>>,
    pub selected_row: Option<usize>,
    pub style: Style,
    pub header_style: Style,
    pub selected_style: Style,
}

pub struct Column {
    pub title: String,
    pub width: Option<u16>,
    pub alignment: Alignment,
}
```

### 3.6 Scroll Widget

Scrollable container with scrollbar.

```rust
pub struct ScrollWidget {
    pub child: Box<dyn Widget>,
    pub show_scrollbar: bool,
    pub scrollbar_style: Style,
}
```

### 3.7 Modal Widget

Overlay dialog.

```rust
pub struct ModalWidget {
    pub child: Box<dyn Widget>,
    pub overlay: bool,
    pub close_on_escape: bool,
    pub style: Style,
}
```

### 3.8 Spacer Widget

Empty space.

```rust
pub struct SpacerWidget {
    pub size: Option<u16>,
}
```

### 3.9 Separator Widget

Horizontal or vertical line.

```rust
pub struct SeparatorWidget {
    pub direction: Direction,
    pub style: Style,
}
```

## 4. Widget Composition

### 4.1 Nesting

Widgets can contain other widgets:

```rust
BoxWidget {
    direction: FlexDirection::Column,
    children: vec![
        Box::new(TextWidget { content: "Header".into(), bold: true, .. }),
        Box::new(SeparatorWidget { direction: Direction::Horizontal, .. }),
        Box::new(ListWidget { items: items.clone(), .. }),
        Box::new(InputWidget { placeholder: "Type here...".into(), .. }),
    ],
}
```

### 4.2 Delegation

The `delegate()` pattern maps methods from an outer widget to specific descendants:

```rust
pub struct LabeledInput {
    label: String,
    input_id: NodeId,
}

impl LabeledInput {
    pub fn focus(&self, ctx: &WidgetContext) {
        ctx.focus_manager.focus(self.input_id);
    }

    pub fn value(&self, ctx: &WidgetContext) -> &str {
        // Read value from the input node
        ctx.arena.get(self.input_id)
            .and_then(|n| n.text.as_ref())
            .map(|s| s.as_ref())
            .unwrap_or("")
    }
}
```

### 4.3 Conditional Rendering

Widgets can conditionally render children:

```rust
pub fn conditional_widget(condition: bool, if_true: impl Widget, if_false: impl Widget) -> impl Widget {
    if condition { if_true } else { if_false }
}
```

### 4.4 List Rendering

Render a list of items:

```rust
pub fn list_widget<F>(items: &[Item], render_item: F) -> BoxWidget
where
    F: Fn(&Item) -> Box<dyn Widget>,
{
    BoxWidget {
        direction: FlexDirection::Column,
        children: items.iter().map(render_item).collect(),
        ..Default::default()
    }
}
```

## 5. Widget Lifecycle

### 5.1 Creation

```
1. Widget is instantiated (by framework adapter or parent widget)
2. Widget.create() is called
3. Nodes are inserted into the arena
4. Nodes are attached to the tree
5. Layout is triggered
```

### 5.2 Update

```
1. State changes (user input, data update)
2. Widget.update() is called
3. Nodes are modified in the arena
4. Dirty flags are set
5. Layout + render triggered
```

### 5.3 Destruction

```
1. Widget is removed from the tree
2. Widget.destroy() is called
3. Nodes are removed from the arena
4. Resources are cleaned up
```

## 6. State Management

### 6.1 Local State

Each widget can maintain local state:

```rust
pub struct InputState {
    value: String,
    cursor_position: usize,
    selection: Option<(usize, usize)>,
}
```

### 6.2 Shared State

Widgets can share state via context:

```rust
pub struct AppState {
    pub items: Vec<ListItem>,
    pub selected: usize,
    pub filter: String,
}
```

### 6.3 State Updates

State updates trigger re-rendering:

```
1. Widget modifies state
2. Widget marks itself as dirty
3. Re-render is scheduled
4. Widget.update() is called with new state
5. Nodes are updated
6. Layout + render triggered
```

## 7. Framework Adapter Integration

### 7.1 React Adapter

```tsx
// React component that wraps a widget
function ListBox({ items, selected, onSelect }) {
  return (
    <list
      items={items}
      selected={selected}
      onSelect={onSelect}
    />
  );
}
```

### 7.2 Vue Adapter

```vue
<template>
  <list
    :items="items"
    :selected="selected"
    @select="onSelect"
  />
</template>
```

### 7.3 Solid Adapter

```tsx
function ListBox(props) {
  return (
    <list
      items={props.items}
      selected={props.selected}
      onSelect={props.onSelect}
    />
  );
}
```

## 8. Performance

### 8.1 Widget Overhead

- Widget creation: ~1μs per widget.
- Widget update: ~0.5μs per widget.
- Widget destruction: ~0.1μs per widget.

### 8.2 Node Overhead

- Node creation: ~0.1μs per node.
- Node update: ~0.05μs per node.
- Node destruction: ~0.01μs per node.

### 8.3 Optimization Strategies

1. **Memoization:** Cache widget output and only re-create when props change.
2. **Lazy creation:** Create child widgets only when they become visible.
3. **Pool nodes:** Reuse node slots instead of creating new ones.
4. **Batch updates:** Group multiple state changes into a single update.

## 9. Future Considerations

### 9.1 Virtual Widgets

For lists with thousands of items, render only visible items:

```rust
pub struct VirtualListWidget {
    items: Vec<ListItem>,
    viewport_height: u16,
    scroll_offset: usize,
    item_height: u16,
}
```

### 9.2 Lazy Widgets

Widgets that load content asynchronously:

```rust
pub struct LazyWidget {
    loader: Box<dyn Fn() -> Box<dyn Widget>>,
    loaded: Option<Box<dyn Widget>>,
}
```

### 9.3 Widget Templates

Predefined widget configurations:

```rust
pub mod templates {
    pub fn alert(title: &str, message: &str) -> ModalWidget { ... }
    pub fn confirm(title: &str, message: &str) -> ModalWidget { ... }
    pub fn prompt(label: &str) -> InputWidget { ... }
    pub fn progress(value: f32) -> ProgressBarWidget { ... }
}
```
