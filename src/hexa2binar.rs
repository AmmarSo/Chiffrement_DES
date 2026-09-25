use std::collections::HashMap;

pub fn hexa2binars(block: &str, sens: &bool) -> String {
    let mut block_converted = String::new();

    // Table Hexa -> Binaire (votre code)
    let mut hexa2binary = HashMap::new();
    hexa2binary.insert('0', "0000");
    hexa2binary.insert('1', "0001");
    hexa2binary.insert('2', "0010");
    hexa2binary.insert('3', "0011");
    hexa2binary.insert('4', "0100");
    hexa2binary.insert('5', "0101");
    hexa2binary.insert('6', "0110");
    hexa2binary.insert('7', "0111");
    hexa2binary.insert('8', "1000");
    hexa2binary.insert('9', "1001");
    hexa2binary.insert('A', "1010");
    hexa2binary.insert('B', "1011");
    hexa2binary.insert('C', "1100");
    hexa2binary.insert('D', "1101");
    hexa2binary.insert('E', "1110");
    hexa2binary.insert('F', "1111");

    // Table Binaire -> Hexa (avec des chaînes de 4 caractères en clés)
    let mut binary2hexa = HashMap::new();
    binary2hexa.insert("0000", "0");
    binary2hexa.insert("0001", "1");
    binary2hexa.insert("0010", "2");
    binary2hexa.insert("0011", "3");
    binary2hexa.insert("0100", "4");
    binary2hexa.insert("0101", "5");
    binary2hexa.insert("0110", "6");
    binary2hexa.insert("0111", "7");
    binary2hexa.insert("1000", "8");
    binary2hexa.insert("1001", "9");
    binary2hexa.insert("1010", "A");
    binary2hexa.insert("1011", "B");
    binary2hexa.insert("1100", "C");
    binary2hexa.insert("1101", "D");
    binary2hexa.insert("1110", "E");
    binary2hexa.insert("1111", "F");

    if *sens {
        // Sens = true (Binaire vers Hexa) : on lit par paquets de 4
        let chars: Vec<char> = block.chars().collect();
        for chunk in chars.chunks(4) {
            let chunk_str: String = chunk.iter().collect();
            block_converted.push_str(binary2hexa.get(chunk_str.as_str()).copied().unwrap_or("0"));
        }
    } else {
        // Sens = false (Hexa vers Binaire) : votre boucle d'origine inchangée
        for c in block.chars() {
            block_converted.push_str(hexa2binary.get(&c).copied().unwrap_or("0"));
        }
    }
    
    block_converted
}