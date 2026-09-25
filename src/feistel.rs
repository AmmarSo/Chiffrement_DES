pub fn hexa2binars(block: &str) -> String {
    let mut binary_block = String::new();

    for c in block.chars() {
        let bin = match c {
            '0' => "0000", '1' => "0001", '2' => "0010", '3' => "0011",
            '4' => "0100", '5' => "0101", '6' => "0110", '7' => "0111",
            '8' => "1000", '9' => "1001", 'A' => "1010", 'B' => "1011",
            'C' => "1100", 'D' => "1101", 'E' => "1110", 'F' => "1111",
            _ => "0000",
        };
        binary_block.push_str(bin);
    }

    binary_block
}


pub fn expansion_r(r: &str) -> String {
    let r0 = hexa2binars(r);
    
    const expansion_table: [usize; 48] = [
        32, 1, 2, 3, 4, 5,
        4, 5, 6, 7, 8, 9,
        8, 9, 10, 11, 12, 13, 
        12, 13, 14, 15, 16, 17, 
        16, 17, 18, 19, 20, 21,
        20, 21, 22, 23, 24, 25,
        24, 25, 26, 27, 28, 29, 
        28, 29, 30, 31, 32, 1
    ];

    let mut expanded = String::new();
        for &pos in expansion_table.iter() {
            let bit = r0.as_bytes()[pos-1] as char;
            expanded.push(bit);
        }
    expanded
}

pub fn xor48(r0: &str, k1: &str) -> String {
    let mut result = String::new();

    let expanded_r0 = expansion_r(r0);

    for (bit_r, bit_k) in expanded_r0.chars().zip(k1.chars()) {
        let r_val = bit_r as u8 - b'0';  
        let k_val = bit_k as u8 - b'0';

        let xor_bit = r_val ^ k_val; 
        result.push(char::from(b'0' + xor_bit));
    }
    result
}

const SBOXES_t: [[[u8; 16]; 4]; 8] = [
        // S1
        [
            [14,4,13,1,2,15,11,8,3,10,6,12,5,9,0,7],
            [0,15,7,4,14,2,13,1,10,6,12,11,9,5,3,8],
            [4,1,14,8,13,6,2,11,15,12,9,7,3,10,5,0],
            [15,12,8,2,4,9,1,7,5,11,3,14,10,0,6,13],
        ],
        // S2
        [
            [15,1,8,14,6,11,3,4,9,7,2,13,12,0,5,10],
            [3,13,4,7,15,2,8,14,12,0,1,10,6,9,11,5],
            [0,14,7,11,10,4,13,1,5,8,12,6,9,3,2,15],
            [13,8,10,1,3,15,4,2,11,6,7,12,0,5,14,9],
        ],
        // S3
        [
            [10,0,9,14,6,3,15,5,1,13,12,7,11,4,2,8],
            [13,7,0,9,3,4,6,10,2,8,5,14,12,11,15,1],
            [13,6,4,9,8,15,3,0,11,1,2,12,5,10,14,7],
            [1,10,13,0,6,9,8,7,4,15,14,3,11,5,2,12],
        ],
        // S4
        [
            [7,13,14,3,0,6,9,10,1,2,8,5,11,12,4,15],
            [13,8,11,5,6,15,0,3,4,7,2,12,1,10,14,9],
            [10,6,9,0,12,11,7,13,15,1,3,14,5,2,8,4],
            [3,15,0,6,10,1,13,8,9,4,5,11,12,7,2,14],
        ],
        // S5
        [
            [2,12,4,1,7,10,11,6,8,5,3,15,13,0,14,9],
            [14,11,2,12,4,7,13,1,5,0,15,10,3,9,8,6],
            [4,2,1,11,10,13,7,8,15,9,12,5,6,3,0,14],
            [11,8,12,7,1,14,2,13,6,15,0,9,10,4,5,3],
        ],
        // S6
        [
            [12,1,10,15,9,2,6,8,0,13,3,4,14,7,5,11],
            [10,15,4,2,7,12,9,5,6,1,13,14,0,11,3,8],
            [9,14,15,5,2,8,12,3,7,0,4,10,1,13,11,6],
            [4,3,2,12,9,5,15,10,11,14,1,7,6,0,8,13],
        ],
        // S7
        [
            [4,11,2,14,15,0,8,13,3,12,9,7,5,10,6,1],
            [13,0,11,7,4,9,1,10,14,3,5,12,2,15,8,6],
            [1,4,11,13,12,3,7,14,10,15,6,8,0,5,9,2],
            [6,11,13,8,1,4,10,7,9,5,0,15,14,2,3,12],
        ],
        // S8
        [
            [13,2,8,4,6,15,11,1,10,9,3,14,5,0,12,7],
            [1,15,13,8,10,3,7,4,12,5,6,11,0,14,9,2],
            [7,11,4,1,9,12,14,2,0,6,10,13,15,3,5,8],
            [2,1,14,7,4,10,8,13,15,12,9,0,3,5,6,11],
        ],
    ];



