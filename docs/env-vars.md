# Environment Variables

Runtime configuration flags for BetterTUI.

## Variables

| Variable | Type | Default | Description |
| --- | --- | --- | --- |
| `BTUI_DEBUG` | `boolean` | `false` | Enable debug mode, event logging, and DevTools inspector in BetterTUI. |
| `BTUI_SHOW_STATS` | `boolean` | `false` | Show performance and FPS debug overlay at startup. |
| `BTUI_USE_CONSOLE` | `boolean` | `true` | Enable global console.* capture for the built-in terminal console overlay. |
| `SHOW_CONSOLE` | `boolean` | `false` | Open the built-in terminal console overlay at startup. |
| `BTUI_DUMP_CAPTURES` | `boolean` | `false` | Dump captured stdout and console logs on process exit. |
| `BTUI_NO_NATIVE_RENDER` | `boolean` | `false` | Skip native Rust frame renderer and run JS-only fallback loop. |
| `BTUI_FORCE_UNICODE` | `boolean` | `false` | Force Mode 2026 Unicode support in terminal capability detection. |
| `BTUI_FORCE_WCWIDTH` | `boolean` | `false` | Force standard wcwidth for character width calculations. |
| `BTUI_FORCE_EXPLICIT_WIDTH` | `string` | `""` | Force explicit character width detection mode (`true`/`1` or `false`/`0`). |
| `BTUI_LOG_LEVEL` | `string` | `"debug"` | Default log level for BetterTUI diagnostics (`debug`, `info`, `warn`, `error`, `trace`). |

---

_generated via packages/core/dev/print-env-vars.ts_
