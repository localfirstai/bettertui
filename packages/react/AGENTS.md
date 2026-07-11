# AGENTS.md

## Container Type

- **`Container` (`{ id, children, buffer }`) is react-only**, tied to `react-reconciler`'s `ContainerInfo` type parameter. Do not move it to core. Future framework adapters (Vue, Solid) will define their own container types.
- Each Container has a `buffer: CommandBufferConsumer` from `@bettertui/core`. This is the only cross-framework type used here.

## Host Config

- **`packages/react/src/renderer.ts` requires `// biome-ignore format: host config is complex`** at the top of the HostConfig object. Without this, Biome destructively reformats the multi-line config (400+ lines) into unreadable single-line entries.
- Import `DefaultEventPriority` from `react-reconciler/constants` for `getCurrentEventPriority()`. The react-reconciler package provides this — it's not a React export.
- The host config was originally `packages/reconciler/src/renderer.ts`. Git tracks it as a rename (91% similarity) with import path modifications.
- `// biome-ignore lint/suspicious/noExplicitAny` is required throughout the HostConfig because `react-reconciler`'s generic type parameters use `any` for opaque types (OpaqueHandle, OpaqueRoot, etc.).

## Bundling

- `react-reconciler` must be `external: ["react", "react-reconciler"]` in tsup config. It's a peer dep — bundling creates version conflicts.
- `@bettertui/react` re-exports types from `@bettertui/core` (Command, CommandBuffer, Instance, Runtime) for consumer convenience. These are value types consumers need when building custom reconcilers or runtime integrations.

## Hooks

- `RuntimeProvider` and `useRuntime` were originally in `packages/runtime/src/hooks.tsx`. They use `createContext`, `useContext`, `useRef`, `useCallback` — all React-specific. They're in the right package.
