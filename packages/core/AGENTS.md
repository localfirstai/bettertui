# AGENTS.md

## Cross-Framework Boundary

- **`CommandBufferConsumer` (`{ push(command: Command): void }`) is the sole API contract between core and framework adapters.** Every framework adapter (React, Vue, Solid, Svelte) implements its own consumer. Keep this interface minimal — adding framework-specific concerns here breaks multi-framework architecture.
- Core must NEVER import from React, Vue, or any framework-specific package. Zero framework dependencies.
- **`@bettertui/core` packages must keep `react-reconciler` external** — but core itself doesn't use it at all.

## Reconciler vs Host Config

- **`packages/core/src/reconciler.ts` (createReconciler) is the simplified, framework-agnostic version.** It wraps pure tree operations (`createInstance`, `appendChild`, etc.) with command emission. No reconciler lifecycle hooks.
- **The React host config (`packages/react/src/renderer.ts`) does equivalent tree ops but also handles reconciler lifecycle** — `prepareForCommit`, `resetAfterCommit`, `commitMount`, etc. Don't try to deduplicate them; the React version must respond to reconciler internals.
- The pure tree operations (`createInstance`, `appendChild`, `removeChild`, etc.) in `command-buffer.ts` are reused by both. Only the wrapper layer differs.

## Runtime

- `packages/core/src/runtime.ts` was originally `packages/runtime/src/runtime.ts`. Git tracks this as a rename (93% similarity). The Runtime class is fully framework-agnostic — zero React imports.
- `Runtime.subscribe()` returns an unsubscribe function. Always capture it for cleanup.

## Git

- When moving files between packages (e.g., reconciler → core), git detects renames if content >50% similar. The diff shows only the delta (import path changes), not add+delete.
