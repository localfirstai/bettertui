# @bettertui/widgets

## Purpose

Widget interface and version constant for the BetterTUI widget system. This package provides the TypeScript-side contract for widgets that will eventually bridge to the Rust engine's native widget system.

## Responsibilities

- **Widget interface:** Defines the base contract for all widgets (`type`, `render()`).
- **Version constant:** `WIDGET_VERSION` for tracking widget API compatibility.

## Public API

```typescript
interface Widget {
  type: string;
  render(): unknown;
}

const WIDGET_VERSION: string; // "0.0.0"
```

## Dependencies

- `@bettertui/core`
- `@bettertui/shared`

## Consumers

- None currently. This package is a placeholder for the TypeScript widget bridge.

## Internal Structure

```
src/
  index.ts   # Widget interface, WIDGET_VERSION constant
```

## Design Principles

- **Framework-agnostic.** The `Widget` interface is not tied to React or any specific adapter.
- **Bridge to Rust.** The Rust engine (`bettertui-engine`) has a comprehensive widget system (`native/engine/src/widgets/`) with 25+ widget types. This package is intended to provide the TypeScript-side interface that maps to those native widgets.

## Example Usage

```typescript
import type { Widget } from "@bettertui/widgets";

// This is a conceptual example — no concrete widgets are implemented yet.
const myWidget: Widget = {
  type: "button",
  render() {
    // Widget rendering logic
    return null;
  },
};
```

## Notes

- This package is a minimal placeholder. The `Widget` interface (`render()` returning `unknown`) does not yet define a meaningful contract.
- The Rust engine's widget system is significantly more comprehensive. A mapping between TypeScript `Widget` instances and Rust native widgets is not yet implemented.
- This package should be extended or removed before v1.0 depending on whether widgets are exposed to TypeScript users or handled entirely in Rust.
