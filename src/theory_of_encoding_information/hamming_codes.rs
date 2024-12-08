/*
Result in matrix, where:
row[0][1 to col_len] - encoded bits
rows[1 to row_len][0] - control bits
 */
pub fn encode(bits: Vec<u8>) -> Vec<Vec<u8>> {
    let n = bits.len();

    let mut control_bits = (n as f64).log2().ceil() as i32 + 1;

    let mut idx_to_insert = 1;

    let mut matrix: Vec<Vec<u8>> = vec![vec![0u8; n]; (control_bits + 1) as usize];

    for i in 1..bits.len() {
        matrix[0] = bits.clone();
    }
    for i in 0..matrix.len() {
        matrix[i].insert(0, 0);
    }

    let mut control_bit_indices = Vec::new();

    while control_bits > 0 {
        control_bit_indices.push(idx_to_insert);
        for i in 0..matrix.len() {
            matrix[i].insert(idx_to_insert, 0);
        }
        idx_to_insert *= 2;
        control_bits -= 1;
    }
    for col in 1..matrix[0].len() {
        let mut cloned_bytes = format!("{:b}", col).as_bytes().to_vec();
        cloned_bytes.reverse();
        let f_len = cloned_bytes.len();
        for row in (1..matrix.len()).rev() {
            if row - 1 < f_len {
                matrix[row][col] = cloned_bytes[row - 1] - b'0';
            } else {
                matrix[row][col] = 0;
            }
        }
    }
    let col_len = matrix[0].len();
    let row_len = matrix.len();
    let mut init_vector = matrix[0].clone();
    let mut bit_index = 0;
    for row in 1..row_len {
        let mut sum = 0;
        let mut row_vector = &matrix[row];
        for col in 1..col_len {
            sum += init_vector[col] * row_vector[col];
        }
        if sum % 2 == 0 {
            init_vector[control_bit_indices[bit_index]] = 0;
            matrix[bit_index + 1][0] = 0;
        } else {
            init_vector[control_bit_indices[bit_index]] = 1;
            matrix[bit_index + 1][0] = 1;
        }
        bit_index += 1;
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamming_codes() {
        let encoded_with_hamming = encode(vec![1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1]);

    }
}