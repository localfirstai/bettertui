# @bettertui/react

**Planned first-class React adapter for BetterTUI.** Not implemented yet.

This directory is a placeholder. The React package will:

- depend on `@bettertui/core` and resolve it automatically (React apps install **only** `@bettertui/react`);
- provide a `react-reconciler` host config, a `render()` entry point, and components/hooks;
- never bypass `@bettertui/core` — it builds on the framework-agnostic runtime, command protocol, and native bridge that core already exposes.

Until the adapter lands, build terminal UIs with the implemented [`@bettertui/core`](../core/README.md) package directly. See the [architecture overview](../../docs/architecture/overview.md) for the intended React layering.
