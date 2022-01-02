use std::io::{Result, Read};
use std::net::{TcpStream};
use std::str;
use std::collections::HashMap;

#[path = "./request_parser.rs"]
mod request_parser;

pub type HttpMethod = request_parser::http_method::HttpMethod;
pub type Request = request_parser::Request;
pub type RouteInfo = request_parser::RouteInfo;

const MAX_REQUEST_SIZE: usize = 1000;

pub fn get_request(stream: &mut TcpStream) -> Result<Request> {
  let raw_request = get_raw_request(stream)?;

  let mut connection_data: HashMap<String, String> = HashMap::new();
  let peer_addr = stream.peer_addr().unwrap().to_string();
  connection_data.insert("Peer-Address".to_string(), peer_addr);
  let req: Request = request_parser::parse(raw_request, connection_data);
  Ok(req)
}

fn get_raw_request(stream: &mut TcpStream) -> Result<String> {
  let mut buffer: Vec<u8> = Vec::new();
  buffer.resize(MAX_REQUEST_SIZE, 0);
  let bytes_read = stream.read(&mut buffer)?;
  let request_data = &buffer[0..bytes_read];
  let request_string = str::from_utf8(request_data).unwrap_or("").to_string();

  Ok(request_string)
}