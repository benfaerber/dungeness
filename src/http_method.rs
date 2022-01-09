use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
    ANY,
}

impl HttpMethod {
    fn to_string(&self) -> String {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::CONNECT => "CONNECT",
            HttpMethod::OPTIONS => "OPTIONS",
            HttpMethod::TRACE => "TRACE",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::ANY => "ANY",
        }
        .to_string()
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "GET" => HttpMethod::GET,
            "HEAD" => HttpMethod::HEAD,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "CONNECT" => HttpMethod::CONNECT,
            "OPTIONS" => HttpMethod::OPTIONS,
            "TRACE" => HttpMethod::TRACE,
            "PATCH" => HttpMethod::PATCH,
            "ANY" => HttpMethod::ANY,
            _ => HttpMethod::GET,
        }
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
