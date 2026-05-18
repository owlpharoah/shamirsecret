// Here we encrypt the file in a given location with a given key

use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
use num_bigint::BigUint;
use rand::RngExt;
use std::{
    fs::{self, File},
    io::read_to_string,
};

pub fn encrypt(path: &String, key: &[u8; 32]) -> Result<(), Box<dyn std::error::Error>> {
    let file_contents = fs::read(path).expect("Couldnt find such a file");

    let mut nonce_bytes: Vec<u8> = (0..12).map(|_| rand::rng().random::<u8>()).collect();
    let nonce = Nonce::from_slice(&nonce_bytes);

    let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| format!("Invalid key: {:?}", e))?;
    let ciphertext = cipher
        .encrypt(nonce, file_contents.as_ref())
        .map_err(|e| format!("Encryption error: {:?}", e))?;

    let mut output = Vec::with_capacity(12 + ciphertext.len());
    output.extend_from_slice(nonce);
    output.extend_from_slice(&ciphertext);

    fs::write(format!("{}.enc", path), output)?;

    Ok(())
}

pub fn decrypt(path: &String, key: &[u8; 32]) -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read(&path)?;

    if data.len() < 12 {
        return Err("File too short to contain a valid nonce".into());
    }

    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| format!("Invalid key: {:?}", e))?;

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "Decryption failed -- wrong key or corrupted file")?;

    let new_file_path = format!("{}-opened", path);
    fs::write(new_file_path, plaintext)?;

    Ok(())
}

pub fn collect_shards(path: String) -> Result<Vec<(u128, BigUint)>, Box<dyn std::error::Error>> {
    let mut shards: Vec<(u128, BigUint)> = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.file_name().and_then(|s| s.to_str()) == Some("prime") {
            continue;
        }

        let contents = fs::read_to_string(&path)?;
        let (x, y) = contents
            .trim()
            .split_once('-')
            .ok_or("Invalid Shard Format")?;
        let a: u128 = x.parse()?;
        let b: BigUint = y.parse()?;

        shards.push((a, b));
    }
    Ok(shards)
}
