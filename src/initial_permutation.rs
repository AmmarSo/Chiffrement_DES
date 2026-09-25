pub fn initial_permutation(binary_block: &str) -> Vec<u8>{
    const IP_TABLE: [usize; 64] = [
    58, 50, 42, 34, 26, 18, 10, 2,
    60, 52, 44, 36, 28, 20, 12, 4,
    62, 54, 46, 38, 30, 22, 14, 6,
    64, 56, 48, 40, 32, 24, 16, 8,
    57, 49, 41, 33, 25, 17, 9,  1,
    59, 51, 43, 35, 27, 19, 11, 3,
    61, 53, 45, 37, 29, 21, 13, 5,
    63, 55, 47, 39, 31, 23, 15, 7,
    ];

    let mut ip_table = vec![0;64];
    for i in 0..64 {
        ip_table[i] = binary_block.as_bytes()[IP_TABLE[i] - 1] - b'0';
    }
    ip_table
}