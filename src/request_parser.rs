use std::collections::{HashMap};

const HTTP_EOL: &str = "\r\n";

#[path = "./http_method.rs"]
pub mod http_method;

#[derive(Debug)]
pub struct Request {
  method: http_method::HttpMethod,
  route: String,
  headers: HashMap<String, String>
}

fn parse_header(raw_header: &str) -> (http_method::HttpMethod, String) {
  let parts: Vec<&str> = raw_header.split(" ").collect();
  let method_str = parts[0];
  let route = parts[1].to_string();

  let method = http_method::HttpMethod::from_str(method_str);
  (method, route)
}

fn parse_params(raw_params: &[&str]) -> HashMap<String, String> {
  let pairs: Vec<(String, String)> = raw_params
  .iter()
  .map(|line| {
    let pairs: Vec<&str> = line.split(":").collect();
    let extract = |i: usize| pairs[i].trim().to_string();
    let key = extract(0);
    let val = extract(1);

    (key, val)
  })
  .collect();

  let request_headers: HashMap<String, String> = pairs.into_iter().collect();
  request_headers
}

pub fn parse(raw_request: String) -> Request {
  let lines: Vec<&str> = raw_request.trim().split(HTTP_EOL).collect();
  let header_line = lines[0];
  let raw_params = &lines[1..];

  let (method, route) = parse_header(header_line);
  let headers = parse_params(raw_params);

  Request {
    method,
    route,
    headers
  }
}