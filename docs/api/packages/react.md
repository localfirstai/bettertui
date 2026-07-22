# @bettertui/react

**Placeholder — React adapter not yet implemented.**

This package directory (`packages/react`) is a placeholder. React apps should wait for `@bettertui/react` to be implemented.

## Planned API

When implemented, this adapter will:

- depend on `@bettertui/core` and resolve it automatically (React apps install **only** `@bettertui/react`);
- provide a `react-reconciler` host config, a `render()` entry point, and components/hooks;
- never bypass `@bettertui/core` — it builds on the framework-agnostic runtime, command protocol, and native bridge that core already exposes.

## Until then

Build terminal UIs with the implemented [`@bettertui/core`](../core.md) package directly. See the [architecture overview](../../architecture/overview.md) for the intended React layering.

## Related Documentation

- [Architecture Overview](../../architecture/overview.md)
- [@bettertui/core API](../core.md)
