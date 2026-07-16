# @bettertui/react

The React 19 adapter for BetterTUI — a framework package. **For a React app, install only `@bettertui/react`**; it depends on `@bettertui/core` and pulls it in automatically, so you never install core by hand. (If you don't use React, install `@bettertui/core` directly for vanilla / native TypeScript support.)

It translates React's virtual-DOM operations into `@bettertui/core` `Command`s through a `react-reconciler` host config.

## What's inside

- `render()` and `RuntimeProvider` / `useRuntime` — mount a BetterTUI tree.
- Hooks: `useTheme`, `useFocus`, `useKeyboard`, `useMouse`, `useTerminal`, `useFrame`, `useClipboard`, `useAnimation`, `useTimeline`, `useSelection`, `useCapabilities`, `useResize`.
- Providers: `Provider`, `FocusProvider`, `TerminalProvider`, `SelectionProvider`, `CapabilitiesProvider`, `RuntimeProvider`.
- 13 component functions: `Box`, `Text`, `Code`, `Input`, `Textarea`, `Select`, `Slider`, `TabSelect`, `ScrollBar`, `ScrollBox`, `Markdown`, `Diff`, `TextTable`.
- `renderToStringAsync` (`src/testing.ts`) — drives the real reconciler against the Rust engine and returns ANSI output for tests.

## Example

```tsx
import { render, Box, Text } from "@bettertui/react";

render(
  <Box>
    <Text>hello</Text>
  </Box>,
);
```

## Testing

```bash
pnpm test                 # vitest run (jsdom + setup file)
pnpm test:coverage        # with @vitest/coverage-v8
```

React output is verified through `renderToStringAsync` — there is **no** `@bettertui/testing` package.

## Status

Reconciler and hooks are real; the 13 component functions are implemented and emit element descriptors consumed by the reconciler.

See [`docs/api/packages/react.md`](../../docs/api/packages/react.md) and [`docs/guides/testing.md`](../../docs/guides/testing.md).
