mod merkle_tree;

use merkle_tree::MerkleTree;

fn main() {
    println!("Hello world");
    let data_list: &[&[u8]] = &[
        b"Hello World",
        b"MerkleNode",
        b"MerkleTree",
        b"Test data",
    ];

    let merkle_tree = MerkleTree::new(data_list);
    println!("{:?}", merkle_tree);
}
