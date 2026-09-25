pub fn second_key_permutation(c: &[u8], d: &[u8], _sens: &bool) -> Vec<u8> {
    // Table de permutation PC-2 standard du DES (48 éléments)[cite: 2]
    const PC2_TABLE: [usize; 48] = [
        14, 17, 11, 24, 1, 5,
        3,  28, 15, 6,  21, 10,
        23, 19, 12, 4,  26, 8,
        16, 7,  27, 20, 13, 2,
        41, 52, 31, 37, 47, 55,
        30, 40, 51, 45, 33, 48,
        44, 49, 39, 56, 34, 53,
        46, 42, 50, 36, 29, 32,
    ];

    // On réunit C_i et D_i pour former les 56 bits (C_i || D_i)[cite: 2]
    let mut cd = c.to_vec();
    cd.extend_from_slice(d);

    // On sélectionne et réorganise les 48 bits pour la sous-clé du tour[cite: 2]
    let mut pc2_result = vec![0; 48];
    for i in 0..48 {
        pc2_result[i] = cd[PC2_TABLE[i] - 1];
    }

    pc2_result
}