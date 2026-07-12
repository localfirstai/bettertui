# @bettertui/testing

Testing utilities for BetterTUI applications.

## Exports

| Export | Kind | Description |
|--------|------|-------------|
| `MockCommandCollector` | class | Records commands for assertion |
| `MOCK_TERMINAL_SIZE` | const | `{ width: 80, height: 24 }` |
| `createPoint` | fn | `(x, y) => Point` |
| `createRect` | fn | `(x, y, w, h) => Rect` |
| `createMockHandler` | fn | Creates a mock function that records calls |
| `expectCommandBuffer` | fn | Asserts buffer length, emptiness, or command types |
| `createTestTree` | fn | Returns a simple `{ root: Instance }` tree |
| `sleep` | fn | `Promise<void>` delay helper |
| `flushMicrotasks` | fn | Flushes microtask queue |
| `renderToString` | fn | Simplified element-to-string (placeholder) |
| `expectToMatchSnapshot` | fn | Snapshot comparison helper |
