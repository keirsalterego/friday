# friday-agents

**Field-expert presets** plus fan-out orchestration recipes. Mostly config and prompts, not a
new engine.

Key insight: there is **no supervisor class**. A "field expert" is a named preset (role, system
prompt, which CLI, allowed tools), and an orchestrator agent fans work out to instances of them
via the bus MCP tools (`add_terminal`, `send_to_node`, `wait_for_nodes`, `connect_nodes`,
`create_plan`). This package is where those presets and recipes live.

Status: **P2** (multi-agent, field experts). Stretch scope for the hackathon.
