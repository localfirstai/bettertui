# @bettertui/react

**React 19 adapter for BetterTUI.** Depends on `@bettertui/core` and `@bettertui/shared`. React apps install **only** `@bettertui/react` — core resolves automatically.

## Exports

### Reconciler

| Export | Type | Notes |
|--------|------|-------|
| `createRoot(element)` | function | Returns `Root` with `{ root, runtime, dispose() }` |
| `Root` | type | `{ root: ReconcilerRoot; runtime: CommandRuntime; dispose(): void }` |

### Runtime context

| Export | Type | Notes |
|--------|------|-------|
| `RuntimeContext` | `React.Context` | Default provides null renderer |
| `useRuntimeContext()` | hook | Returns `RuntimeContextValue` |

### Hooks

| Export | Notes |
|--------|-------|
| `useEffectEvent(effect, deps?)` | Wraps `useEffect` for runtime-aware lifecycle |
| `useFocus()` | Focus management |
| `useKeyboard(options?)` | `UseKeyboardOptions` |
| `useRuntime()` | Throws if called outside `createRoot()` |
| `useTerminalDimensions()` | Returns `TerminalDimensions` |
| `useTheme()` | Returns `ThemeMode` |
| `useTimeline()` | Timeline hook |

### JSX types

| Export | Notes |
|--------|-------|
| `BaseProps` | Common props for all elements |
| `BoxProps` | Props for Box element |
| `BetterTUIElementType` | Union of all element types |
| `InputProps` | Props for Input element |
| `ScrollBoxProps` | Props for ScrollBox element |
| `TextProps` | Props for Text element |

### Other

| Export | Notes |
|--------|-------|
| `useEffectEvent`, `useRuntime` | Re-exported from hooks |
| `TerminalDimensions` | `{ width: number; height: number }` |
| `ThemeMode` | Theme mode type |
| `UseKeyboardOptions` | Keyboard hook options |
