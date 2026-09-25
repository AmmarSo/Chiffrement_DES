mod hexa2binar;
mod initial_permutation;

use hexa2binar::hexa2binars;
use initial_permutation::initial_permutation;

// Fonction de chiffrement (pour l'instant : Hexa -> Binaire -> Permutation Initiale IP)
pub fn chiffrement(input_hex: &str) -> String {
    let sens_hexa_to_bin = false;
    
    // 1. On convertit l'hexadécimal en binaire (64 bits)
    let binaire = hexa2binars(input_hex, &sens_hexa_to_bin);
    
    // 2. On applique la permutation initiale (IP)
    let perm_vec = initial_permutation(&binaire, &sens_hexa_to_bin);
    
    // Conversion du Vec<u8> en String pour le résultat
    perm_vec.iter().map(|b| (b + b'0') as char).collect()
}

// Fonction de déchiffrement (pour l'instant : Permutation Inverse IP^-1 -> Binaire -> Hexa)
pub fn dechiffrement(input_perm_bin: &str) -> String {
    let sens_inverse = true;
    
    // 1. On applique la permutation inverse (IP^-1)
    let bin_vec = initial_permutation(input_perm_bin, &sens_inverse);
    let bin_str: String = bin_vec.iter().map(|b| (b + b'0') as char).collect();
    
    // 2. On reconvertit le binaire en hexadécimal d'origine
    hexa2binars(&bin_str, &sens_inverse)
}

fn main() {
    let message_origine = "14AD88F52A645789";
    
    println!("=== TEST DU CHIFFREMENT ===");
    println!("Message hexa d'origine : {}", message_origine);
    let resultat_chiffre = chiffrement(message_origine);
    println!("Résultat après chiffrement (IP) : {}", resultat_chiffre);

    println!("\n=== TEST DU DÉCHIFFREMENT ===");
    let resultat_dechiffre = dechiffrement(&resultat_chiffre);
    println!("Résultat après déchiffrement (IP^-1 + Hexa) : {}", resultat_dechiffre);
}