# solana-protocol-gym

> A full-stack Solana protocol implementation in Rust — built for engineers who want to understand the validator from the inside out.

This is not a dApp framework. It is not a wallet SDK. It is not another JSON-RPC wrapper.

**solana-protocol-gym** is a ground-up implementation of the Solana validator stack — gossip, TVU, TPU, PoH, Tower BFT, Sealevel, and RPC — written in Rust, modular by design, and built to be read, hacked, and learned from.

If you want to understand how Solana actually works at the wire level, this is where you start.

---

## Why this exists

Most Solana developers live at the smart contract layer. They use `@solana/web3.js`, call JSON-RPC endpoints, and never think about what happens underneath.

That's fine — until you want to work on the protocol itself.

The gap between "I can write Anchor programs" and "I understand how the validator processes a block" is enormous. Very few resources bridge it. Reading the Agave source cold is brutal. Running a full validator just to observe behavior requires datacenter hardware.

**solana-protocol-gym fills that gap.**

Every module implements a real piece of the Solana stack — same wire protocols, same data structures, same behaviors as Agave — but written to be understood, not just to run in production.

---

## What's inside

| Module | What it implements | Protocol |
|---|---|---|
| `dc-gossip` | Peer discovery, CRDS table, cluster state sync | UDP / QUIC |
| `dc-tvu` | Shred receiver, erasure reconstruction, block assembly | Turbine / UDP |
| `dc-tpu` | Transaction forwarding to leader | QUIC |
| `dc-ledger` | Block storage, account state, ledger parsing | Local |
| `dc-poh` | Proof of History hash chain verifier | — |
| `dc-consensus` | Tower BFT simulator — votes, forks, lockouts | — |
| `dc-runtime` | Sealevel-lite parallel transaction execution | — |
| `dc-rpc` | JSON-RPC surface over local replayed ledger | TCP |
| `dc-cli` | CLI tools for every module | — |

All modules speak the real Solana wire protocol. This is not a simulation of Solana — it connects to mainnet, receives real data, and processes it correctly.

---

## The protocol stack — bottom up

Most people learn Solana top-down: RPC → transactions → programs. This project goes the other direction.

```
Layer 7 — RPC              dc-rpc        serve getBlock, getTransaction locally
Layer 6 — Execution        dc-runtime    Sealevel-lite, parallel tx execution
Layer 5 — Consensus        dc-consensus  Tower BFT, fork choice, lockouts
Layer 4 — PoH              dc-poh        hash chain replay and verification
Layer 3 — Ledger           dc-ledger     blockstore, account state, shred DB
Layer 2 — TVU / TPU        dc-tvu        shred recv, erasure coding, block assembly
                           dc-tpu        QUIC transaction forwarding
Layer 1 — Gossip           dc-gossip     CRDS, peer discovery, cluster state
Layer 0 — Network          UDP/QUIC/TCP  actual wire protocols, sockets, packets
```

Each layer builds on the one below it. You can run any module independently, or wire them together into a functioning light node.

---

## Roadmap

### ✅ Phase 1 — JSON-RPC baseline
Query historical data. Understand the data model before touching the wire.

### ✅ Phase 2 — Gossip
- Connect to Solana mainnet gossip over UDP/QUIC
- Parse CRDS table entries — validator identities, contact info, slot announcements
- Discover peers without touching RPC
- CLI: `dc gossip peers`, `dc gossip slots`

This is where the project gets real. You open a UDP socket, speak Solana's gossip protocol, and watch mainnet validators announce themselves to you.

### 🔜 Phase 3 — TVU / Turbine (block receiver)
- Bind to TVU port and receive raw shreds from the network
- Parse data shreds and coding shreds
- Implement Reed-Solomon erasure reconstruction
- Reassemble shreds into full blocks — no RPC involved
- Compare native TVU speed vs `getBlock` RPC latency

**Why this matters:** This is how validators actually get blockchain data. Not via HTTP. Via Turbine — a shred propagation tree built on UDP. Understanding this is the difference between knowing Solana and knowing *how Solana works*.

