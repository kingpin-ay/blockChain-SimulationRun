use sha2::{Digest, Sha256};
use std::fmt::Write;
use std::io::Write as OtherWrite;
use std::{collections::HashMap, fs::File};
pub fn split_vec(vec: Vec<u8>, split_factor: usize) {
    let chunks = vec.chunks(split_factor);
    let mut new_string = String::new();
    let mut storage: HashMap<String, &[u8]> = HashMap::new();
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
        let new_value = format!("\n{}", hash_string);
        new_string.push_str(&new_value);
        storage.insert(hash_string, chunk);
    }
    write_data_to_ref_file(&new_string);
}

fn write_data_to_ref_file(value: &str) {
    let mut ref_file =
        File::create("ref/ref_example.txt").expect("Error creation the refrence file");
    let _ = match writeln!(ref_file, "{}", value) {
        Ok(contents) => contents,
        Err(_err) => (),
    };
}