pub fn sboxes(xor48: &str) -> String {
    let mut result = String::new();

    // 8 blocs de 6 bits
    for (i, chunk) in xor48.as_bytes().chunks(6).enumerate() {
        let b0 = (chunk[0] - b'0') as usize;
        let b5 = (chunk[5] - b'0') as usize;

        let row: usize = (b0 << 1) | b5; // bits 1 et 6
        let col= (((chunk[1] - b'0') << 3)
                | ((chunk[2] - b'0') << 2)
                | ((chunk[3] - b'0') << 1)
                |  (chunk[4] - b'0')) as usize;

        let s_val = SBOXES_t[i][row][col];

        // convertir s_val (0..15) en 4 bits
        result.push_str(&format!("{:04b}", s_val));
    }

    result
}

const P_TABLE: [usize; 32] = [
    16, 7, 20, 21,
    29, 12, 28, 17,
    1, 15, 23, 26,
    5, 18, 31, 10,
    2, 8, 24, 14,
    32, 27, 3, 9,
    19, 13, 30, 6,
    22, 11, 4, 25,
];


pub fn permutation_p(sbox_output: &str) -> String {
    let mut result = String::new();

    for &pos in P_TABLE.iter() {
        let bit = sbox_output.as_bytes()[pos - 1] as char;
        result.push(bit);
    }
    result
}



pub fn function_f(r0_hex: &str, k1: &str) -> String{
    let xor = xor48(r0_hex, k1);
    let sbox = sboxes(&xor);
    let p = permutation_p(&sbox);
    p
}


pub fn tour_feistel(l: &str, r:&str, k: &str) -> (String, String){
    let f = function_f(r, k);
      let mut new_r = String::new();

    for (bit_l, bit_f) in l.chars().zip(f.chars()) {
        let l_val = bit_l as u8 - b'0';
        let f_val = bit_f as u8 - b'0';
        let xor_bit = l_val ^ f_val;
        new_r.push(char::from(b'0' + xor_bit));
    }

    let new_l = r.to_string();

    (new_l, new_r)

}

pub fn des_rounds(l0: &str, r0: &str, keys: &[String]) -> (String, String) {
    let mut l = l0.to_string();
    let mut r = r0.to_string();

    for i in 0..16 {
        let (new_l, new_r) = tour_feistel(&l, &r, &keys[i]);
        l = new_l;
        r = new_r;
    }

    (l, r)
}


const IP_INVERSE_TABLE: [usize; 64] = [
    40, 8, 48, 16, 56, 24, 64, 32,
    39, 7, 47, 15, 55, 23, 63, 31,
    38, 6, 46, 14, 54, 22, 62, 30,
    37, 5, 45, 13, 53, 21, 61, 29,
    36, 4, 44, 12, 52, 20, 60, 28,
    35, 3, 43, 11, 51, 19, 59, 27,
    34, 2, 42, 10, 50, 18, 58, 26,
    33, 1, 41, 9, 49, 17, 57, 25,
];

pub fn permutation_finale(preoutput: &str) -> String {
    let mut result = String::new();

    for &pos in IP_INVERSE_TABLE.iter() {
        let bit = preoutput.as_bytes()[pos - 1] as char;
        result.push(bit);
    }

    result
}
