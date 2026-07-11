# @bettertui/shared

## Purpose

Pure type definitions for the BetterTUI framework. This package exists so that all TypeScript packages (`core`, `react`, `themes`, `widgets`) share a single source of truth for core types without introducing runtime dependencies.

## Responsibilities

- Geometric types: `Point`, `Size`, `Rect`, `Direction`, `Alignment`, `Overflow`
- Layout types: `LayoutConstraints`, `LayoutResult`
- Rendering types: `Style`, `Border`, `Frame`, `FrameCell`, `RenderNode`, `RenderCommand`
- Event types: `Event`, `KeyEvent`, `MouseEvent`, `ResizeEvent`, `EventType`, `MouseButton`
- Color types: `Color`, `ColorValue`
- Theme types: `Theme`, `BorderStyle`
- Node types: `NodeId`

## Public API

All exports are TypeScript types and interfaces — zero runtime code.

```typescript
export type NodeId = string;
export interface Point { x: number; y: number; }
export interface Size { width: number; height: number; }
export interface Rect { x: number; y: number; width: number; height: number; }
export interface Style { fg?: ColorValue; bg?: ColorValue; bold?: boolean; /* ... */ }
export interface Theme { name: string; colors: Record<string, ColorValue>; borders: BorderStyle; }
// ... and more
```

## Dependencies

None.

## Consumers

- `@bettertui/core` — imports types for `Style`, `LayoutConstraints`, `Event`, etc.
- `@bettertui/react` — imports `Style`, `LayoutConstraints`, `ColorValue`
- `@bettertui/themes` — imports `Theme`, `BorderStyle`, `ColorValue`
- `@bettertui/widgets` — imports types via core

## Internal Structure

```
src/
  index.ts   # All type exports in a single file
```

## Design Principles

- **Zero runtime cost.** This package compiles to nothing — only type declarations ship.
- **No dependencies.** Types are defined from scratch, not imported from external packages.
- **Single source of truth.** All packages must import shared types from here, not redefine them.

## Example Usage

```typescript
import type { Style, Point } from "@bettertui/shared";

const style: Style = { fg: "#ffffff", bold: true };
const position: Point = { x: 10, y: 5 };
```

## Notes

- Future types (accessibility, animation, etc.) should be added here if consumed by multiple packages.
- Package-private types (internal to a single package) should NOT live here.
