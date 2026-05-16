// Here we encrypt the file in a given location with a given key

use cocoon::{self, Cocoon};
use std::{
    fmt::format,
    fs::{self, File},
};

pub fn encrypt(path: String, key: &[u8; 32]) -> Result<(), Box<dyn std::error::Error>> {
    let file_contents = fs::read_to_string(path).expect("Couldnt find such a file");
    let mut cocoon = Cocoon::new(&key[..]);
    let mut new_file = File::create("secrets.enc")?;
    cocoon
        .dump(file_contents.into_bytes().to_vec(), &mut new_file)
        .map_err(|e| format!("Encryption error:{:?}", e))?;
    Ok(())
}

pub fn decrypt(path: String, key: &[u8; 32]) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(&path)?;
    let mut cocoon = Cocoon::new(&key[..]);
    let new_file_path = "open";
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
