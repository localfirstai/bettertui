# bettertui-logger

## Purpose

File-based logger for native (Rust) BetterTUI code. Provides structured
logging to daily-rotating files via `tracing` / `tracing-subscriber`, so
native-side diagnostics don't pollute stdout (which the UI relies on).

## Responsibilities

- **Daily rotation:** Writes to `bettertui-YYYY-MM-DD.log`. On open, if the
  existing file was last modified on the current day it is appended;
  otherwise it is truncated and a new day's file is created.
- **Path validation:** Validates the configured log directory. Paths must be
  absolute; Windows additionally rejects `< > : " | ? *`, Unix rejects NUL.
- **Global subscriber install:** Installs a `tracing` fmt layer writing to
  the daily file (no ANSI, with target/file/line metadata) as the global
  default subscriber.

## Public API

| Item | Description |
|------|-------------|
| `init()` | Initializes the logger. Uses `BETTERTUI_LOG_DIR` if set and valid; otherwise falls back to `logs/` under the crate manifest dir. Failures are non-fatal (logger is disabled with an eprintln). |
| `init_with_dir(log_dir: &Path)` | Initializes the logger to a specific absolute directory. |

## Configuration

| Env var | Effect |
|---------|--------|
| `BETTERTUI_LOG_DIR` | Absolute directory for log files. If unset or invalid, the logger falls back to `<manifest>/logs`. |

## Dependencies

- `tracing` / `tracing-subscriber` — logging facade and fmt layer
- `tempfile` (dev) — used by the test suite

## Consumers

- `bettertui-examples` — calls `bettertui_logger::init()` for production-style logging
- Any native crate that wants file-based diagnostics

## Build & Test

```bash
cargo test -p bettertui-logger
```

## Notes

- The logger is guarded by a `OnceLock`, so repeated `init()` calls are no-ops.
- Initialization failures degrade gracefully: the logger is disabled rather than panicking.
- Test coverage includes leap-year logic, date formatting, path validation (platform-specific),
  and daily-file append/truncate behavior.
