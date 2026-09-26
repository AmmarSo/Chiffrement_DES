mod hexa2binars;
mod initial_permutation;
mod feistel;
mod key_permutation;
mod circular_shift;
mod second_key_permutation;

use hexa2binars::hexa2binars;
use initial_permutation::initial_permutation;
use key_permutation::key_permutation;
use circular_shift::circular_shift;
use second_key_permutation::second_key_permutation;
use feistel::{des_rounds, permutation_finale};

pub fn des(input_hex: &str, k: &str, sens: bool) -> String {
    let conversion = false;

    let binaire = hexa2binars(input_hex, &conversion);
    let binary_key = hexa2binars(k, &conversion);

    let perm_vec = initial_permutation(&binaire, &conversion);

    let l0: String = perm_vec[0..32]
        .iter()
        .map(|b| (b + b'0') as char)
        .collect();

    let r0: String = perm_vec[32..64]
        .iter()
        .map(|b| (b + b'0') as char)
        .collect();

    let key_56 = key_permutation(&binary_key, &conversion);

    let mut c = key_56[0..28].to_vec();
    let mut d = key_56[28..56].to_vec();

    let mut keys: Vec<String> = Vec::new();

    for round in 1..=16 {
        let (new_c, new_d) = circular_shift(&c, &d, round, &conversion);

        c = new_c;
        d = new_d;

        let key_vec = second_key_permutation(&c, &d, &conversion);

        let key: String = key_vec
            .iter()
            .map(|b| (b + b'0') as char)
            .collect();

        keys.push(key);
    }

    if sens {
        keys.reverse();
    }

    let (l16, r16) = des_rounds(&l0, &r0, &keys);

    let preoutput = format!("{}{}", r16, l16);
    let final_bin = permutation_finale(&preoutput);

    let inverse = true;
    hexa2binars(&final_bin, &inverse)
}

pub fn chiffrement(input_hex: &str, k: &str) -> String {
    des(input_hex, k, false)
}

pub fn dechiffrement(input_hex: &str, k: &str) -> String {
    des(input_hex, k, true)
}

fn main() {
    let message_origine = "0123456789ABCDEF";
    let k = "133457799BBCDFF1";

    let message_chiffre = chiffrement(message_origine, k);
    let message_dechiffre = dechiffrement(&message_chiffre, k);

    println!("Message original : {}", message_origine);
    println!("Message chiffré : {}", message_chiffre);
    println!("Message déchiffré : {}", message_dechiffre);
}