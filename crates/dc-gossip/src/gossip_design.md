# dc-gossip design

> Target network: Solana devnet  
> Emission: `tokio::sync::broadcast` channel  
> Transport: async tokio UDP  

---

## What this module does

`dc-gossip` connects to the Solana devnet gossip network, speaks the
real Solana gossip wire protocol, and emits structured events that other
crates (`dc-tvu`, `dc-cli`, `dc-ledger`) subscribe to.

It is not a full validator. It does not vote. It does not produce blocks.
It listens, merges state, and tells the rest of the system what it learned.

---

## Module structure

```
crates/dc-gossip/src/
├── main.rs        — entry point, wires all modules, starts the loop
├── lib.rs         — public API, re-exports GossipEvent and Emitter
├── types.rs       — all shared data types
├── transport.rs   — raw UDP socket, send/receive bytes
├── protocol.rs    — deserialize bytes into typed Protocol messages
├── handler.rs     — route Push/Pull/Ping to correct logic
├── crds.rs        — CRDS table: merge, dedup, extract peer info
└── emitter.rs     — broadcast channel, GossipEvent enum
```

One file, one job. No file knows more than it needs to.

---

## Data flow — step by step

```
devnet entrypoint
      │  UDP packet (raw bytes)
      ▼
transport.rs          — recv_from() → (Vec<u8>, SocketAddr)
      │
      ▼
protocol.rs           — bincode::deserialize → Protocol enum
      │
      ├─ Push  ──► handler.rs → merge CRDS entries
      ├─ Pull  ──► handler.rs → respond with our state
      └─ Ping  ──► handler.rs → reply with Pong
                        │
                        ▼
                    crds.rs   — dedup by version, update peer table
                        │
                        ▼
                   emitter.rs — tx.send(GossipEvent) → subscribers
```

This cycle runs every 1 second. The gossip round also pushes our own
state to random peers (fanout = 3 by default).

---

## Types — types.rs

```rust
/// A validator we discovered on the network
pub struct ValidatorInfo {
    pub id: Pubkey,               // validator identity key
    pub gossip_addr: SocketAddr,  // their gossip port
    pub tvu_addr: SocketAddr,     // where to receive shreds from them
    pub tpu_addr: SocketAddr,     // where to send transactions to them
    pub last_seen: u64,           // unix timestamp
    pub version: u64,             // CRDS version, used for dedup
}

/// A slot announcement from the network
pub struct SlotInfo {
    pub slot: u64,
    pub parent: u64,
    pub root: u64,
    pub validator_id: Pubkey,
}

/// Cluster health snapshot
pub struct ClusterHealth {
    pub active_validators: usize,
    pub latest_slot: u64,
}
```

---

## Protocol — protocol.rs

Solana gossip uses `bincode` serialization. Messages map to:

```rust
pub enum Protocol {
    PullRequest(CrdsFilter, CrdsValue),
    PullResponse(Pubkey, Vec<CrdsValue>),
    PushMessage(Pubkey, Vec<CrdsValue>),
    PruneMessage(Pubkey, PruneData),
    PingMessage(Ping),
    PongMessage(Pong),
}
```

We handle: `PushMessage`, `PullResponse`, `PingMessage`.
We send: `PullRequest` (to discover peers), `PongMessage` (reply to pings).

---

## CRDS table — crds.rs

CRDS = Conflict-free Replicated Data Structure.

Rules:
- Every entry has a `version` (unix timestamp from the sender)
- On merge: keep the entry with the **higher version**
- Entries older than 15 minutes are pruned
- On update: extract `ValidatorInfo` and emit a `GossipEvent`

```rust
pub struct CrdsTable {
    entries: HashMap<Pubkey, CrdsEntry>,
}

impl CrdsTable {
    pub fn merge(&mut self, incoming: Vec<CrdsValue>) -> Vec<GossipEvent>
    pub fn prune_stale(&mut self)
    pub fn get_peers(&self) -> Vec<ValidatorInfo>
}
```

---

## Emitter — emitter.rs

```rust
pub enum GossipEvent {
    NewValidator(ValidatorInfo),   // first time we see a validator
    ValidatorUpdated(ValidatorInfo), // their data changed
    SlotUpdate(SlotInfo),          // new slot announced
    PeerLeft(Pubkey),              // validator went stale
    ClusterHealth(ClusterHealth),  // periodic health snapshot
}

// channel capacity: 1000 events
pub type GossipTx = broadcast::Sender<GossipEvent>;
pub type GossipRx = broadcast::Receiver<GossipEvent>;
```

Other crates subscribe like this:

```rust
// in dc-tvu
let mut rx = gossip_tx.subscribe();
while let Ok(event) = rx.recv().await {
    match event {
        GossipEvent::NewValidator(v) => {
            // connect to v.tvu_addr and start receiving shreds
        }
        _ => {}
    }
}
```

---

## Transport — transport.rs

```rust
pub struct Transport {
    socket: UdpSocket,  // tokio async UdpSocket
}

impl Transport {
    pub async fn new(bind_addr: &str) -> Result<Self>
    pub async fn send(&self, msg: &[u8], to: &SocketAddr) -> Result<()>
    pub async fn recv(&self) -> Result<(Vec<u8>, SocketAddr)>
}
```

Uses `tokio::net::UdpSocket`. Non-blocking. WouldBlock is handled by
the async runtime — no manual loop needed.

---

## Devnet entrypoint

```
entrypoint.devnet.solana.com:8001
```

Bootstrap sequence:
1. Send a `PullRequest` to the entrypoint with an empty CRDS filter
2. Receive `PullResponse` containing a list of validators
3. Add those validators to our peer table
4. Begin the gossip loop — push to 3 random peers every second

---

## Gossip loop — main.rs

```
loop every 1s:
  1. receive_messages()     — drain the UDP socket
  2. process_messages()     — decode + handle each one
  3. gossip_round()         — pick 3 random peers, push our state
  4. prune_stale()          — remove peers not seen in 15 min
  5. emit ClusterHealth()   — broadcast current snapshot
```

---

## What dc-gossip does NOT do

- Does not vote
- Does not produce or validate blocks
- Does not implement the full CRDS filter (bloom filter optimization)
- Does not handle shreds (that is dc-tvu's job)

These are intentional limits. dc-gossip's job is to know who is on the
network and what slot they are on. Nothing more.

---

## Files to write next (in order)

1. `types.rs` — no dependencies, write first
2. `transport.rs` — depends on types
3. `protocol.rs` — depends on types
4. `crds.rs` — depends on types
5. `emitter.rs` — depends on types
6. `handler.rs` — depends on crds + emitter
7. `main.rs` — wires everything together
