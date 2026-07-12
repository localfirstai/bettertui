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

## Native Bridge (napi-rs)

- **tsdown must externalize `bettertui_bindings`.** The native napi-rs binary is loaded at runtime by Node.js and cannot be bundled. Add `deps: { neverBundle: ["bettertui_bindings"] }` in `tsdown.config.ts`.
- **Runtime name collision:** Native bridge exports a `Runtime` class (napi Rust engine wrapper). Core also has a `Runtime` class (framework-agnostic runtime). In `packages/core/src/index.ts`, rename the native import: `import { Runtime as NativeRuntime } from "./native/runtime.js"` to avoid collision.
- **Packages/core/src/index.ts must explicitly re-export native bridge symbols** (createNativeRuntime, NativeRuntime, etc.) from `src/native/index.ts`. Without explicit re-exports, the symbols are not part of the public API.

## Git

- When moving files between packages (e.g., reconciler → core, or native → core/src/native/), git detects renames if content >50% similar. The diff shows only the delta (import path changes), not add+delete.
- **Misleading rename detection during refactors:** Deleting a root `Cargo.toml` and creating a new one at a different path is detected as ~93% similar by git, even if the member paths differ. Always verify content manually — don't trust rename percentages alone during workspace migrations.