### 🔜 Phase 4 — Proof of History verifier
- Implement PoH hash chain replay in Rust
- Verify time-ordering of transactions cryptographically
- Confirm that the global clock is honest

### 🔜 Phase 5 — TPU / QUIC (transaction sender)
- Build a QUIC client that connects directly to validator TPU ports
- Send raw transaction bytes — no RPC, no middleware
- Benchmark against `sendTransaction()` RPC

### 🔜 Phase 6 — Tower BFT simulator
- Simulate validator votes and fork choice
- Implement lockout rules
- Visualize how finality accumulates across forks

### 🔜 Phase 7 — Sealevel-lite runtime
- Execute transactions locally against replayed ledger state
- Understand parallel execution — why Solana can process thousands of TPS
- Replay historical blocks and verify outputs

### 🔜 Phase 8 — RPC surface
- Serve `getBlock`, `getTransaction`, `getAccountInfo` from your local ledger
- No Helius, no QuickNode — your own node, your own data
- Add custom analytics endpoints beyond standard RPC

### 🔜 Phase 9 — Finality proofs
- Generate cryptographic proofs that a transaction is finalized in the Solana ledger
- Export portable proof format verifiable outside Solana — on Ethereum, Cosmos, or any chain
- Foundation for trustless bridges and zk-light clients

**Why this matters:** Solana today doesn't expose raw finality proofs in a portable format. This module fills that gap — enabling trustless cross-chain verification without relying on a centralized bridge relayer.

### 🔮 Phase 10 — zk-light client
- Verify Solana state transitions using zero-knowledge proofs
- No need to replay the full ledger — verify correctness with a proof
- Cross-client testing against Agave and Firedancer

---

## Architecture

```
solana-protocol-gym/
├── crates/
│   ├── dc-gossip/        # CRDS, peer discovery, UDP/QUIC sockets
│   ├── dc-tvu/           # Turbine shred receiver, erasure coding
│   ├── dc-tpu/           # QUIC transaction forwarding
│   ├── dc-ledger/        # Blockstore, account state, ledger DB
│   ├── dc-poh/           # Proof of History verifier
│   ├── dc-consensus/     # Tower BFT simulator
│   ├── dc-runtime/       # Sealevel-lite execution engine
│   ├── dc-rpc/           # JSON-RPC server
│   └── dc-cli/           # CLI entry points
├── configs/
├── docs/
└── examples/
```

Each crate is independent. Pull in only what you need.

---

## Getting started

### Prerequisites
- Rust (latest stable)
- Cargo
- Git

### Clone
```bash
git clone https://github.com/victorchukwuemeka/solana-protocol-gym.git
cd solana-protocol-gym
```

### Build
```bash
cargo build
```

### Run gossip listener
```bash
cargo run --bin dc-gossip
```

You should start seeing mainnet validators announce themselves within seconds.

---

## Who this is for

- Engineers who want to contribute to Solana core (Agave, Firedancer, or ecosystem clients)
- Developers transitioning from dApp development into protocol engineering
- Anyone who has read the Solana whitepaper and wants to see it in running Rust code
- Researchers building zk-light clients, bridges, or cross-chain interoperability tools

---

## Relation to Agave

solana-protocol-gym is an independent implementation. It is not a fork of Agave.

Where Agave optimises for production performance, this project optimises for clarity. Where Agave abstracts away wire-level details, this project exposes them. The goal is not to replace Agave — it is to make Agave understandable.

Key source references used during implementation:
- `solana/ledger/src/shred.rs` — shred structure and erasure coding
- `solana/turbine/src/turbine.rs` — block propagation tree
- `solana/core/src/banking_stage` — transaction processing pipeline
- `solana/core/src/consensus` — Tower BFT implementation

---

## Contributing

Contributions welcome — especially from developers working through the protocol stack for the first time. If something is unclear, that is a bug worth fixing.

See [CONTRIBUTING.md](CONTRIBUTING.md).

---

## License

MIT — build whatever you want with it.

---

*Built by a contributor to solana-program/token, solana-program/token-2022, Pinocchio, and the Agave validator client.*
