# @bettertui/react

> **Website:** [bettertui.dev](https://bettertui.dev) | **Docs:** [bettertui.dev/docs](https://bettertui.dev/docs)

**React 19 adapter for BetterTUI.** React apps install **only** `@bettertui/react` — it depends on `@bettertui/core` and resolves it automatically.

## Overview

- **Reconciler** — `createRoot(element)` returns root with runtime
- **Hooks** — `useRuntime`, `useFocus`, `useKeyboard`, `useTheme`, `useTimeline`, `useTerminalDimensions`, `useEffectEvent`
- **Runtime context** — `RuntimeContext`, `useRuntimeContext`
- **JSX types** — `BoxProps`, `TextProps`, `InputProps`, `ScrollBoxProps`, plus global JSX namespace augmentation
- **Dependencies** — `@bettertui/core`, `@bettertui/shared`, `react-reconciler`; peer `react@^19`

## Installation

```bash
npm install @bettertui/react
```

## Quick start

```tsx
import { createRoot } from "@bettertui/react";

function App() {
  return <Box>Hello, terminal!</Box>;
}

const { root, runtime } = createRoot(<App />);
```

## Features

- Full react-reconciler host config (mutation mode)
- Runtime provider with frame loop
- Focus, keyboard, theme, and terminal dimension hooks
- JSX type support with autocomplete
- 10 test files covering public API, hooks, renderer, and runtime

## Related Documentation

- [Website](https://bettertui.dev)
- [API reference](../../docs/api/packages/react.md)
- [Architecture overview](../../docs/architecture/overview.md)
- [Getting started](../../docs/guides/getting-started.md)
