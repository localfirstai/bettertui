# AGENTS.md

## Runtime

- `is_running(&self)` works because it checks `TerminalState`, not `PtyProcess::is_running(&mut self)`.
- `kill()` is idempotent — returns `Ok(())` if no process, not an error.
- `check_process()` must be called inside `read()`/`write()` to detect external process death before I/O.
- Uses `let` chain syntax (edition 2024): `if let Some(proc) = &self.process && proc.is_running() && ...`.

## State

- `ProcessStatus` is separate from `PtyProcess::is_running()` — tracked internally, updated by `check_process()`.
- Struct field `status: ProcessStatus` coexists with `fn status(&self)` method. `self.status` in impl blocks accesses the FIELD, not the method.

## Spawner

- `ProcessSpawner::spawn()` returns `Result<TerminalRuntime>` — never returns a raw `PtyProcess`.
- Path resolution (`which::which`) happens in the spawner, not in the config.
