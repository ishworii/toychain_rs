use crate::{block::Block, blockchain::Blockchain};

mod block;
mod blockchain;

fn main() {
    let mut bc = Blockchain::new();
    bc.add_block(Block::new(
        1,
        vec!["Alice->Bob:100".to_string()],
        bc.get_latest_block().hash.clone(),
    ));
    bc.add_block(Block::new(
        2,
        vec!["Blob->Charlie:26".to_string()],
        bc.get_latest_block().hash.clone(),
    ));

    for b in bc.chains {
        println!("{:#?}", b);
    }
}
