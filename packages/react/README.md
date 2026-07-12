# @bettertui/react

## Purpose

React adapter for BetterTUI. Provides a custom `react-reconciler` host config, hooks, and component primitives that let you build terminal UIs with React.

## Responsibilities

- **Reconciler host config** (`renderer.ts`): Implements `react-reconciler`'s `HostConfig` interface, translating React tree operations into BetterTUI commands.
- **`render()` function:** Creates a `Runtime`, sets up the reconciler, and returns `{ root, runtime, dispose }`.
- **Hooks:** `Provider`/`useTheme`, `FocusProvider`/`useFocus`, `useKeyboard`, `TerminalProvider`/`useTerminal`, `useFrame`, `useClipboard`, `useAnimation`, `useTimeline`, `useMouse`, `SelectionProvider`/`useSelection`, `CapabilitiesProvider`/`useCapabilities`, `useRuntime`.
- **Components (69 exported):** Box, Text, Flex, Grid, Stack, Spacer, Separator, Heading, Label, Code, Blockquote, Button, Input, Textarea, Checkbox, Radio, Switch, Slider, Select, Combobox, Tabs, Accordion, Badge, Progress, Spinner, List, Tree, Table, DataTable, Tooltip, Modal, Popover, Dropdown, ContextMenu, Toast, StatusLine, StatusBar, Pane, Viewport, Calendar, Chart, ScrollArea, Markdown, CodeBlock, Diff, PromptComposer, ChatView, ThinkingIndicator, Terminal, TerminalViewport, TerminalProcess.
- **Providers:** `Provider` (theme), `FocusProvider`, `TerminalProvider`, `RuntimeProvider`, `SelectionProvider`, `CapabilitiesProvider`.

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

- Components are currently thin wrappers that emit element descriptors via `createElement`; they are not yet wired to a live native render loop. The reconciler (`renderer.ts`) and hooks are fully implemented.
- `useTheme`'s `Theme`/`ThemeColors`/`ThemeSpacing` is React-authored and distinct from `@bettertui/shared`'s `Theme`. Map between them at the render boundary.
