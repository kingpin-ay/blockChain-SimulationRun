use rand::rngs::OsRng;
use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey};
use rsa::{RsaPrivateKey, RsaPublicKey};
use std::fs::File;
use std::io::Write;

fn main() {
    println!("Generating the private and public key -> ");
    let _key_value = match generate_rsa_key_pair("key/private_key.pem", "key/public_key.pem") {
        Ok(value) => value,
        Err(_error) => (),
    };
}

fn generate_rsa_key_pair(
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
    public_key_file.write_all(public_pem.as_bytes())?;

    Ok(())
}
