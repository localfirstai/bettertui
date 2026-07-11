# Widgets

Widgets are the high-level, composable UI layer. The real framework is in the Rust engine; the TypeScript package is currently a stub.

## Rust widget framework (`bettertui-engine/src/widgets/`)

- `trait Widget: Send + Sync` with `create`/`update`/`handle_event`/`destroy`, returning `WidgetId(NodeId)`.
- `WidgetContext` gives widgets the arena, focus manager, scheduler, terminal size, and theme.
- `WidgetHost` mounts/unmounts widgets and dispatches events.
- ~30 built-in widgets: `BoxWidget`, `FlexWidget`, `ContainerWidget`, `StackWidget`, `GridWidget`, `TextWidget`, `LabelWidget`, `ButtonWidget`, `BadgeWidget`, `HeadingWidget`, `InputWidget`, `TextareaWidget`, `ModalWidget`, `TabsWidget`, `TooltipWidget`, `SpinnerWidget`, `ProgressWidget`, `CodeWidget`, `ScrollAreaWidget`, `SeparatorWidget`, `SpacerWidget`, `PromptComposer`, plus `chat/` and `markdown/` submodules.

See [Architecture: Widget Model](architecture/WidgetModel.md).

## TypeScript `@bettertui/widgets`

Currently only:

```ts
export interface Widget { type: string; render(): unknown; }
export const WIDGET_VERSION = "0.0.0";
```

No concrete widgets are implemented on the TS side yet.

## Documenting a widget (when implemented)

Each widget doc should cover: purpose, props, example, lifecycle, rendering flow, interaction flow, keyboard support, mouse support.
