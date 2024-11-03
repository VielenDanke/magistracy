use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Error, Write};

fn copy_files(files: Vec<String>) -> Result<(), Error> {
    if files.len() < 2 {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        for line in stdin.lock().lines() {
            stdout.write(line?.as_bytes())?;
        }
        Ok(())
    } else {
        let source = File::open(&files[0])?;
        let target = File::create(&files[1])?;

        let buff_reader = BufReader::new(source);
        let mut buff_writer = BufWriter::new(target);

        for line in buff_reader.lines() {
            buff_writer.write(line?.as_bytes())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::os_architecture::lab_1::task_8::copy_files;

    #[test]
    fn copy_file() {
        copy_files(vec![String::from("README.md"), String::from("new_file.txt")]).unwrap();
    }

    #[test]
    fn copy_file_std_in_out() {
        copy_files(vec![]).unwrap();
    }
}