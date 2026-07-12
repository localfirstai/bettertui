# @bettertui/widgets

**Proposed TypeScript widget library. This package does not exist yet** — there is no `packages/widgets` directory. It is listed in the architecture as a planned future package.

## Planned scope

- A `Widget` interface and version constant on the TypeScript side.
- A bridge to the Rust widget framework (`bettertui-engine/src/widgets/`, ~200 tests): `BoxWidget`, `TextWidget`, `ButtonWidget`, `TableWidget`, `MarkdownRenderer`, `ChatView`, `PromptComposer`, and more.

## Status

Not implemented as a package. The real widget framework lives in the Rust engine; it is not exposed through TypeScript yet. Do not document concrete TS-side widgets. See [Architecture: Widget Model](../../architecture/WidgetModel.md).
