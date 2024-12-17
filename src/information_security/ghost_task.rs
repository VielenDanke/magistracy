// ГОСТ 28147_89
struct GostCypher {
    s_box: [[u8; 16]; 8],
}

impl GostCypher {
    // Инициализируем массив S-Boxes
    fn new() -> Self {
        let s_box = [
            [4, 10, 9, 2, 13, 8, 0, 14, 6, 11, 1, 12, 7, 15, 5, 3],
            [14, 11, 4, 12, 6, 13, 15, 10, 2, 3, 8, 1, 0, 7, 5, 9],
            [5, 8, 1, 13, 10, 3, 4, 2, 14, 15, 12, 7, 6, 0, 9, 11],
            [7, 13, 10, 1, 0, 8, 9, 15, 14, 4, 6, 12, 11, 2, 5, 3],
            [6, 12, 7, 1, 5, 15, 13, 8, 4, 10, 9, 14, 0, 3, 11, 2],
            [4, 11, 10, 0, 7, 2, 1, 13, 3, 6, 8, 5, 9, 12, 15, 14],
            [13, 11, 4, 1, 3, 15, 5, 9, 0, 10, 14, 7, 6, 8, 2, 12],
            [1, 15, 13, 0, 5, 7, 10, 4, 9, 2, 3, 14, 6, 11, 8, 12],
        ];
        GostCypher { s_box }
    }

    // Реализует функцию `f` алгоритма ГОСТ 28147-89.
    // Принимает правую половину блока данных (`right`) и раундовый ключ (`k_i`) в качестве входных данных.
    // Выполняет сложение по модулю 2, подстановку с помощью S-блоков и циклический сдвиг.
    fn f(&self, right: u32, k_i: u32) -> u32 {
        let mut right = right.wrapping_add(k_i) & 0xFFFFFFFF;
        right = self.s(right);
        ((right << 11) & 0xFFFFFFFF) | (right >> 21)
    }

    // Выполняет подстановку байтов правой половины блока данных с помощью S-блоков.
    // Каждый байт заменяется на соответствующее значение из S-блока.
    fn s(&self, right: u32) -> u32 {
        let mut result = 0;
        for i in 0..8 {
            let s_val = self.s_box[i][((right >> (4 * i)) & 0xf) as usize];
            result |= (s_val as u32) << (4 * i);
        }
        result
    }

    // Выполняет один раунд шифрования.
    // Принимает левую и правую половины блока данных и раундовый ключ в качестве входных данных.
    // Возвращает новые значения левой и правой половин после раунда шифрования.
    fn encryption_round(&self, input_left: u32, input_right: u32, round_key: u32) -> (u32, u32) {
        let output_left = input_right;
        let output_right = input_left ^ self.f(input_right, round_key);
        (output_left, output_right)
    }

    // Выполняет один раунд расшифрования. Аналогичен `encryption_round`,
    // но использует обратный порядок раундовых ключей.
    fn decryption_round(&self, input_left: u32, input_right: u32, round_key: u32) -> (u32, u32) {
        let output_right = input_left;
        let output_left = input_right ^ self.f(input_left, round_key);
        (output_left, output_right)
    }

    // Шифрует 64-битный блок данных (`block`) с использованием заданного ключа (`key`).
    // Выполняет 32 раунда шифрования.
    fn encrypt(&self, block: u64, key: &[u32; 8]) -> u64 {
        let mut left = (block >> 32) as u32;
        let mut right = (block & 0xFFFFFFFF) as u32;

        for i in 0..32 {
            let k_i = if i < 24 { key[i % 8] } else { key[7 - (i % 8)] };
            let (new_left, new_right) = self.encryption_round(left, right, k_i);
            left = new_left;
            right = new_right;
        }

        ((left as u64) << 32) | (right as u64)
    }

    // Расшифровывает 64-битный блок данных (`block`) с использованием заданного ключа (`key`).
    // Выполняет 32 раунда расшифрования с обратным порядком раундовых ключей.
    fn decrypt(&self, block: u64, key: &[u32; 8]) -> u64 {
        let mut left = (block >> 32) as u32;
        let mut right = (block & 0xFFFFFFFF) as u32;

        for i in 0..32 {
            let k_i = if i < 8 { key[7 - i] } else { key[i % 8] };
            let (new_left, new_right) = self.decryption_round(left, right, k_i);
            left = new_left;
            right = new_right;
        }

        ((left as u64) << 32) | (right as u64)
    }
}

#[cfg(test)]
mod tests {
    use crate::information_security::ghost_task::GostCypher;
    use std::time::Instant;

    #[test]
    fn decrypt_success() {
        let cipher = GostCypher::new();
        let data = 0xb202da8a5342f0ac_u64; // 64-bit block
        let key = [
            0xFFFFFFFF, 0x12345678, 0x00120477, 0x77AE441F, 0x81C63123, 0x99DEEEEE, 0x09502978,
            0x68FA3105,
        ];

        let g = 128 * 1024; // One MB
        let start = Instant::now();

        for _ in 0..g {
            let _ = cipher.encrypt(data, &key);
        }

        let duration = start.elapsed().as_secs_f64() / g as f64;

        let encrypted = cipher.encrypt(data, &key);
        let decrypted = cipher.decrypt(encrypted, &key);

        println!("===============================");
        println!("Average time per encryption: {:.10} seconds", duration);
        println!(
            "Text: {:#x}\nCiphertext: {:#x}\nDecrypted text: {:#x}\nMatch: {}",
            data,
            encrypted,
            decrypted,
            data == decrypted
        );
    }
}
