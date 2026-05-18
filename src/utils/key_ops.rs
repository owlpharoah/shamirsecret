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
    for i in 1..=shares {
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

fn mod_reduce(value: BigInt, modulus: &BigInt) -> BigInt {
    ((value % modulus) + modulus) % modulus
}

fn extended_gcd(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    if b == BigInt::from(0) {
        (a, BigInt::from(1), BigInt::from(0))
    } else {
        let (g, x, y) = extended_gcd(b.clone(), a.clone() % b.clone());
        (g, y.clone(), x - (a / b) * y)
    }
}

fn mod_inverse(value: &BigInt, modulus: &BigInt) -> Option<BigInt> {
    let (g, x, _) = extended_gcd(value.clone(), modulus.clone());
    if g != BigInt::from(1) {
        None
    } else {
        Some(mod_reduce(x, modulus))
    }
}

pub fn join_shards(shards: Vec<(u128, BigUint)>, p: Option<BigUint>) -> BigInt {
    let p = p.unwrap_or(
        BigUint::parse_bytes(
            b"231584178474632390847141970017375815706539969331281128078915168015826259279871",
            10,
        )
        .unwrap(),
    );
    let p_big = BigInt::from_biguint(num_bigint::Sign::Plus, p);

    let mut l: BigInt = BigInt::from(0);
    for i in 0..shards.len() {
        let xi = BigInt::from(shards[i].0);
        let yi = BigInt::from(shards[i].1.clone());
        let mut li: BigInt = BigInt::from(1);
        for j in 0..shards.len() {
            if j != i {
                let xj = BigInt::from(shards[j].0);
                let diff = mod_reduce(xi.clone() - xj.clone(), &p_big);
                let inverse = mod_inverse(&diff, &p_big).expect("No modular inverse");
                let term = mod_reduce(-xj, &p_big);
                li = mod_reduce(li * term * inverse, &p_big);
            }
        }
        l = mod_reduce(l + yi * li, &p_big);
    }
    return l;
}
