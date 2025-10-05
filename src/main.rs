mod block;

use block::Block;

fn main() {
    let txs = vec!["Alice->Bob:100".to_string(), "Bob->Charlie:20".to_string()];
    let block = Block::new(1, txs, "000000".to_string());
    println!("{:?}", block);
    println!("Block hash : {}", block.get_hash());
}
