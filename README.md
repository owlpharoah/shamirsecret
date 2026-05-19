# Shamir Threshold File Encryption

A compact implementation of Shamir's Secret Sharing for splitting a 32-byte symmetric key that encrypts a file. The code shows the full path from finite-field math to practical key handling and file encryption.

## What's Here

- Shamir secret sharing over a prime field with BigInt arithmetic and Lagrange interpolation
- AEAD file encryption using a randomly generated 32-byte key
- Parameterized n, t, and p with safe defaults for experimentation
- End-to-end flow: encrypt, split, reconstruct, decrypt

## Code Map

- [src/main.rs](src/main.rs) - orchestration and demo entry points
- [src/utils/key_generation.rs](src/utils/key_generation.rs) - RNG key generation
- [src/utils/key_ops.rs](src/utils/key_ops.rs) - Shamir split/reconstruct, modular arithmetic
- [src/utils/file_operations.rs](src/utils/file_operations.rs) - encryption/decryption and shard IO

## Mathematical Model

Let the secret be s in a prime field F_p. Choose random coefficients a_1, ..., a_(t-1) and define:

$$
f(x) = s + a_1 x + a_2 x^2 + \cdots + a_{t-1} x^{t-1} \pmod p
$$

A shard is $(x_i, y_i)$ where $y_i = f(x_i)$. Given any $t$ shards, reconstruct:

$$
s = \sum_{i=1}^{t} y_i \cdot \prod_{j \ne i} \frac{-x_j}{x_i - x_j} \pmod p
$$

The implementation uses extended Euclid for modular inversion and BigInt for safe arithmetic across the field.

## Usage

There is no CLI yet. The demo flow is invoked from [src/main.rs](src/main.rs).

1. Choose an input file path and update the `encrypt_file` call
2. Run:

```sh
cargo run
```

3. Use the generated shards plus the stored prime to reconstruct the key and decrypt

Example flow (edit main, then run):

```rust
encrypt_file("path/to/input.txt".to_string(), None, None, None)?;
decrypt_file("path/to/encrypted_output".to_string(), "path/to/shards_dir")?;
```

## Somethings to cover

- This is a research/demo implementation and is not audited for production use
- Output naming and shard directory layout are fixed in code
- (Being Worked On) Polynomial coefficients are sampled from 128-bit randomness and reduced mod p, which is not uniform when p is large
- Not verifiable secret sharing yet; corrupted shards are not detected
- Test cases

## Dependencies

- `num-bigint` for finite-field arithmetic
- `rand` for randomness
- `aes-gcm` for AEAD file encryption
