# Widgets

Widgets are the high-level UI layer in the Rust engine. The widget host lives in `bettertui-engine` (exposed via the native bridge's `createWidgetHost`); there is no separate `bettertui-widgets` crate.

- `trait Widget` with `create`/`update`/`handle_event`/`destroy`
- `WidgetId(NodeId)`, `WidgetContext` (arena, focus manager, scheduler, terminal size, theme)
- `WidgetHost` mounts/unmounts widgets and dispatches events
- ~30 built-in widgets: `BoxWidget`, `FlexWidget`, `ContainerWidget`, `StackWidget`, `GridWidget`, `TextWidget`, `LabelWidget`, `ButtonWidget`, `BadgeWidget`, `HeadingWidget`, `InputWidget`, `TextareaWidget`, `ModalWidget`, `TabsWidget`, `TooltipWidget`, `SpinnerWidget`, `ProgressWidget`, `CodeWidget`, `ScrollAreaWidget`, `SeparatorWidget`, `SpacerWidget`, plus `chat/` and `markdown/` submodules

See [Architecture: Widget Model](architecture/widget-model.md).
