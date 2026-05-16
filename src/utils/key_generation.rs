// Here we securely generate a 32 bit key that will be used for encrypting the file and then shared as shards

use rand::RngExt;

pub fn key_gen() -> [u8; 32] {
    let key: [u8; 32] = rand::rng().random();
    key
}
