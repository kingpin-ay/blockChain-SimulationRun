mod basic;

fn main() {
    let private_key_path = "key/private_key.pem";
    let public_key_path = "key/public_key.pem";
    let file_path = "assets/example.txt";
    println!("Generating the private and public key -> ");
    let _key_value = match basic::generate_rsa_key_pair(private_key_path, public_key_path) {
        Ok(value) => value,
        Err(_error) => (),
    };
    basic::testing(file_path);
    let _some_variable =
        basic::encrypt_file_with_public_key(file_path, "key/public_key.pem", "assets/test.bin");
}
