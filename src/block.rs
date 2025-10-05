use serde::Serialize;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Debug)]
pub struct Block {
    pub index: i32,
    timestamp: u64,
    pub transactions: Vec<String>,
    pub previous_hash: String,
    #[serde(skip)]
    pub hash: String,
}

impl Block {
    pub fn new(index: i32, transactions: Vec<String>, previous_hash: String) -> Self {
        let tmp = Self {
            index,
            timestamp: Self::calculate_current_time().unwrap(),
            transactions,
            previous_hash,
            hash: String::new(),
        };
        let hash = tmp.calculate_hash();
        Self { hash, ..tmp }
    }

    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    pub fn set_prev_hash(&mut self, new_hash: &str) {
        self.previous_hash = new_hash.to_string();
    }

    fn calculate_current_time() -> Option<u64> {
        let current_time = SystemTime::now();
        let since_the_epoch = current_time.duration_since(UNIX_EPOCH).ok()?;
        let in_ms =
            since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;
        Some(in_ms)
    }

    pub fn calculate_hash(&self) -> String {
        let block_string = serde_json::to_string(&self).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(block_string.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}
