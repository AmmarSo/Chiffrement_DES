pub fn circular_shift(c: &[u8], d: &[u8], round: usize, sens: &bool) -> (Vec<u8>, Vec<u8>) {
    // Table des décalages circulaires (1 à 16)[cite: 1]
    const SHIFT_TABLE: [usize; 16] = [
        1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1
    ];

    let effective_round = if !*sens { round } else { 17 - round };
    let shift_amount = SHIFT_TABLE[effective_round - 1];

    let mut c_shifted = c.to_vec();
    let mut d_shifted = d.to_vec();

    if !*sens {
        c_shifted.rotate_left(shift_amount);[cite: 1]
        d_shifted.rotate_left(shift_amount);[cite: 1]
    } else {
        c_shifted.rotate_right(shift_amount);
        d_shifted.rotate_right(shift_amount);
    }

    (c_shifted, d_shifted)
}