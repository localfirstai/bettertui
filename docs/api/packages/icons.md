# @bettertui/icons

**In-memory icon registry.** No internal dependencies. Implemented (but no icons pre-registered).

## Exports

| Export | Type | Notes |
|--------|------|-------|
| `Icon` | `interface` | `{ name, char, tags[] }` |
| `registerIcon(icon: Icon): void` | function | adds to module-level `Map` |
| `getIcon(name: string): Icon \| undefined` | function | lookup |
| `listIcons(): Icon[]` | function | all registered icons |

## Status

Functional registry. Per the project taste guidance, Phosphor icons are the preferred icon set, but no icon sets are bundled yet.
