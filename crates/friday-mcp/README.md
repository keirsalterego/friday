# friday-mcp

MCP servers — thin **adapters** over `friday-core` and `friday-memory`, hand-rolled over
`axum` (server side only, just the transports the agent CLIs actually speak).

- **bus** — agent-to-agent messaging (`message_peer`, `check_inbox`, `list_peers`,
  `get_peer_context`, task board). Loopback + per-canvas bearer token. Gated on the
  bus-connection spike (PLAN §3a).
- **memory** — `add_memories` / `search_memory` (OpenMemory-style), over `friday-memory`.

Status: **P1.5 / P2.** See [../PLAN.md](../PLAN.md) §3a, §3d.
