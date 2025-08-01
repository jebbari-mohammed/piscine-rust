use std::fs::File;

pub fn open_file(s: &str) -> File {
    let f = File::open(s);
    match f {
        Ok(file) => file,
        Err(error) => panic!("{}", error)
    }
}