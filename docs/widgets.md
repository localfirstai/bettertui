# Widgets

Widgets are the high-level, composable UI layer in the Rust engine.

## Rust widget framework

The widget host lives in the `bettertui-engine` crate (exposed via the native bridge's `createWidgetHost`); there is no separate `bettertui-widgets` crate.

- `trait Widget: Send + Sync` with `create`/`update`/`handle_event`/`destroy`, returning `WidgetId(NodeId)`.
- `WidgetContext` gives widgets the arena, focus manager, scheduler, terminal size, and theme.
- `WidgetHost` mounts/unmounts widgets and dispatches events.
- ~30 built-in widgets: `BoxWidget`, `FlexWidget`, `ContainerWidget`, `StackWidget`, `GridWidget`, `TextWidget`, `LabelWidget`, `ButtonWidget`, `BadgeWidget`, `HeadingWidget`, `InputWidget`, `TextareaWidget`, `ModalWidget`, `TabsWidget`, `TooltipWidget`, `SpinnerWidget`, `ProgressWidget`, `CodeWidget`, `ScrollAreaWidget`, `SeparatorWidget`, `SpacerWidget`, `PromptComposer`, plus `chat/` and `markdown/` submodules.

See [Architecture: Widget Model](architecture/widget-model.md).

## TypeScript exposure

Widgets are exposed through `@bettertui/core`'s native bridge. All Rust widget functionality ships through core.
