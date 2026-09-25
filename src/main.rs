mod hexa2binar;
mod initial_permutation;

use hexa2binar::hexa2binars;
use initial_permutation::initial_permutation;

pub fn chiffrement(input_hex: &str) -> String {
    let sens_hexa_to_bin = false;
    
    let binaire = hexa2binars(input_hex, &sens_hexa_to_bin);
    let perm_vec = initial_permutation(&binaire, &sens_hexa_to_bin);
    
    // Séparation du bloc en deux moitiés (L0 et R0)
    let _l0 = &perm_vec[0..32];
    let _r0 = &perm_vec[32..64];

    // Pour l'instant on retourne le vecteur complet en String
    perm_vec.iter().map(|b| (b + b'0') as char).collect()
}

pub fn dechiffrement(input_perm_bin: &str) -> String {
    let sens_inverse = true;
    
    let bin_vec = initial_permutation(input_perm_bin, &sens_inverse);
    
    // On simule la récupération des deux moitiés à la fin des rondes
    let l_final = &bin_vec[0..32];
    let r_final = &bin_vec[32..64];
    
    // Réciproque de la séparation : on recolle les morceaux pour reformer les 64 bits
    let combined_vec: Vec<u8> = [l_final, r_final].concat();
    
    let bin_str: String = combined_vec.iter().map(|b| (b + b'0') as char).collect();
    
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