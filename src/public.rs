use std::fs;
use std::io::Result;
use std::path::Path;
use std::str;

#[path = "./constants.rs"]
mod constants;

fn get_file_bytes(relative_path: &str) -> Result<Vec<u8>> {
    let public_file = format!("{}{}", constants::PUBLIC_FILEPATH, relative_path);
    std::fs::read(public_file)
}

pub fn get_file(path: &str) -> Result<Vec<u8>> {
    let chunks: Vec<&str> = path
        .split(format!("{}/", constants::PUBLIC_FOLDER).as_str())
        .collect();
    let relative_path = chunks[1];

    get_file_bytes(relative_path)
}
