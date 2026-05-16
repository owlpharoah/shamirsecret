use num_bigint::{BigInt, BigUint};
use rand::{Rng, RngExt};

//3 parts to this:
// generate key
// encrypt the file
// SSS the key

mod utils;
use utils::file_operations::{decrypt, encrypt};
use utils::key_generation::key_gen;
fn main() {
    let k = key_gen();
    let k2 = key_gen();
    let _ = encrypt(String::from("test.txt"), &k);
    let _ = decrypt(String::from("secrets.enc"), &k);
}
fn sss(p: BigUint, secret: BigUint, n: u128, t: u128) {
    // Define constants
    // let p = BigUint::from(19u32);
    // let secret = BigUint::from(7u32);
    // let n = 5;
    // let t = 2;

    //Get random coeffs
    let coeff: Vec<BigUint>;
    let mut rng = rand::rng();
    coeff = (0..t - 1)
        .map(|_| BigUint::from(rng.random::<u128>()) % &p)
        .collect();

    //Collect (x,f(x))
    let mut mpp: Vec<(u128, BigUint)> = Vec::new();
    for i in 1..=n {
        let ib = BigUint::from(i);
        let mut f = secret.clone();
        for (j, aj) in coeff.iter().enumerate() {
            let power = ib.modpow(&BigUint::from((j + 1) as u32), &p);

            f = (f + (aj * power)) % &p;
        }
        mpp.push((i, f));
    }

    //output the mpp
    println!("Heres the mpp:\n {:?}", mpp);

    // apply lagrange polynomial theorem
    put(mpp[..t as usize].to_vec(), p);
}

fn put(a: Vec<(u128, BigUint)>, mut p: BigUint) {
    let prime = &mut p;
    let mut l: BigInt = BigInt::from(0);
    for i in 0..a.len() {
        let mut li: BigInt = BigInt::from(1);
        for j in 0..a.len() {
            if j != i {
                let inverse = BigInt::from(BigInt::from(a[i].0) - BigInt::from(a[j].0)).modpow(
                    &BigInt::from(prime.clone() - BigUint::from(2u32)),
                    &BigInt::from(prime.clone()),
                );
                li = (li * ((-1 * BigInt::from(a[j].0) * inverse) % BigInt::from(prime.clone())))
                    % BigInt::from(prime.clone());
            }
        }
        l = (l + BigInt::from(a[i].1.clone()) * BigInt::from(li)) % BigInt::from(prime.clone());
        l = (l + BigInt::from(prime.clone())) % BigInt::from(prime.clone());
    }
    println!("SECRET:{}", l);
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;

    #[test]
    fn reconstructs_secret() {
        let p = BigUint::from(19u32);
        let secret = BigUint::from(7u32);

        // n = total shares
        // t = threshold
        let n = 5;
        let t = 3;

        sss(p, secret, n, t);
    }

    #[test]
    fn larger_prime_test() {
        let p = BigUint::from(170141183460469231731687303715884105727u128);

        let secret = BigUint::from(69u32);

        let n = 10;
        let t = 5;

        sss(p, secret, n, t);
    }

    #[test]
    fn threshold_equals_two() {
        let p = BigUint::from(7919u32);

        let secret = BigUint::from(1234u32);

        let n = 6;
        let t = 2;

        sss(p, secret, n, t);
    }
}
