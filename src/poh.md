new(start_hash: String, ledger: Ledger)
  ↓
  Data created: PoH struct with start_hash, current_hash, ledger, empty verified_slots
  ↓
verify_slot_range(start_slot: u64, end_slot: u64)
  ↓
  Data flowing: fetches ParsedBlock from ledger for each slot
  ParsedBlock contains: slot, parent_slot, blockhash, transactions
  ↓
extract_poh_entry(block: &ParsedBlock)
  ↓
  Data created: PoHEntry struct
  Contains: slot, expected_hash (blockhash), transaction signatures (Vec<String>)
  ↓
replay_poh(entry: &PoHEntry)
  ↓
  Data flowing: current_hash (String), transaction signatures
  Process: SHA256(current_hash + each tx_signature)
  Data output: replayed_hash (String)
  ↓
verify_entry(entry: &PoHEntry, replayed_hash: &String)
  ↓
  Data compared: replayed_hash vs entry.expected_hash
  Data updated: current_hash = replayed_hash, verified_slots.push(slot)
  Data output: bool (match or not)
  ↓
print_report()
  ↓
  Data displayed: total slots, verified_slots count, failed_slots count


  