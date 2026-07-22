# PTY

BetterTUI can embed a real terminal process (shell, REPL, etc.) via a PTY. Code: `packages/core/crates/engine/src/pty.rs` (PTY types) and `terminal/process.rs` (process management).

## Components

| Type | Purpose |
|------|---------|
| `PtyConfig { program, args, env: Vec<(String,String)>, working_directory, size: PtySize }` | spawn configuration |
| `PtySize { cols, rows, pixel_width, pixel_height }` | dimensions |
| `PtyProcess` | `spawn(config)`, `read`, `write`, `resize`, `is_running(&mut self)`, `exit_status`, `kill`, `wait`, `pid` |
| `PtyRuntime` | facade over `PtyProcess` |
| `PtyReader` / `PtyWriter` | stream wrappers |
| `PtyError` | `SpawnFailed`, `ResizeFailed`, `ReadFailed`, `WriteFailed`, `KillFailed`, `NotRunning`, `ProcessExited(i32)` |

Built on `portable-pty`.

## Process management (`terminal/process.rs`)

`TerminalRuntime` manages the spawned process lifecycle: `spawn`, `read`, `write`, `resize`, `is_running`, `kill`, `wait`. Also defines `ProcessConfig`, `ProcessSpawner`, `SpawnResult`, `enum ProcessStatus`, `TerminalState`, `TerminalViewport { cols, rows, scroll }`, `enum ScrollMode`.
