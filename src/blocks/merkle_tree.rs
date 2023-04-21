use sha3::{Digest, Sha3_256};

#[derive(Debug)]
enum MerkleNode {
    Internal { left: Box<MerkleNode>, right: Box<MerkleNode>, hash: [u8; 32] },
    Leaf { hash: [u8; 32] },
}

impl MerkleNode {
    fn hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha3_256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    pub fn new_leaf(data: &[u8]) -> MerkleNode {
        let hash = MerkleNode::hash(data);
        MerkleNode::Leaf { hash }
    }

    pub fn new_internal(left: MerkleNode, right: MerkleNode) -> MerkleNode {
        let mut combined_hash = left.get_hash().to_vec();
        combined_hash.extend(right.get_hash().iter());
        let hash = MerkleNode::hash(&combined_hash.to_vec());
        MerkleNode::Internal { left: Box::new(left), right: Box::new(right), hash }
    }

    fn get_hash(&self) -> &[u8; 32] {
        match self {
            MerkleNode::Leaf { hash } => hash,
            MerkleNode::Internal { hash, .. } => hash,
        }
    }
}

#[derive(Debug)]
pub struct MerkleTree {
    root: MerkleNode,
}

impl MerkleTree {
    pub fn new(data_list: &[&[u8]]) -> Self {
        let leaf_nodes = data_list.iter().map(|data| MerkleNode::new_leaf(data)).collect::<Vec<_>>();
        let root = MerkleTree::build_tree(leaf_nodes);
        MerkleTree { root }
    }
    pub fn get_root_hash(&self) -> &[u8;32]{
        self.root.get_hash()
    }
    fn build_tree(mut nodes: Vec<MerkleNode>) -> MerkleNode {
        while nodes.len() > 1 {
            let mut combined_nodes = Vec::new();
            while let (Some(left), Some(right)) = (nodes.pop(), nodes.pop()) {
                combined_nodes.push(MerkleNode::new_internal(left, right));
            }
            if let Some(remaining) = nodes.pop() {
                combined_nodes.push(remaining);
            }
            nodes = combined_nodes;
        }
        nodes.pop().unwrap()
    }
}