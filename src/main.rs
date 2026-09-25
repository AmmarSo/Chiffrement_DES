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

pub fn chiffrement(input_hex: &str, k: &str) -> String {
    let sens = false;
    
    // Passage du message et de la clé en binaire (64 bits)
    let binaire = hexa2binars(input_hex, &sens);
    let binary_key = hexa2binars(k, &sens);
    
    // Permutations initiales (IP et PC-1)
    let perm_vec = initial_permutation(&binaire, &sens);
    let key_56 = key_permutation(&binary_key, &sens);
    
    // Séparation du bloc de données en deux moitiés
    let _l0 = &perm_vec[0..32];
    let _r0 = &perm_vec[32..64];

    // Séparation de la clé en C0 et D0 (28 bits chacun)
    let c = &key_56[0..28];
    let d = &key_56[28..56];

    // Exemple pour le tour 1 : décalage puis génération de la sous-clé K1 (PC-2)
    let (c1, d1) = circular_shift(c, d, 1, &sens);
    let k1 = second_key_permutation(&c1, &d1, &sens);
    
    println!("Taille de la sous-clé K1 générée : {} bits", k1.len());

    // Retourne le bloc après IP sous forme de chaîne (pour l'instant)
    perm_vec.iter().map(|b| (b + b'0') as char).collect()
}

pub fn dechiffrement(input_perm_bin: &str, _k: &str) -> String {
    let sens = true;
    
    // Permutation inverse IP^-1
    let bin_vec = initial_permutation(input_perm_bin, &sens);
    
    // Recolement des morceaux (réciproque de la séparation)
    let l_final = &bin_vec[0..32];
    let r_final = &bin_vec[32..64];
    let combined_vec: Vec<u8> = [l_final, r_final].concat();
    
    let bin_str: String = combined_vec.iter().map(|b| (b + b'0') as char).collect();
    
    // Retour au format hexadécimal d'origine
    hexa2binars(&bin_str, &sens)
}

use crate::feistel::{permutation_p, sboxes, xor48};

fn main() {
    let message_origine = "0123456789ABCDEF";
    let k = "133457799BBCDFF1";
    
    println!("L'hexadécimal {} en binaire est : {}", mon_nombre, resultat);
    println!("La table initial de permutation est : {:?}", initial_perm_table);


    let r0 = "F0AAF0AA";
    let k1 = "000110110000001011101111111111000111000001110010";


    let expansion = feistel::expansion_r(r0);
    let xor_result = xor48(r0, k1);
    let sbox_ = sboxes(&xor_result);
    let p = permutation_p(&sbox_);


    println!("Expansion R : {}", expansion);
    println!("XOR : {}", xor_result);
    println!("Sboxes : {}", sbox_);


}
