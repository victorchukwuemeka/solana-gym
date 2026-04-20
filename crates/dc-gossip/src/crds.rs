use crate::types::{ClusterHealth, SlotInfo, ValidatorInfo};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub struct CrdsTable {
    validators: HashMap<String, ValidatorInfo>,
    latest_slot: u64,
}

impl CrdsTable {
    pub fn new() -> Self {
        Self {
            validators: HashMap::new(),
            latest_slot: 0,
        }
    }

    pub fn merge(&mut self, incoming: ValidatorInfo) {
        match self.validators.get(&incoming.id) {
            Some(existing) => {
                if incoming.version > existing.version {
                    self.validators.insert(incoming.id.clone(), incoming);
                }
            }
            None => {
                self.validators.insert(incoming.id.clone(), incoming);
            }
        }
    }

    pub fn prune(&mut self) {
        let now = current_time();
        let fifteen_minutes = 15 * 60;
        self.validators
            .retain(|_, v| now - v.last_seen < fifteen_minutes);
    }

    pub fn get_peers(&self) -> Vec<ValidatorInfo> {
        self.validators.values().cloned().collect()
    }
}
