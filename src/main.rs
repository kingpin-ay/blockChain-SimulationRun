mod basic;
use std::io;

enum Cryptography {
    ENCRYPTION,
    DECRYPTION,
}

fn main() {
    const PRIVATE_KEY_PATH: &str = "key/private_key.pem";
    const PUBLIC_KEY_PATH: &str = "key/public_key.pem";

    loop {
        println!("Choose Functionality : ");
        println!("1. Generate Private key and Public key ");
        println!("2. Encrypt a File  ");
        println!("3. Decrypt a File  ");
        println!("4. Exit   ");
        let mut user_input = String::new();
        let _ = match io::stdin().read_line(&mut user_input) {
            Ok(content) => content,
            Err(error) => {
                println!("this is the error -> {}", error);
                return ();
            }
        };

        let user_choice_number: i32 = match user_input.trim().parse() {
            Ok(content) => content,
            Err(_err) => {
                println!("Parsing Failed , Please Enter a Valid Number.");
                return ();
            }
        };

        if user_choice_number == 1 {
            generate_key(PRIVATE_KEY_PATH, PUBLIC_KEY_PATH);
        } else if user_choice_number == 2 {
            encrypt_file(PUBLIC_KEY_PATH);
        } else if user_choice_number == 3 {
            decrypt_file(PRIVATE_KEY_PATH);
        } else if user_choice_number == 4 {
            break;
        } else {
            println!("Please Select A valid Option");
        }
    }
}

fn generate_key(private_key_path: &str, public_key_path: &str) {
    let _key_value = match basic::generate_rsa_key_pair(private_key_path, public_key_path) {
        Ok(_value) => {
            println!("Public key and private key Generation Complete");
        }
        Err(_error) => {
            println!("Public key and private key Generation Failed");
        }
    };
}

fn encrypt_file(public_key_path: &str) {
    let (file_path, encrypted_path) = take_file_input(Cryptography::ENCRYPTION);

    let _some_variable =
        basic::encrypt_file_with_public_key(&file_path, public_key_path, &encrypted_path);
}

fn decrypt_file(private_key_path: &str) {
    let (file_path, decrypted_path) = take_file_input(Cryptography::DECRYPTION);
}

fn take_file_input(action_type: Cryptography) -> (String, String) {
    let mut file_path = String::new();
    let mut encrypted_path = String::new();
    let mut decrypted_path = String::new();

    println!("Enter the file path: ");
    let _ = match io::stdin().read_line(&mut file_path) {
        Ok(_contents) => {}
        Err(error) => {
            println!("Was not able to take the file path input -> {}", error);
        }
    };
    match action_type {
        Cryptography::ENCRYPTION => {
            println!("Enter the encrypted file path: ");
            let _ = match io::stdin().read_line(&mut encrypted_path) {
                Ok(_contents) => {}
                Err(error) => {
                    println!(
                        "Was not able to take the encryption file path input -> {}",
                        error
                    );
                }
            };
            return (file_path, encrypted_path);
        }
        Cryptography::DECRYPTION => {
            println!("Enter the decrypted file path: ");
            let _ = match io::stdin().read_line(&mut decrypted_path) {
                Ok(_contents) => {}
                Err(error) => {
                    println!(
                        "Was not able to take the decryption file path input -> {}",
                        error
                    );
                }
            };
            return (file_path, decrypted_path);
        }
    }
}
