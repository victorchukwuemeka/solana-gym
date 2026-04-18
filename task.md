# GOSSIP TASKS - Study & Implementation Checklist

## **Phase 1: Understanding (Study First)**

### **Week 1: Core Concepts**
- [ ] Read `solana/gossip/src/contact_info.rs`
  - Understand ContactInfo struct (what info peers share)
  - Note: IP, ports (gossip, TPU, TVU, RPC), pubkey
  
- [ ] Read `solana/gossip/src/crds_value.rs`
  - List all CrdsData variants (message types)
  - Understand: ContactInfo, Vote, LowestSlot, etc.
  
- [ ] Read `solana/gossip/src/crds.rs`
  - Understand CRDS (Conflict-free Replicated Data Store)
  - How gossip prevents conflicts

- [ ] Practice bincode serialization
  - Write simple struct
  - Serialize to bytes
  - Deserialize back
  - Compare with JSON

### **Week 2: Protocol Mechanics**
- [ ] Read `solana/gossip/src/cluster_info.rs`
  - How peers are discovered
  - How data is stored locally
  - Pull vs Push gossip
  
- [ ] Read `solana/gossip/src/gossip_service.rs`
  - Background service loop
  - How often gossip runs
  - Packet sending/receiving

- [ ] Study gossip message flow:
  - Pull Request → Pull Response
  - Push Message → Prune Message
  
- [ ] Capture real gossip with tcpdump:
  ```bash
  sudo tcpdump -i any -w gossip.pcap port 8001
  ```

---

## **Phase 2: Implementation (Build It)**

### **Task 1: Passive Listener**
- [ ] Bind UDP socket to port 8001
- [ ] Receive raw bytes from gossip
- [ ] Try to deserialize with bincode
- [ ] Log packet types you see
- [ ] Goal: Just observe, don't send anything

### **Task 2: Decode Messages**
- [ ] Import `solana-gossip` crate
- [ ] Decode CrdsValue from bytes
- [ ] Parse ContactInfo from messages
- [ ] Extract validator IPs and ports
- [ ] Build list of peers

### **Task 3: Use Gossip Library**
- [ ] Create ClusterInfo instance
- [ ] Set entrypoint (known validator)
- [ ] Call `all_peers()` to get peer list
- [ ] Extract TVU addresses from peers
- [ ] Compare with your RPC `fetch_peers()`

### **Task 4: Join Gossip Network**
- [ ] Generate keypair (your identity)
- [ ] Create your ContactInfo
- [ ] Start GossipService
- [ ] Send Pull Requests
- [ ] Receive Pull Responses
- [ ] Maintain peer list

### **Task 5: Integration**
- [ ] Replace RPC peer discovery with gossip
- [ ] Use gossip to find validator TVU ports
- [ ] Update your Ledger to use gossip peers
- [ ] Compare speed: gossip vs RPC

---

## **Phase 3: Advanced (Deep Dive)**

### **Task 6: Build Custom Gossip Client**
- [ ] Manually craft Pull Request packet
- [ ] Send to entrypoint validator
- [ ] Parse Pull Response
- [ ] Handle CRDS version conflicts
- [ ] Implement Prune messages

### **Task 7: Understand Turbine (Next Level)**
- [ ] Read `solana/turbine` source
- [ ] Understand shred structure
- [ ] How blocks split into shreds
- [ ] How shreds propagate via gossip

---

## **Quick Reference Checklist**

### **Gossip Gives You:**
- [ ] Validator IP addresses
- [ ] Gossip port (8001)
- [ ] TPU port (8003-8009) - for transactions
- [ ] TVU port (8000-8010) - for receiving blocks
- [ ] RPC port (8899) - for queries
- [ ] Validator public keys
- [ ] Vote information

### **Gossip Does NOT Give You:**
- [ ] ❌ Actual blocks/slots (that's TVU)
- [ ] ❌ Transactions (that's TPU)
- [ ] ❌ Account data (that's RPC)

---

## **Milestone Goals**

**Milestone 1:** Understand what gossip messages look like
**Milestone 2:** Decode real gossip packets
**Milestone 3:** Get peer list using `solana-gossip` crate
**Milestone 4:** Replace RPC peer discovery completely
**Milestone 5:** Build custom gossip client from scratch

---

## **Study Resources Checklist**

- [ ] Clone solana repo
- [ ] Read gossip source files
- [ ] Run local test validator
- [ ] Use `solana gossip` CLI command
- [ ] Capture gossip with tcpdump
- [ ] Read Solana gossip docs
- [ ] Study CRDS papers
- [ ] Watch Solana Breakpoint talks on gossip

---

**Start with Phase 1, Task 1 (contact_info.rs) and check them off as you go!**