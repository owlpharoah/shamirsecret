use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::str::FromStr;

use num_bigint::{BigInt, BigUint};
use rand::RngExt;

//3 parts to this:
// generate key
// encrypt the file
// SSS the key

mod utils;
use utils::file_operations::{collect_shards, decrypt, encrypt};
use utils::key_generation::key_gen;
use utils::key_ops::{join_shards, key_split, key_to_int};

use crate::utils::key_ops::int_to_key;

fn encrypt_file(
    path: String,
    shares: Option<u128>,
    req_shares: Option<u128>,
    prime: Option<BigUint>,
) -> Result<(), Box<dyn std::error::Error>> {
    let key = key_gen();
    let _ = encrypt(&path, &key);
    let p = prime.clone();

    // now well split the key
    let k = key_to_int(&key);
    let shards: Vec<(u128, BigUint)> = key_split(k, shares, req_shares, prime)
        .map_err(|e| format!("Error generating shards: {}", e))?;

    let file_name = match path.split_once('.') {
        Some((before, _)) => before,
        None => &path,
    };
    let output_dir = format!("{}/shards", file_name);
    fs::create_dir_all(output_dir)?;

    for i in 0..shards.len() {
        let file_name = format!("{}/shards/shard_{}", file_name, i + 1);

        let file = File::create(&file_name)?;
        let mut writer = BufWriter::new(file);
        writeln!(writer, "{}-{}", shards[i].0, shards[i].1)?;
    }
    let prime_file = File::create(&format!("{}/shards/prime", file_name))?;
    let mut writer = BufWriter::new(prime_file);
    writeln!(
        writer,
        "{}",
        p.unwrap_or(
            BigUint::parse_bytes(
                b"231584178474632390847141970017375815706539969331281128078915168015826259279871",
                10
            )
            .unwrap()
        )
    )?;

    return Ok(());
}

fn decrypt_file(path: String, shards_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    //first we get the key
    let shards = collect_shards(String::from(shards_dir))?;
    let p = fs::read_to_string(format!("{}/prime", shards_dir))?;
    let prime = BigUint::from_str(p.trim());

    let a = join_shards(shards, Some(prime?));

    let key: [u8; 32] = int_to_key(&a);
    decrypt(&path, &key)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let _ = encrypt_file(String::from("secret.txt"), Some(10), Some(5), None);
    let _ = decrypt_file(String::from("secret.txt.enc"), "secret/shards")?;
    Ok(())
}
