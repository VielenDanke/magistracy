/*
Принимает правую часть блока (right), ключ раунда (k_i) и S-блоки (s_box).
Складывает right с ключом раунда по модулю 2^32 (& 0xFFFFFFFF).
Применяет S-блоки к результату (подробнее о функции s ниже).
Циклически сдвигает результат на 11 бит влево.
 */
fn f(mut right: u64, k_i: u64, s_box: &Vec<Vec<u64>>) -> u64 {
    right = (right + k_i) & 0xFFFFFFFF;
    right = s(right, s_box);
    ((right << 11) & 0xFFFFFFFF) | (right >> 21)
}

/*
Реализует подстановку с помощью S-блоков.
Разбивает right на 8 4-битных блоков.
Каждый блок используется как индекс для выбора значения из соответствующего S-блока.
Результаты объединяются в 32-битное слово.
 */
fn s(right: u64, s_box: &Vec<Vec<u64>>) -> u64 {
    let mut result = 0;
    for i in 0usize..8 {
        result |= (s_box[i][((right >> (4 * i)) & 0xf) as usize]) << (4 * i)
    }
    result
}

/*
Реализуют один раунд шифрования/дешифрования.
В шифровании:
Левая часть выходного блока равна правой части входного.
Правая часть выходного блока равна XOR левой части входного блока и результату функции f.

В дешифровании порядок операций меняется (правая часть становится левой и наоборот).
 */
fn encryption_round(input_left: u64, input_right: u64, round_key: u64, s_box: &Vec<Vec<u64>>) -> (u64, u64) {
    let output_left = input_right;
    let output_right = input_left ^ f(input_right, round_key, s_box);
    (output_left, output_right)
}

fn decryption_round(input_left: u64, input_right: u64, round_key: u64, s_box: &Vec<Vec<u64>>) -> (u64, u64) {
    let output_right = input_left;
    let output_left = input_right ^ f(input_left, round_key, s_box);
    (output_left, output_right)
}

/*
Реализуют полный цикл шифрования/дешифрования.
Разбивают 64-битный блок на левую и правую части.
Выполняют 32 раунда шифрования/дешифрования.
В шифровании используется схема расширения ключа: первые 24 раунда ключи циклически повторяются,
а в последних 8 раундах ключи используются в обратном порядке.

В дешифровании используется немного другая схема расширения ключа.
Объединяют левую и правую части в 64-битный результат.
 */
fn encrypt(block: u64, key: &Vec<u64>, s_box: &Vec<Vec<u64>>) -> u64 {
    let (mut left, mut right) = (block >> 32, block & 0xFFFFFFFF);
    for i in 0usize..32 {
        let k_i = if i < 24 { key[i % 8] } else { key[7 - (i % 8)] };
        let temp = encryption_round(left, right, k_i, s_box);
        left = temp.0;
        right = temp.1;
    }
    (left << 32) | right
}

fn decrypt(block: u64, key: &Vec<u64>, s_box: &Vec<Vec<u64>>) -> u64 {
    let (mut left, mut right) = (block >> 32, block & 0xFFFFFFFF);
    for i in 0usize..32 {
        let k_i = if i < 8 { key[i] } else { key[7 - (i % 8)] };
        let temp = decryption_round(left, right, k_i, s_box);
        left = temp.0;
        right = temp.1;
    }
    (left << 32) | right
}

#[cfg(test)]
mod tests {
    use crate::information_security::ghost_task::{decrypt, encrypt};
    use std::time::Instant;

    #[test]
    fn decrypt_success() {
        let s_box = vec![vec![4, 10, 9, 2, 13, 8, 0, 14, 6, 11, 1, 12, 7, 15, 5, 3],
                         vec![14, 11, 4, 12, 6, 13, 15, 10, 2, 3, 8, 1, 0, 7, 5, 9],
                         vec![5, 8, 1, 13, 10, 3, 4, 2, 14, 15, 12, 7, 6, 0, 9, 11],
                         vec![7, 13, 10, 1, 0, 8, 9, 15, 14, 4, 6, 12, 11, 2, 5, 3],
                         vec![6, 12, 7, 1, 5, 15, 13, 8, 4, 10, 9, 14, 0, 3, 11, 2],
                         vec![4, 11, 10, 0, 7, 2, 1, 13, 3, 6, 8, 5, 9, 12, 15, 14],
                         vec![13, 11, 4, 1, 3, 15, 5, 9, 0, 10, 14, 7, 6, 8, 2, 12],
                         vec![1, 15, 13, 0, 5, 7, 10, 4, 9, 2, 3, 14, 6, 11, 8, 12]];
        let s_key: Vec<u64> = vec![0xFFFFFFFF, 0x12345678, 0x00120477, 0x77AE441F, 0x81C63123, 0x99DEEEEE, 0x09502978, 0x68FA3105];
        let data = 0xFE12847EFE12847Eu64;

        let start_encryption_time = Instant::now();

        let ct = encrypt(data, &s_key, &s_box);

        println!("Encryption time: {:.2?}", start_encryption_time.elapsed());

        let start_decryption_time = Instant::now();

        let decrypted = decrypt(ct, &s_key, &s_box);

        println!("Decryption time: {:.2?}", start_decryption_time.elapsed());

        assert_eq!(data, decrypted);
    }
}