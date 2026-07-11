# AGENTS.md

## Process

- `PtyProcess::is_running(&mut self)` — takes `&mut self` because it calls `child.try_wait()`. Cannot be called from `&self` context.
- `PtyProcess` does NOT use `portable-pty` — it uses `std::process::Command` with piped stdio. True PTY allocation is deferred.
- `PtyRuntime.wrap()` wraps an already-spawned `PtyProcess`. It does NOT spawn one.

## Config

- `PtyConfig::new()` vs `PtyConfig::default()` differ: `new()` takes command/args, `default()` gives an empty command that will fail on spawn.
- `PtySize` is at `config.size`, not a separate argument to `spawn()`.
