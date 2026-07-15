# friday-memory

mem0-inspired memory as a lean Rust **library**. `add/search/get/delete`, scoped by
`(user_id, agent_id, run_id)`. One-LLM-call fact extraction (`infer=true`, falls back to
verbatim if no model), md5 hash dedup, append-only SQLite history, `sqlite-vec` vector store,
`fastembed-rs` local embeddings. No graph, no reranker, no plugin sprawl in v1.

Status: **P3.** See [../PLAN.md](../PLAN.md) §3c.
