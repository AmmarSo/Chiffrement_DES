mod hexa2binar;
mod initial_permutation;

use hexa2binar::hexa2binars;
use initial_permutation::initial_permutation;

fn main() {
    let mon_nombre: String = "14AD88F52A645789".to_string(); 
    let resultat = hexa2binars(&mon_nombre);
    let initial_perm_table = initial_permutation(&resultat);
    
    println!("L'hexadécimal {} en binaire est : {}", mon_nombre, resultat);
    println!("La table initial de permutation est : {:?}", initial_perm_table);
}