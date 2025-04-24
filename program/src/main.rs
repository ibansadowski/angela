//! A program that builds a Merkle tree using SHA-256 hash function
//! and demonstrates the power of precompiles in SP1.

#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::{sol, SolType};
use sha2::{Digest, Sha256};

// Define a struct for the public values we'll output
sol! {
    struct MerkleTreeValues {
        uint32 leaf_count;
        uint32 verification_index;
        bytes32 root;
        bool verification_result;
        uint64 hash_operations;
    }
}

// Build a Merkle tree from leaves using SHA-256
fn build_merkle_tree(leaves: &[Vec<u8>]) -> (Vec<Vec<Vec<u8>>>, usize) {
    let mut total_hash_operations = 0;
    let mut levels = Vec::new();

    // The bottom level is just the leaves
    levels.push(leaves.to_vec());

    // Build the tree bottom-up
    let mut current_level = leaves.to_vec();
    while current_level.len() > 1 {
        let mut next_level = Vec::new();

        // Process pairs of nodes
        for i in (0..current_level.len()).step_by(2) {
            let left = &current_level[i];

            // If we have an odd number of elements, duplicate the last one
            let right = if i + 1 < current_level.len() {
                &current_level[i + 1]
            } else {
                left
            };

            // Hash the concatenation of left and right nodes
            let mut hasher = Sha256::new();
            hasher.update(left);
            hasher.update(right);
            let hash_result = hasher.finalize().to_vec();
            total_hash_operations += 1;

            next_level.push(hash_result);
        }

        levels.push(next_level.clone());
        current_level = next_level;
    }

    (levels, total_hash_operations)
}

// Generate a proof for a specific leaf
fn generate_proof(levels: &[Vec<Vec<u8>>], leaf_index: usize) -> Vec<(Vec<u8>, bool)> {
    let mut proof = Vec::new();
    let mut current_index = leaf_index;

    // For each level except the root
    for i in 0..levels.len() - 1 {
        let level = &levels[i];

        // Determine if the sibling is on the left or right
        let is_right = current_index % 2 == 0;
        let sibling_index = if is_right {
            current_index + 1
        } else {
            current_index - 1
        };

        // If the sibling exists, add it to the proof
        if sibling_index < level.len() {
            proof.push((level[sibling_index].clone(), is_right));
        }

        // Move to the parent index for the next level
        current_index /= 2;
    }

    proof
}

// Verify a proof against the root
fn verify_proof(leaf: &[u8], proof: &[(Vec<u8>, bool)], root: &[u8]) -> (bool, usize) {
    let mut hash_operations = 0;
    let mut current = leaf.to_vec();

    // Apply each step in the proof
    for (sibling, is_right) in proof {
        let mut hasher = Sha256::new();

        // Order matters - put the current node on the left or right depending on is_right
        if *is_right {
            hasher.update(&current);
            hasher.update(sibling);
        } else {
            hasher.update(sibling);
            hasher.update(&current);
        }

        current = hasher.finalize().to_vec();
        hash_operations += 1;
    }

    (current == root, hash_operations)
}

pub fn main() {
    // Read the number of leaves and verification index
    let leaf_count = sp1_zkvm::io::read::<u32>();
    let verification_index = sp1_zkvm::io::read::<u32>();

    println!("Building Merkle tree with {} leaves", leaf_count);

    // Generate random leaves for the Merkle tree
    // In a real scenario, these could be transaction hashes or other data
    let mut leaves = Vec::new();
    for i in 0..leaf_count {
        let mut hasher = Sha256::new();
        hasher.update(format!("Leaf data {}", i).as_bytes());
        leaves.push(hasher.finalize().to_vec());
    }

    // Build the Merkle tree
    let (levels, build_hash_operations) = build_merkle_tree(&leaves);
    println!(
        "Built Merkle tree with {} hash operations",
        build_hash_operations
    );

    let root = &levels[levels.len() - 1][0];
    println!("Merkle Root: {:?}", root);

    // Generate a proof for the specified leaf
    let verification_index_usize = verification_index as usize;
    let proof = generate_proof(&levels, verification_index_usize);

    // Verify the proof
    let (verification_result, verify_hash_operations) =
        verify_proof(&leaves[verification_index_usize], &proof, root);

    let total_hash_operations = build_hash_operations + verify_hash_operations;
    println!("Verification result: {}", verification_result);
    println!("Total hash operations: {}", total_hash_operations);

    // Convert the Vec<u8> root to a fixed bytes32 array
    let mut root_bytes = [0u8; 32];
    if root.len() == 32 {
        root_bytes.copy_from_slice(root);
    } else {
        println!("Error: Root hash is not 32 bytes");
    }

    // Encode the public values of the program
    let public_values = MerkleTreeValues {
        leaf_count,
        verification_index,
        root: alloy_sol_types::private::FixedBytes(root_bytes),
        verification_result,
        hash_operations: total_hash_operations as u64,
    };

    let bytes = MerkleTreeValues::abi_encode(&public_values);

    // Commit the public values
    sp1_zkvm::io::commit_slice(&bytes);
}
