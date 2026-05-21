# RelayEdge

> Edge-native AI agent kernel in Rust — 2MB binary, 5-layer trait-driven runtime, WASM target

[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange)](https://www.rust-lang.org/)

[![Status](https://img.shields.io/badge/status-in%20development-yellow)]()

---

## What is RelayEdge?

RelayEdge is a lightweight AI agent kernel written in Rust. It sits at the edge of the [Relay](https://github.com/vijaykrishn55/relay) distributed AI OS — handling prompt routing, tool execution, memory, and multi-model fallback in a single fast binary.

Part of the **Relay Portfolio**:
```
Relay       (Built)  → Distributed AI OS: multi-agent, multi-model, persistent memory
RelayEdge   (This)   → Rust agent kernel: 2MB binary, 5-layer runtime, WASM target
```

---

## Key Numbers (Targets)

| Metric | Target |
|--------|--------|
| Binary size | < 5MB |
| Boot time | < 10ms |
| Cold start | < 100ms |
| Throughput | > 1K msg/sec |

---

## Architecture

RelayEdge is built on a 5-layer trait-driven runtime:

```
┌─────────────────────────────────────────┐
│              RELAYEDGE KERNEL           │
├─────────────────────────────────────────┤
│  L0: Security     │ deny-by-default,    │
│                   │ API key masking     │
├─────────────────────────────────────────┤
│  L1: Orchestrator │ ModelRouter,        │
│                   │ ChainExecutor,      │
│                   │ ContextManager      │
├─────────────────────────────────────────┤
│  L2: Memory       │ SQLite FTS5 +       │
│                   │ Vector Search       │
├─────────────────────────────────────────┤
│  L3: Event Bus    │ tokio::broadcast    │
│                   │ pub/sub             │
├─────────────────────────────────────────┤
│  L4: Tool+Channel │ ToolRegistry,       │
│                   │ CLI/HTTP/WASM       │
└─────────────────────────────────────────┘
```

---

## Project Structure

```
relayedge/
├── Cargo.toml
├── config/
│   └── default.toml
└── src/
    ├── main.rs
    ├── lib.rs
    ├── security/
    │   └── mod.rs
    ├── orchestrator/
    │   ├── mod.rs
    │   └── context.rs
    ├── memory/
    │   └── mod.rs
    ├── eventbus/
    │   └── mod.rs
    ├── tools/
    │   └── mod.rs
    ├── channel/
    │   └── mod.rs
    └── providers/
        └── mod.rs
```

## Tech Stack

| Crate | Version | Purpose |
|-------|---------|---------|
| tokio | 1.43 | Async runtime |
| axum | 0.8 | HTTP server |
| reqwest | 0.12 | HTTP client (LLM API calls) |
| serde | 1.0 | JSON serialization |
| rusqlite | 0.32 | SQLite memory layer |
| clap | 4.5 | CLI framework |
| tracing | 0.1 | Structured logging |
| thiserror | 2.0 | Error types |
| wasm-bindgen | 0.2 | WASM compilation target |

---

## Quick Start

```powershell
git clone https://github.com/vijaykrishn55/relayedge
cd relayedge
cargo run
```

> Requires Rust 1.85+. Install at https://rustup.rs



*Built by Vijaykrishna — part of the [Relay Portfolio](https://github.com/vijaykrishn55/relay)*
