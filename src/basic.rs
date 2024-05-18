use rand::rngs::OsRng;
use rsa::pkcs1::{DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey};
use rsa::Pkcs1v15Encrypt;
use rsa::{RsaPrivateKey, RsaPublicKey};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

pub fn generate_rsa_key_pair(
    private_key_path: &str,
    public_key_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Generate an RSA private key
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits)?;
    let public_key = RsaPublicKey::from(&private_key);

    // Convert private key to PEM format
    let private_pem = private_key.to_pkcs1_pem(Default::default())?;
    let public_pem = public_key.to_pkcs1_pem(Default::default())?;

    // Write the private key to a file
    let mut private_key_file = File::create(private_key_path)?;
    private_key_file.write_all(private_pem.as_bytes())?;

    // Write the public key to a file
    let mut public_key_file = File::create(public_key_path)?;
    let _ = public_key_file.write_all(public_pem.as_bytes());

    Ok(())
}

pub fn testing(file_path: &str) {
    println!("hello world");
    let mut file = File::open(file_path).unwrap();
    let mut buffer = Vec::new();
    let _ = file.read_to_end(&mut buffer);
    // for number in &buffer {
    //     println!("file data {}", number);
    // }
}

pub fn encrypt_file_with_public_key(
    file_path: &str,
    public_key_path: &str,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
    // Open and read the file contents into a buffer
    let mut file = File::open(file_path).unwrap();
    let mut buffer = Vec::new();
    let _ = file.read_to_end(&mut buffer);

    // Read the public key from the PEM file
    let public_key_pem: String = match std::fs::read_to_string(public_key_path) {
        Ok(contents) => contents,
        Err(err) => {
            println!("error ->  {}", err);
            return Err(Box::new(err));
        }
    };
    let public_key = match RsaPublicKey::from_pkcs1_pem(&public_key_pem) {
        Ok(contents) => contents,
        Err(err) => {
            println!("error ->  {}", err);
            return Err(Box::new(err));
        }
    };
    // Encrypt the buffer using the public key
    let mut rng = OsRng;
    let encrypted_data = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &buffer)?;

    // Write the encrypted data to the output file
    let mut output_file = File::create(output_path)?;
    output_file.write_all(&encrypted_data)?;

    Ok(())
}
