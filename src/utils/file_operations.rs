// Here we encrypt the file in a given location with a given key

use cocoon::{self, Cocoon};
use num_bigint::BigUint;
use std::{
    fs::{self, File},
    io::read_to_string,
};

pub fn encrypt(path: &String, key: &[u8; 32]) -> Result<(), Box<dyn std::error::Error>> {
    let file_contents = fs::read_to_string(path).expect("Couldnt find such a file");
    let mut cocoon = Cocoon::new(&key[..]);
    let mut new_file = File::create("secrets.enc")?;
    cocoon
        .dump(file_contents.into_bytes().to_vec(), &mut new_file)
        .map_err(|e| format!("Encryption error:{:?}", e))?;
    Ok(())
}

pub fn decrypt(path: &String, key: &[u8; 32]) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(&path)?;
    let cocoon = Cocoon::new(&key[..]);
    let new_file_path = format!("{}-opened", path);
    let decrypted = match cocoon.parse(&mut file) {
        Ok(data) => data,
        Err(cocoon::Error::Cryptography) => {
            fs::write(new_file_path, "!!! Wrong Key !!!")?;
            return Err("Wrong Key".into());
        }
        Err(e) => {
            return Err(format!("Other decryption error: {:?}", e).into());
        }
    };

    fs::write(new_file_path, decrypted)?;
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
