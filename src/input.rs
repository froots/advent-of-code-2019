use std::fs::File;
use std::io::prelude::*;

pub fn read(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read file");
    Ok(contents)
}
