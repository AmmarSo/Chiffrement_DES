pub fn key_permutation(binary_key: &str, sens: &bool) -> Vec<u8> {
    // Table PC-1 standard du DES (56 éléments)
    const PC1_TABLE: [usize; 56] = [
        57, 49, 41, 33, 25, 17, 9,
        1, 58, 50, 42, 34, 26, 18,
        10, 2, 59, 51, 43, 35, 27,
        19, 11, 3, 60, 52, 44, 36,
        63, 55, 47, 39, 31, 23, 15,
        7, 62, 54, 46, 38, 30, 22,
        14, 6, 61, 53, 45, 37, 29,
        21, 13, 5, 28, 20, 12, 4,
    ];

    if !*sens {
        // Sens normal : 64 bits -> 56 bits (on extrait selon la table PC-1)
        let mut pc1_result = vec![0; 56];
        for i in 0..56 {
            pc1_result[i] = binary_key.as_bytes()[PC1_TABLE[i] - 1] - b'0';
        }
        pc1_result
    } else {
        // Sens inverse : 56 bits -> 64 bits (on remet les bits à leur place d'origine)
        let mut pc1_result = vec![0; 64];
        for i in 0..56 {
            pc1_result[PC1_TABLE[i] - 1] = binary_key.as_bytes()[i] - b'0';
        }
        pc1_result
    }
}