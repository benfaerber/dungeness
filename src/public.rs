use std::fs;
use std::io::{Result, Error};
use std::path::Path;
use std::str;

#[path = "./constants.rs"]
mod constants;

fn get_file_bytes(relative_path: &str) -> Result<Vec<u8>> {
  let public_file = format!("{}{}", constants::PUBLIC_DIRECTORY, relative_path);
  std::fs::read(public_file)
}

pub fn get_file(path: &str) -> Result<Vec<u8>> {
  let chunks: Vec<&str> = path.split("public/").collect();
  let relative_path = chunks[1];

  get_file_bytes(relative_path)
}
