mod hexa2binars;
mod initial_permutation;
mod feistel;

use hexa2binars::hexa2binars;
use initial_permutation::initial_permutation;

use crate::feistel::{permutation_p, sboxes, xor48};

fn main() {
    let mon_nombre: String = "14AD88F52A645789".to_string(); 
    let resultat = hexa2binars(&mon_nombre);
    let initial_perm_table = initial_permutation(&resultat);
    
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
