use std::fs::File;
use std::io::Read;

pub fn decrypt(filename: String) {
    let mut f = File::open(filename).unwrap();

    let mut buff = vec![0u8; 10];

    f.read(&mut buff).unwrap();

    println!("--------------- MATRIX ---------------");

    for i in 0..6 {
        let row = format!("{:08b}", buff[i]);

        for (j, bit) in row.chars().skip(1).take(6).enumerate() {
            print!("{}", bit);
            if j < 5 {
                print!(",");
            }
        }
        println!()
    }
    let mut key = String::new();

    for i in 6..9 {
        key += &format!("{:02x}", buff[i]);
    }
    let num = i64::from_str_radix(&key, 16).unwrap();

    println!("--------------- KEY ---------------");
    println!("{}", num);
}

#[cfg(test)]
mod tests {
    use super::decrypt;
    use std::fs;

    #[test]
    fn decrypt_success() {
        let paths = fs::read_dir(".").unwrap();

        for path in paths {
            if let Ok(path) = path.as_ref() {
                let path_str = path.path().display().to_string();
                if path_str.ends_with(".EFE") {
                    decrypt(path_str);
                }
            }
        }
    }

    #[test]
    #[should_panic]
    fn decrypt_failed() {
        decrypt(String::from("MY11.EFE"));
    }
}