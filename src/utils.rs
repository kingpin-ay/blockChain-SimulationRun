use sha2::{Digest, Sha256};
use std::fmt::Write;

pub fn split_vec(vec: Vec<u8>, split_factor: usize) {
    let chunks = vec.chunks(split_factor);
    for chunk in chunks {
        let mut hasher = Sha256::new();
        for &val in chunk {
            hasher.update(val.to_le_bytes()); // Convert the integer to bytes and update the hasher
        }
        let hash_result = hasher.finalize();
        // Convert the hash result to a hexadecimal string
        let mut hash_string = String::new();
        for byte in hash_result {
            write!(&mut hash_string, "{:02x}", byte).expect("Unable to write");
        }

        println!("Hash for chunk {:?}: {}", chunk, hash_string);
    }
}
