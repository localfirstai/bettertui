# @bettertui/react

The React 19 adapter for BetterTUI. It is the first (but not the only) framework binding — it translates React's virtual-DOM operations into `@bettertui/core` `Command`s through a `react-reconciler` host config.

## What's inside

- `render()` and `RuntimeProvider` / `useRuntime` — mount a BetterTUI tree.
- Hooks: `useTheme`, `useFocus`, `useKeyboard`, `useMouse`, `useTerminal`, `useFrame`, `useClipboard`, `useAnimation`, `useTimeline`, `useSelection`, `useCapabilities`, `useResize`.
- Providers: `Provider`, `FocusProvider`, `TerminalProvider`, `SelectionProvider`, `CapabilitiesProvider`, `RuntimeProvider`.
- 69 component functions (layout, text, input, navigation, data display, overlays, status, specialized, terminal). These are currently thin wrappers that emit element descriptors and are not yet wired to a live native render loop.
- `renderToStringAsync` (`src/testing.ts`) — drives the real reconciler against the Rust engine and returns ANSI output for tests.

## Example

```tsx
import { render, Box, Text } from "@bettertui/react";

render(
  <Box>
    <Text>hello</Text>
  </Box>,
  container,
);
```

## Testing

```bash
pnpm test                 # vitest run (jsdom + setup file)
pnpm test:coverage        # with @vitest/coverage-v8
```

React output is verified through `renderToStringAsync` — there is **no** `@bettertui/testing` package.

## Status

Reconciler and hooks are real; the 69 component functions are thin wrappers not yet connected to a live native render loop.

See [`docs/api/packages/react.md`](../../docs/api/packages/react.md) and [`docs/guides/testing.md`](../../docs/guides/testing.md).
