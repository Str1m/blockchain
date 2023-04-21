mod blocks;

use blocks::Block;
use blocks::merkle_tree::MerkleTree;

fn main() {
    let transaction_data:&[&[u8]] = &[
        b"Transaction 1",
        b"Transaction 2",
        b"Transaction 3",
    ];

    let genesis_block = Block::new_genesis(transaction_data.clone(), 2);

    println!("Genesis Block:\n{}", genesis_block.to_string());

    let next_block = Block::new(&transaction_data.iter().map(AsRef::as_ref).collect::<Vec<&[u8]>>(), &genesis_block, 2);

    println!("\nNext Block:\n{}", next_block.to_string());
    let data_list: &[&[u8]] = &[
        b"Hello World",
        b"MerkleNode",
        b"MerkleTree",
        b"Test data",
    ];

    let merkle_tree = MerkleTree::new(data_list);
    println!("{:?}", merkle_tree);
}
