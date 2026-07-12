# @bettertui/shared

Pure type definitions for BetterTUI — zero runtime code, zero dependencies. Every other TypeScript package and the Rust engine agree on these shapes.

## What's inside

- Geometry: `Point`, `Size`, `Rect`
- Layout: `FlexDirection`, `JustifyContent`, `AlignItems`, `AlignContent`, `FlexWrap`, `Gap`
- Theme/color: `Theme`, border, and token types
- Command and event payload types shared with `@bettertui/core`

## Usage

```ts
import type { Point, Rect, Theme } from "@bettertui/shared";
```

Because this package is types-only, it adds nothing to your bundle.

## Status

Implemented. Type-only foundation re-exported by `@bettertui/core` and `@bettertui/react`.

See [`docs/api/packages/shared.md`](../../docs/api/packages/shared.md).
