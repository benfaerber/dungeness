use std::io::{Result, Write};
use std::net::{TcpStream};

#[path = "./content_type.rs"]
mod content_type;

pub type ContentType = content_type::ContentType;

#[derive(Debug)]
pub struct Response {
  pub status_code: i32,
  pub content_type: ContentType,
  pub content: String
}

impl Response {
  fn get_header(&self) -> String {
    let lines: Vec<String> = vec![
      format!("HTTP/1.1 {} OK", self.status_code),
      format!("Content-Type: {}; charset=utf-8", self.content_type)
    ];

    lines.join("\r\n")
  }

  fn get_raw(&self) -> String {
    let header = self.get_header();
    format!("{}\r\n\r\n{}", header, self.content)
  }

  pub fn send(&self, stream: &mut TcpStream) -> Result<()> {
    let raw_response = self.get_raw();
    let response_bytes = raw_response.as_bytes();
    stream.write(response_bytes)?;
    Ok(())
  }
}

// TODO: Implement all these content types