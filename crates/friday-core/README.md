# friday-core

The Rust orchestration engine — a **library**, consumed by `friday-desktop` (Tauri commands)
and `friday-mcp` (adapters). No UI, no I/O framework lock-in.

Owns: the PTY / terminal manager (direct-CLI spawn, the `terminal:*` contract, `seq`/ring-buffer
replay), agent-on-PATH detection, the canvas store (`.friday/canvas.json`), git-worktree
isolation, and the harness trait (Model A direct-PTY now, ACP-over-stdio later).

Status: **P1.** See [../PLAN.md](../PLAN.md) §3a.
