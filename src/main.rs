use block::Block;
use blockchain::BlockChain;

mod block;
mod blockchain;

fn main() {
  let mut blockchain = BlockChain::new();
  
  blockchain.genesis();
  let block1 = Block::new(1, String::from("block 1"), &blockchain.blocks[0]);
  blockchain.add_block(block1);
  println!("{:#?}", blockchain);
  let block2 = Block::new(2, String::from("block 2"), &blockchain.blocks[1]);
  blockchain.add_block(block2);
  println!("{:#?}", blockchain);
}
