# @bettertui/devtools

## Purpose

Developer tooling for debugging BetterTUI applications. Intended to provide tree inspection, command logging, and performance metrics.

## Responsibilities

- Tree inspection and visualization
- Command protocol logging
- Performance metrics and frame timing

## Public API

```typescript
interface DevToolsOptions {
  enabled: boolean;
  port: number;
}

function createDevTools(options?: Partial<DevToolsOptions>): unknown;
```

**Status:** `createDevTools()` is a stub that returns `null`. No actual tooling is implemented.

## Dependencies

None.

## Consumers

- None currently.

## Internal Structure

```
src/
  index.ts   # DevToolsOptions interface, createDevTools() stub
```

## Design Principles

- **Standalone tooling.** DevTools should not affect the rendering pipeline.
- **Optional dependency.** Applications should be able to run without DevTools.

## Example Usage

```typescript
import { createDevTools } from "@bettertui/devtools";

// Currently returns null — no implementation exists
const devtools = createDevTools({ enabled: true, port: 3001 });
```

## Notes

- This package is a placeholder. `createDevTools()` ignores all options and returns `null`.
- A future implementation would depend on `@bettertui/core` to inspect the `CommandBuffer` and tree state.
- Recommended: either implement basic tree inspection or remove this package from the workspace until ready to develop.
