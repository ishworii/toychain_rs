use crate::Block;
pub struct Blockchain {
    pub chains: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            chains: vec![Block::new(
                0,
                vec!["Genesis Block".to_string()],
                "000000".to_string(),
            )],
        }
    }

    pub fn get_latest_block(&self) -> &Block {
        &self.chains[self.chains.len() - 1]
    }

    pub fn add_block(&mut self, mut new_block: Block) {
        let latest = self.get_latest_block();
        new_block.set_prev_hash(latest.get_hash());
        new_block.hash = new_block.calculate_hash();
        self.chains.push(new_block);
    }

    pub fn is_valid(&self) -> (bool, String) {
        for i in 1..self.chains.len() {
            let current = &self.chains[i];
            let previous = &self.chains[i - 1];
            if current.hash != current.calculate_hash() {
                return (false, format!("Block {} has invalid hash", i));
            }
            if current.previous_hash != previous.hash {
                return (false, format!("Block {} previous hash mismatch", i));
            }
            if current.index != previous.index + 1 {
                return (false, format!("Block {} index error", i));
            }
        }
        (true, String::from("Looks good."))
    }
}
