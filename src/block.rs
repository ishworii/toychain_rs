use serde::Serialize;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

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
        let display_len = (difficulty + 6) as usize;
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

        // Atomic flags for thread coordination
        let found = AtomicBool::new(false);
        let attempts = AtomicI32::new(0);

        // Define chunk size - smaller for better early exit, larger for less overhead
        const CHUNK_SIZE: i32 = 50_000;
        const CHECK_INTERVAL: i32 = 1000; // Check for early exit every N iterations

        // Parallel search using Rayon
        let result = (0..i32::MAX).into_par_iter()
            .step_by(CHUNK_SIZE as usize)
            .find_map_any(|chunk_start| {
                // Early exit if another thread found it
                if found.load(Ordering::Relaxed) {
                    return None;
                }

                let mut local_attempts = 0;

                // Process this chunk
                for nonce in chunk_start..(chunk_start + CHUNK_SIZE) {
                    // Check for early exit less frequently to reduce atomic overhead
                    if local_attempts % CHECK_INTERVAL == 0 && found.load(Ordering::Relaxed) {
                        attempts.fetch_add(local_attempts, Ordering::Relaxed);
                        return None;
                    }

                    // Clone the pre-hashed state and add only the nonce
                    let mut hasher = base_hasher.clone();
                    hasher.update(&nonce.to_le_bytes());
                    let hash_bytes: [u8; 32] = hasher.finalize().into();

                    local_attempts += 1;

                    // Fast leading zero check - early exit on first non-zero based on difficulty
                    let required_bytes = required_zeros / 2;
                    let mut valid = true;

                    // Check full zero bytes
                    for i in 0..required_bytes {
                        if hash_bytes[i] != 0 {
                            valid = false;
                            break;
                        }
                    }

                    // Check partial byte if needed
                    if valid && required_zeros % 2 == 1 {
                        if hash_bytes[required_bytes] >> 4 != 0 {
                            valid = false;
                        }
                    }

                    if valid {
                        // Found it! Signal other threads and store the result
                        found.store(true, Ordering::Relaxed);
                        attempts.fetch_add(local_attempts, Ordering::Relaxed);
                        return Some((nonce, hash_bytes));
                    }
                }

                // Update attempts count for this chunk
                attempts.fetch_add(local_attempts, Ordering::Relaxed);
                None
            });

        // If we found a solution, the result will contain the nonce and hash
        if let Some((nonce, hash_bytes)) = result {
            self.nonce = nonce;
            self.hash = hex::encode(hash_bytes);
        }

        let elapsed = Self::calculate_current_time().unwrap() - start;
        let elapsed_sec = elapsed as f64 / 1000.0;
        let total_attempts = attempts.load(Ordering::Relaxed);
        let rate = if elapsed != 0 { total_attempts as f64 / elapsed_sec } else { 0.0 };
        let rate_millions = rate / 1_000_000.0;

        let hash_display = if self.hash.len() > display_len {
            &self.hash[..display_len]
        } else {
            &self.hash
        };

        println!("Block {} mined!", self.index);
        println!("  Hash: {}...", hash_display);
        println!("  Nonce: {}", self.nonce);
        println!("  Attempts: {}", total_attempts);
        println!("  Time: {:.2} sec", elapsed_sec);
        println!("  Hash rate: {:.2}M hashes/sec", rate_millions);
    }
}
