# @bettertui/react

## Purpose

React adapter for BetterTUI. Provides a custom `react-reconciler` host config, hooks, and component primitives that let you build terminal UIs with React.

## Responsibilities

- **Reconciler host config** (`renderer.ts`): Implements `react-reconciler`'s `HostConfig` interface, translating React tree operations into BetterTUI commands.
- **`render()` function:** Creates a `Runtime`, sets up the reconciler, and returns `{ root, runtime, dispose }`.
- **Hooks:** `useTheme`, `useFocus`, `useKeyboard`, `useTerminal`, `useFrame`, `useClipboard`, `useAnimation`, `useRuntime`.
- **Components:** Box, Text, Flex, Button, Input, Textarea, Tabs, Modal, Badge, Progress, Spinner, Tooltip, Separator, Heading, Label, Code, Grid, Stack.
- **Providers:** `Provider` (theme), `FocusProvider`, `TerminalProvider`, `RuntimeProvider`.

## Public API

### Core

```typescript
function render(element: ReactNode): { root: OpaqueRoot; runtime: Runtime; dispose: () => void };
function RuntimeProvider({ runtime, children }: { runtime: Runtime; children: ReactNode }): JSX.Element;
function useRuntime(): RuntimeContextValue | null;
```

### Hooks

```typescript
function useTheme(): { theme: Theme; setTheme: (t: Theme) => void };
function useFocus(): { focusedId: string | null; setFocusedId; focusNext; focusPrevious };
function useKeyboard(handler: (event: KeyEvent) => boolean): void;
function useTerminal(): { width: number; height: number; resize(w, h): void };
function useFrame(): { requestFrame: () => void; frameRequested: boolean };
function useClipboard(): { clipboard: string; copy: (text: string) => Promise<void>; paste: () => Promise<string> };
function useAnimation(callback: (progress: number) => void, duration: number, deps?: unknown[]): void;
```

### Components

```typescript
function Box(props: BoxProps): JSX.Element;        // Flexbox container
function Text(props: TextProps): JSX.Element;        // Styled text
function Flex(props: FlexProps): JSX.Element;        // Flex layout
function Button(props: ButtonProps): JSX.Element;     // Pressable button
function Input(props: InputProps): JSX.Element;        // Text input
// ... 13 more components (see src/index.ts)
```

### Re-exports from core

```typescript
export type { Command, CommandBuffer, Instance, Runtime } from "@bettertui/core";
```

## Dependencies

- `@bettertui/core` — tree operations, `CommandBuffer`, `Instance`, `TextInstance`, `Runtime`
- `@bettertui/shared` — `Style`, `LayoutConstraints`, `ColorValue`
- `react-reconciler` ^0.31.0

### Peer dependencies

- `react` ^19.0.0

## Consumers

- Example applications (counter, future apps)

## Internal Structure

```
src/
  index.ts          # Public exports: components, hooks, types
  renderer.ts       # createBetterTUIReconciler() host config (363 lines)
  runtime.tsx       # render(), RuntimeProvider, useRuntime
  hooks/
    index.tsx       # All hook implementations and context providers
```

## Design Principles

- **React-only.** This package imports `react` and `react-reconciler`. Future framework adapters (Vue, Solid, Svelte) get their own packages that depend on `@bettertui/core` directly.
- **`Container` is react-only.** The `Container` type (`{ id, children, buffer }`) is tied to `react-reconciler`'s `ContainerInfo`. Do not move it to core.
- **`CommandBufferConsumer` is the cross-framework interface.** Each container holds a `buffer: CommandBufferConsumer` from `@bettertui/core`. This is the only cross-framework type.

## Example Usage

```tsx
import { render, Box, Text, Flex, Provider } from "@bettertui/react";

function App() {
  return (
    <Provider>
      <Flex flexDirection="column" gap={1}>
        <Box padding={1}>
          <Text bold>Hello BetterTUI</Text>
        </Box>
      </Flex>
    </Provider>
  );
}

const { runtime, dispose } = render(<App />);
// Later: dispose();
```

## Notes

- Components are currently stub implementations (return `props.children` or `null`). They will be implemented once the native rendering pipeline is fully connected.
- `useKeyboard` hooks into `globalThis.addEventListener("keydown")` — for browser environments. Terminal keyboard handling is not yet wired through the TypeScript layer.
