use std::fs::{self, File};
use std::io::{BufWriter, Write};

use num_bigint::{BigInt, BigUint};
use rand::RngExt;

//3 parts to this:
// generate key
// encrypt the file
// SSS the key

mod utils;
use utils::file_operations::{decrypt, encrypt};
use utils::key_generation::key_gen;
use utils::key_ops::{join_shards, key_split, key_to_int};

fn encrypt_file(
    path: String,
    shares: Option<u128>,
    req_shares: Option<u128>,
    prime: Option<BigUint>,
) -> Result<(), Box<dyn std::error::Error>> {
    let key = key_gen();
    let _ = encrypt(&path, &key);

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

    for i in 0..=shards.len() {
        let file_name = format!("{}/shards/shard_{}", file_name, i + 1);

        let file = File::create(&file_name)?;
        let mut writer = BufWriter::new(file);
        writeln!(writer, "{}-{}", shards[i].0, shards[i].1)?;
    }

    return Ok(());
}

fn main() {
    encrypt_file(String::from("test.txt"), None, None, None);
}
