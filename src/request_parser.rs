use std::collections::{HashMap};

const HTTP_EOL: &str = "\r\n";

#[path = "./http_method.rs"]
pub mod http_method;

#[derive(Debug, Clone)]
pub struct Request {
  pub method: http_method::HttpMethod,
  pub route: RouteInfo,
  pub headers: HashMap<String, String>
}

impl Request {
  pub fn get(&self, key: &str) -> Option<&String> {
    self.route.query.get(key)
  }
}

#[derive(Debug, Clone)]
pub struct RouteInfo {
  pub path: String,
  pub paths: Vec<String>,
  pub query: Query
}

pub type Query = HashMap<String, String>;

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

fn parse_query(raw_query: &str) -> Query {
  // TODO: Splitting by '&' is very naive approch, '&amp;' and other encoded chars will break it
  let raw_pairs: Vec<&str> = raw_query.split("&").collect();
  let pairs: Vec<(String, String)> = raw_pairs
  .iter()
  .map(|raw_pair| {
    let is_kv = raw_pair.contains("=");
    if is_kv {
      let kvparts: Vec<&str> = raw_pair.split("=").collect();
      let key = kvparts[0];
      let val = kvparts[1];

      (key.to_string(), val.to_string())
    } else {
      (raw_pair.to_string(), "".to_string())
    }
  })
  .filter(|(a, _)| a.to_owned() != "".to_string())
  .collect();

  pairs.into_iter().collect()
}

fn parse_route(raw_route: String) -> RouteInfo {
  let start_slash = raw_route.starts_with("/");
  let trimmer = if start_slash {1} else {0};
  let wo_slash = &raw_route[trimmer..];

  let is_index = wo_slash == "";

  let has_query = wo_slash.contains("?");
  let empty_query: Query = HashMap::new();

  let route_parts: Vec<&str> = wo_slash.split("?").collect();
  let query = if has_query {
    let raw_query: &str = route_parts[1];
    parse_query(raw_query)
  } else {
    empty_query
  };

  let path = (if !is_index {route_parts[0]} else {"index"}).to_string();
  let paths: Vec<String> = path
  .split("/")
  .map(|p| p.to_string())
  .collect();

  RouteInfo {
    path,
    paths,
    query
  }
}

pub fn parse(raw_request: String) -> Request {
  let lines: Vec<&str> = raw_request.trim().split(HTTP_EOL).collect();
  let header_line = lines[0];
  let raw_params = &lines[1..];

  let (method, raw_route) = parse_header(header_line);
  let headers = parse_params(raw_params);
  let route = parse_route(raw_route);

  Request {
    method,
    route,
    headers
  }
}
