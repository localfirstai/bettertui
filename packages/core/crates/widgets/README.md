# bettertui-widgets

## Purpose

Widget framework for BetterTUI: the `Widget` trait, built-in widgets,
widget context, reconciliation, theming, and a host that manages the
reactive component lifecycle on top of `bettertui-engine`'s node tree.

## Responsibilities

- **Widget trait:** `Widget` defines the create/update/handle_event/destroy
  lifecycle. `WidgetId` wraps an engine `NodeId`.
- **Widget host:** `WidgetHost` registers widget factories, mounts/unmounts
  widgets into a `WidgetTree`, routes events, and tracks widget instances.
- **Built-in widgets:** A library of composable widgets under `basic/`,
  `chat/`, and `text/`.
- **Reconciliation:** `Reconciler` applies `ReconcileOp`s to keep the
  engine tree in sync with the widget tree.
- **Theming:** `Theme`, `ThemeColors`, `ThemeSpacing`, `ThemeBorders` plus a
  `Pipeline` for themed rendering.
- **App state:** `AppState` for application-level state shared across widgets.

## Public API (modules)

| Module | Description |
|--------|-------------|
| `app` | `AppState` application state container |
| `basic` | Box, button, container, flex, grid, modal, progress, scroll area, separator, spacer, spinner, stack, tabs, tooltip widgets |
| `callback_types` | Event/callback type definitions |
| `chat` | `ChatView`, `Message`, `Role`, `ChatStatus`, `ChatState`, `StatusBar`, `ThinkingIndicator` |
| `context` | `WidgetContext` (arena, focus manager, scheduler, terminal size, theme) |
| `pipeline` | `Pipeline` themed render pipeline |
| `reconcile` | `Reconciler`, `ReconcileOp` |
| `registry` | `WidgetRegistry` of factory closures |
| `text` | Text widgets: label, text, heading, code, badge, input, textarea, prompt composer, markdown |
| `theme` | `Theme`, `ThemeColors`, `ThemeSpacing`, `ThemeBorders` |
| `tree` | `WidgetTree` |

Top-level re-exports include `AppState`, `WidgetHost`, `Widget` (trait),
`WidgetId`, `WidgetLifecycle`, `ChatView`/`Message`/`Role`/etc.,
`WidgetContext`, `Pipeline`, `Reconciler`/`ReconcileOp`, `WidgetRegistry`,
`Theme`/themed types, and `WidgetTree`.

## Dependencies

- `bettertui-engine` — node tree, layout, input, scheduler
- `slotmap` — node id arena types
- `unicode-width` / `unicode-segmentation` — text measurement

## Consumers

- `bettertui-bindings` — exposes `NapiWidgetHost` to Node.js
- `bettertui-examples` — demonstrates `WidgetHost` usage
- Higher-level adapters (e.g. `@bettertui/react`) build on this framework

## Build & Test

```bash
cargo test -p bettertui-widgets
```

## Notes

- 257 lib tests (verified via `cargo test -p bettertui-widgets --lib`).
- This crate is framework-agnostic Rust — no React/TypeScript dependency.
  React bindings live in the `@bettertui/react` package.
