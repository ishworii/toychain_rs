use crate::{block::Block, blockchain::Blockchain};

mod block;
mod blockchain;

fn main() {
    let mut bc = Blockchain::new();
    bc.add_block(Block::new(
        1,
        vec!["Alice->Bob:100".to_string()],
        String::new(),
    ));
    bc.add_block(Block::new(
        2,
        vec!["Blob->Charlie:26".to_string()],
        String::new(),
    ));
    println!("{:?}", bc.is_valid());
    //tamper transaction
    bc.chains[1].transactions = vec!["Bob->Charlie:200".to_string()];
    println!("{:?}", bc.is_valid());
}
