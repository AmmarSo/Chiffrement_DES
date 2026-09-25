// src/utils.rs
use std::collections::HashMap;

pub fn hexa2binars(block: &str) -> String {
    let mut binary_block = String::new();

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

    for c in block.chars() {
        binary_block.push_str(hexa2binary.get(&c).copied().unwrap_or("0"));
    }
    
    binary_block
}