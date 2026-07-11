# AGENTS.md

## Architecture

- `NeovimConfig::to_process_config()` is the adapter boundary — converts editor-specific config into generic `ProcessConfig`.
- `NeovimProcess` delegates everything to `TerminalRuntime`. Editor-specific behavior lives in error message formatting only.
- `NeovimState` stores only editor-specific fields (`mode`, `filename`, `cursor`). All base process state comes from `TerminalState`.
- Error chain: `PtyError` → `TerminalError` → `NeovimError`. `From` impls auto-convert up the chain.

## Dependencies

- Imports `TerminalRuntime` from `crate::terminal_process::runtime`, not from pty directly.
- Imports `PtyConfig` only via the terminal_process module boundary, not from `crate::pty`.
