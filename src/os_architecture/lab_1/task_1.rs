use std::fs::File;
use std::process;

fn file_open(file_path: &str) {
    match File::open(file_path) {
        Ok(file) => {
            println!("Open successfully: {}", file_path);
            if let Ok(metadata) = file.metadata() {
                println!("File metadata: {:?}", metadata);
            }
        }
        Err(_) => {
            eprintln!("Failed to open file: {}", file_path);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::os_architecture::lab_1::task_1::file_open;
    use std::env;

    #[test]
    fn open_file() {
        let args: Vec<String> = env::args().collect();
        if args.len() == 1 {
            panic!("invalid number of arguments, expecting file name")
        }
        file_open(&args[args.len() - 1]);
    }
}