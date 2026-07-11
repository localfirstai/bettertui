# @bettertui/icons

## Purpose

Icon registry for terminal UI icons. Provides an in-memory registry of named icons with character codes and search tags.

## Responsibilities

- **Icon registration:** `registerIcon()` adds icons to the global registry.
- **Icon lookup:** `getIcon()` retrieves an icon by name.
- **Icon listing:** `listIcons()` returns all registered icons.

## Public API

```typescript
interface Icon {
  name: string;    // Unique identifier (e.g., "check", "arrow-right")
  char: string;    // Single character to render (e.g., "✓", "→")
  tags: string[];  // Search tags (e.g., ["success", "done"])
}

function registerIcon(icon: Icon): void;
function getIcon(name: string): Icon | undefined;
function listIcons(): Icon[];
```

## Dependencies

None.

## Consumers

- None currently. This package has no consumers in the monorepo.

## Internal Structure

```
src/
  index.ts   # Icon interface, Map-based registry, CRUD functions
```

## Design Principles

- **Simple registry.** Icons are stored in an in-memory `Map`. No file system or build-time processing.
- **Framework-agnostic.** No React or framework-specific code.

## Example Usage

```typescript
import { registerIcon, getIcon, listIcons } from "@bettertui/icons";

registerIcon({ name: "check", char: "✓", tags: ["success", "done"] });
registerIcon({ name: "arrow-right", char: "→", tags: ["navigation", "forward"] });

const check = getIcon("check");
console.log(check?.char); // "✓"

const all = listIcons();
console.log(all.length); // 2
```

## Notes

- This package stores icon metadata but has no integration with the rendering pipeline. Icons must be manually converted to characters and passed to `@bettertui/react` components.
- The Rust engine's Nerd Font system (`native/engine/src/nerdfont/`) handles multi-codepoint glyph rendering. This package's `char: string` field assumes single-character icons, which may not align with Nerd Font ligatures.
- No icons are pre-registered — the registry starts empty.
