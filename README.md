# DES Implementation in Rust

This repository contains a **Rust implementation of the Data Encryption Standard (DES)** developed as part of a **university project in cryptography and security protocols**.

The main objective of this project is educational: implementing the different components of DES manually in order to better understand the internal mechanisms of a symmetric block cipher.

> **Note:** DES is considered cryptographically obsolete today and should not be used for modern secure applications. This implementation is intended for educational purposes only.

## Project Objectives

The project focuses on understanding the different stages involved in DES rather than relying on an existing cryptographic library.

The implementation includes:

- Hexadecimal ↔ binary conversion
- Initial Permutation (IP)
- DES key permutation (PC-1)
- Separation of the key into `C` and `D` blocks
- Circular left shifts according to the DES key schedule
- Round-key generation using PC-2
- Expansion of the 32-bit right block to 48 bits
- XOR operation with the round key
- DES S-Boxes
- P permutation
- Feistel function
- Feistel rounds
- Final permutation (`IP⁻¹`)

## DES Structure

DES is a symmetric block cipher operating on **64-bit blocks** using a **64-bit key**, of which 56 bits are effectively used by the algorithm.

After the initial permutation, the input block is divided into two 32-bit halves:

```text
L0 | R0
```

Each DES round follows the Feistel structure:

```text
L(i+1) = R(i)

R(i+1) = L(i) XOR F(R(i), K(i))
```

where `K(i)` is the 48-bit round key generated from the original DES key.

The Feistel function performs:

```text
32-bit R
   │
   ▼
Expansion E
32 → 48 bits
   │
   ▼
XOR with round key
   │
   ▼
8 S-Boxes
48 → 32 bits
   │
   ▼
Permutation P
   │
   ▼
32-bit output
```

## Project Structure

```text
Chiffrement_DES/
│
├── Cargo.toml
├── Cargo.lock
│
└── src/
    ├── main.rs
    ├── hexa2binars.rs
    ├── initial_permutation.rs
    ├── key_permutation.rs
    ├── circular_shift.rs
    ├── second_key_permutation.rs
    └── feistel.rs
```

### Modules

`hexa2binars.rs`  
Handles hexadecimal-to-binary and binary-to-hexadecimal conversions.

`initial_permutation.rs`  
Implements the DES Initial Permutation and its inverse.

`key_permutation.rs`  
Implements the PC-1 permutation used to transform
