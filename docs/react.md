# React Adapter

`@bettertui/react` is the React 19 adapter. It translates React's virtual DOM operations into core `Command`s via a `react-reconciler` host config.

**For React apps, install only `@bettertui/react`** — it depends on `@bettertui/core` and pulls it in automatically.

## Pieces

- **Reconciler** (`reconciler/renderer.ts`): `createRoot()`, host config in mutation mode
- **Runtime context** (`context/runtimeContext.ts`): `RuntimeContext`, `useRuntimeContext`
- **Hooks** (`hooks/`): `useEffectEvent`, `useFocus`, `useKeyboard`, `useRuntime`, `useTerminalDimensions`, `useTheme`, `useTimeline`
- **JSX types** (`types/jsx.types.ts`): `BaseProps`, `BoxProps`, `BetterTUIElementType`, `InputProps`, `ScrollBoxProps`, `TextProps`

## Package info

- Dependencies: `@bettertui/core`, `@bettertui/shared`, `react-reconciler`
- Peer dependency: `react@^19`
- Only `@bettertui/react` imports React

## Status

Implemented: reconciler, hooks, runtime context, JSX types. The package has 10 test files covering public API, hooks, host config, runtime provider, and renderer.

## See also

- [Architecture overview](architecture/overview.md)
- [@bettertui/core API](api/packages/core.md)
- [Getting started guide](guides/getting-started.md)
