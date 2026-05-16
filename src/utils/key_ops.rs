use num_bigint::{BigInt, BigUint};
use rand::RngExt;

pub fn key_to_int(k: &[u8; 32]) -> BigUint {
    BigUint::from_bytes_be(k)
}

pub fn int_to_key(a: &BigInt) -> [u8; 32] {
    let a = a.to_biguint().unwrap();
    let bytes = a.to_bytes_be();
    if bytes.len() > 32 {
        panic!("Recovered key is larger than 32 bytes");
    }

    let mut key = [0u8; 32];
    let start = 32 - bytes.len();
    key[start..].copy_from_slice(&bytes);
    key
}

pub fn key_split(
    secret: BigUint,
    shares: Option<u128>,
    req_shares: Option<u128>,
    p: Option<BigUint>,
) -> Result<Vec<(u128, BigUint)>, Box<dyn std::error::Error>> {
    let shares = shares.unwrap_or(100);
    let req_shares = req_shares.unwrap_or(51);
    let p = p.unwrap_or(
        BigUint::parse_bytes(
            b"231584178474632390847141970017375815706539969331281128078915168015826259279871",
            10,
        )
        .unwrap(),
    );

    let coeff: Vec<BigUint>;
    let mut rng = rand::rng();
    coeff = (0..(req_shares - 1))
        .map(|_| BigUint::from(rng.random::<u128>()) % &p)
        .collect();

    let mut shards: Vec<(u128, BigUint)> = Vec::new();
    for i in 1..shares {
        let mut poly = BigUint::from(0 as u8);
        let shard = BigUint::from(i);
        for (j, k) in coeff.iter().enumerate() {
            let power = shard.modpow(&BigUint::from(j + 1), &p);

            poly = (poly + (k * power) % &p) % &p;
        }
        poly = (poly + &secret) % &p;
        shards.push((i, poly));
    }
    return Ok(shards);
}

pub fn join_shards(shards: Vec<(u128, BigUint)>, p: Option<BigUint>) -> BigInt {
    let p = p.unwrap_or(
        BigUint::parse_bytes(
            b"231584178474632390847141970017375815706539969331281128078915168015826259279871",
            10,
        )
        .unwrap(),
    );

    let mut l: BigInt = BigInt::from(0);
    for i in 0..shards.len() {
        let mut li: BigInt = BigInt::from(1);
        for j in 0..shards.len() {
            if j != i {
                let inverse = BigInt::from(BigInt::from(shards[i].0) - BigInt::from(shards[j].0))
                    .modpow(
                        &BigInt::from(&p - BigUint::from(2u32)),
                        &BigInt::from_biguint(num_bigint::Sign::Plus, p.clone()),
                    );
                li = (li * ((-1 * BigInt::from(shards[j].0) * inverse) % BigInt::from(p.clone())))
                    % BigInt::from(p.clone());
            }
        }
        l = (l + BigInt::from(shards[i].1.clone()) * BigInt::from(li))
            % BigInt::from_biguint(num_bigint::Sign::Plus, p.clone());
        l = (l + BigInt::from(p.clone())) % BigInt::from_biguint(num_bigint::Sign::Plus, p.clone());
    }
    return l;
}
