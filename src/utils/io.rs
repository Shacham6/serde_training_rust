use std::io::Read;
use std::io::Error;
use std::fs::File;


pub fn read_content_of_file_to_string<'a>(filepath: &'a str, buffer: &mut String) -> Result<usize, Error> {
    let mut file = File::open(filepath).unwrap();
    file.read_to_string(buffer)
}

pub fn read_content_of_file<'a>(filepath: &'a str) -> Result<String, Error> {
    let mut buffer = String::new();
    match read_content_of_file_to_string(filepath, &mut buffer) {
        Ok(_) => Ok(buffer),
        Err(error) => Err(error)
    }
}

