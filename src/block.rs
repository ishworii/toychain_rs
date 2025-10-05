use serde::Serialize;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Debug)]
pub struct Block {
    pub index: i32,
    timestamp: u64,
    pub transactions: Vec<String>,
    pub previous_hash: String,
    pub nonce: i32,
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
            nonce: 0,
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
        let hash_bytes = self.calculate_hash_bytes();
        hex::encode(hash_bytes)
    }

    fn calculate_hash_bytes(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();

        // Feed raw bytes directly - no string allocation
        hasher.update(&self.index.to_le_bytes());
        hasher.update(&self.timestamp.to_le_bytes());

        // Hash each transaction
        for tx in &self.transactions {
            hasher.update(tx.as_bytes());
        }

        hasher.update(self.previous_hash.as_bytes());
        hasher.update(&self.nonce.to_le_bytes());

        hasher.finalize().into()
    }

    pub fn mine(&mut self, difficulty: u32) {
        let mut attempts = 0;
        let start = Self::calculate_current_time().unwrap();

        // Pre-hash everything except the nonce
        let mut base_hasher = Sha256::new();
        base_hasher.update(&self.index.to_le_bytes());
        base_hasher.update(&self.timestamp.to_le_bytes());
        for tx in &self.transactions {
            base_hasher.update(tx.as_bytes());
        }
        base_hasher.update(self.previous_hash.as_bytes());

        // Calculate how many leading zero bits we need
        let required_zeros = difficulty as usize;

        loop {
            // Clone the pre-hashed state and add only the nonce
            let mut hasher = base_hasher.clone();
            hasher.update(&self.nonce.to_le_bytes());
            let hash_bytes: [u8; 32] = hasher.finalize().into();

            attempts += 1;

            // Check if we have enough leading zeros (in hex representation)
            // Each byte gives us 2 hex digits
            let mut leading_zeros = 0;
            for byte in hash_bytes.iter() {
                if *byte == 0 {
                    leading_zeros += 2;
                } else {
                    // Check high nibble
                    if byte >> 4 == 0 {
                        leading_zeros += 1;
                    }
                    break;
                }
            }

            if leading_zeros >= required_zeros {
                // Found it! Convert to string for storage
                self.hash = hex::encode(hash_bytes);
                break;
            }

            self.nonce += 1;
        }

        let elapsed = Self::calculate_current_time().unwrap() - start;
        let elapsed_sec = elapsed as f64 / 1000.0;
        let rate = if elapsed != 0 { attempts as f64 / elapsed_sec } else { 0.0 };

        println!("Block {} mined!", self.index);
        println!("  Hash: {}", self.hash);
        println!("  Nonce: {}", self.nonce);
        println!("  Attempts: {}", attempts);
        println!("  Time: {:.2} sec", elapsed_sec);
        println!("  Hash rate: {:.0} hashes/sec", rate);
    }
}
