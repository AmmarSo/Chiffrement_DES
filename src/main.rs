mod hexa2binar;
mod initial_permutation;
mod key_permutation;
mod circular_shift;

use hexa2binar::hexa2binars;
use initial_permutation::initial_permutation;
use key_permutation::key_permutation;
use circular_shift::circular_shift;

pub fn chiffrement(input_hex: &str, k: &str) -> String {
    let sens = false;
    
    // Passage du message et de la clé en binaire
    let binaire = hexa2binars(input_hex, &sens);
    let binary_key = hexa2binars(k, &sens);
    
    // Application des permutations initiales (IP et PC-1)
    let perm_vec = initial_permutation(&binaire, &sens);
    let key_56 = key_permutation(&binary_key, &sens);
    
    // Séparation du bloc de données en deux moitiés
    let _l0 = &perm_vec[0..32];
    let _r0 = &perm_vec[32..64];

    // Séparation de la clé en C0 et D0 (28 bits chacun)
    let c = &key_56[0..28];
    let d = &key_56[28..56];

    // Exemple de test du décalage pour le premier tour
    let (_c1, _d1) = circular_shift(c, d, 1, &sens);

    // Retourne le bloc après IP sous forme de chaîne
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

fn main() {
    let message_origine = "0123456789ABCDEF";
    let k = "133457799BBCDFF1";
    
    println!("=== TEST DU CHIFFREMENT ===");
    println!("Message d'origine : {}", message_origine);
    let resultat_chiffre = chiffrement(message_origine, k);
    println!("Résultat après IP : {}", resultat_chiffre);

    println!("\n=== TEST DU DÉCHIFFREMENT ===");
    let resultat_dechiffre = dechiffrement(&resultat_chiffre, k);
    println!("Message déchiffré : {}", resultat_dechiffre);
}