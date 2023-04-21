pub mod merkle_tree;

use std::ops::Deref;
use std::time::SystemTime;
use merkle_tree::MerkleTree;
use uuid::Uuid;
use sha3::{Digest, Sha3_256};
use sha3::digest::FixedOutput;

use rand::Rng;

pub struct Block {
    index: Uuid,
    timestamp: SystemTime,
    prev_hash: [u8; 32],
    transaction_data: Vec<Vec<u8>>,
    merkle_tree: MerkleTree,
    nonce: i128,
    hash: [u8; 32],
}

impl Block {
    pub fn new(transaction_data: &[&[u8]], prev_block: &Block, difficulty:u32) -> Self {
        let index = Uuid::new_v4();
        let timestamp = SystemTime::now();
        let merkle_tree = MerkleTree::new(transaction_data);
        let prev_hash = prev_block.get_hash();
        let (nonce, hash) = Self::proof_of_work(&index, &timestamp, &prev_hash, &transaction_data, &merkle_tree, difficulty);
        let transaction_data = transaction_data.iter().map(|&x|x.to_vec()).collect();
        Self { index, timestamp, prev_hash, transaction_data, merkle_tree, nonce, hash }
    }

    pub fn new_genesis(transaction_data: &[&[u8]], difficulty:u32) -> Self{
        let index = Uuid::new_v4();
        let timestamp = SystemTime::now();
        let merkle_tree = MerkleTree::new(&transaction_data.iter().map(AsRef::as_ref).collect::<Vec<&[u8]>>());
        let prev_hash = [0; 32];
        let (nonce, hash) = Self::proof_of_work(&index, &timestamp, &prev_hash, &transaction_data, &merkle_tree, difficulty);
        let transaction_data = transaction_data.iter().map(|&x|x.to_vec()).collect();
        Self { index, timestamp, prev_hash, transaction_data, merkle_tree, nonce, hash }
    }

    pub fn to_string(&self) -> String {
        format!("Index: {}\nTimestamp: {:?}\nPrevious Hash: {:x?}\nTransaction Data: {:?}\nMerkle Root: {:x?}\nNonce: {}\nHash: {:x?}",
                self.index, self.timestamp, self.prev_hash, self.transaction_data, self.merkle_tree.get_root_hash(), self.nonce, self.hash)
    }

    fn get_hash(&self) -> [u8; 32] {
        self.hash
    }

    fn proof_of_work(index: &Uuid, timestamp: &SystemTime,
                     prev_hash: &[u8; 32], transaction_data: &[&[u8]],
                     merkle_tree: &MerkleTree, difficulty: u32) -> (i128, [u8;32]){
        let mut nonce: i128 = 0;
        let mut hash: [u8; 32] = [1; 32];
        let mut rng = rand::thread_rng();
        while !Self::check_difficulty(&hash, difficulty) {
            let mut vec_for_hash = index.as_bytes().to_vec();
            let time_bytes = Self::timestamp_to_vec(timestamp);
            vec_for_hash.extend(&time_bytes);
            vec_for_hash.extend(prev_hash);
            for td in transaction_data{
                vec_for_hash.extend(*td);
            }
            vec_for_hash.extend(merkle_tree.get_root_hash());
            nonce = rng.gen();
            vec_for_hash.extend(nonce.to_be_bytes());
            let mut hasher = Sha3_256::new();
            hasher.update(vec_for_hash);
            hash = hasher.finalize().into();
        }
        (nonce, hash)
    }

    fn check_difficulty(hash: &[u8;32], difficulty:u32) -> bool{
        let prefix = vec![0u8;difficulty as usize];
        hash.starts_with(&prefix)
    }

    fn timestamp_to_vec(timestamp:&SystemTime)->Vec<u8>{
        let duration = timestamp.duration_since(SystemTime::UNIX_EPOCH).expect("Time error");
        let millis = duration.as_millis();
        let bytes = (millis as u128).to_be_bytes();
        bytes.to_vec()
    }
}
