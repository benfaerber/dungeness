use std::fmt;

// Generated from: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#information_responses

enum StatusCode {

  // * Information: 100-199
  Continue100,
  SwitchingProtocols101,
  Processing102,
  EarlyHints103,

  // * Success: 200-299
  Ok200,
  Created201,
  Accepted202,
  NonAuthoritativeInformation203,
  NoContent204,
  ResetContent205,
  PartialContent206,
  MultiStatus207,
  AlreadyReported208,
  ImUsed226,

  // * Redirection: 300-399
  MultipleChoice300,
  MovedPermantly301,
  Found302,
  SeeOther303,
  NotModified304,
  UseProxy305,
  Unused306,
  TemporaryRedirect307,
  PermanentRedirect308,

  // * Client Errors: 400-499
  BadRequest400,
  Unauthorized401,
  PaymentRequired402,
  Forbidden403,
  NotFound404,
  MethodNotAllowed405,
  NotAcceptable406,
  ProxyAuthenticationRequired407,
  RequestTimeout408,
  Conflict409,
  Gone410,
  LengthRequired411,
  PreconditionFailed412,
  PayloadTooLarge413,
  UnsupportedMediaType415,
  RangeNotSatisfiable416,
  ExpectationFailed417,
  ImATeapot418,
  MisdirectedRequest421,
  UnprocessableEntity422,
  Locked423,
  FailedDependency424,
  TooEarly425,
  UpgradeRequired426,
  PreconditionRequired428,
  TooManyRequests429,
  // TODO: Return this when 'bytes_recieved' greater than MAX_REQ
  RequestHeaderFieldsTooLarge431,
  UnavailableForLegalReasons451,

  // * Server Errors: 500-599
  InternalServerError500,
  NotImplemented501,
  BadGateway502,
  ServiceUnavailable503,
  GatewayTimeout504,
  HttpVersionNotSupported505,
  VariantAlsoNegotiates506,
  InsufficientStorage507,
  LoopDetected508,
  NotExtended510,
  NetworkAuthenticationRequired511,
}

impl StatusCode {
  pub fn code(&self) -> i32 {
    match self {
      // * Information: 100-199
      Self::Continue100 => 100,
      Self::SwitchingProtocols101 => 101,
      Self::Processing102 => 102,
      Self::EarlyHints103 => 103,

      // * Success: 200-299
      Self::Ok200 => 200,
      Self::Created201 => 201,
      Self::Accepted202 => 202,
      Self::NonAuthoritativeInformation203 => 203,
      Self::NoContent204 => 204,
      Self::ResetContent205 => 205,
      Self::PartialContent206 => 206,
      Self::MultiStatus207 => 207,
      Self::AlreadyReported208 => 208,
      Self::ImUsed226 => 226,

      // * Redirection: 300-399
      Self::MultipleChoice300 => 300,
      Self::MovedPermantly301 => 301,
      Self::Found302 => 302,
      Self::SeeOther303 => 303,
      Self::NotModified304 => 304,
      Self::UseProxy305 => 305,
      Self::Unused306 => 306,
      Self::TemporaryRedirect307 => 307,
      Self::PermanentRedirect308 => 308,

      // * Client Errors: 400-499
      Self::BadRequest400 => 400,
      Self::Unauthorized401 => 401,
      Self::PaymentRequired402 => 402,
      Self::Forbidden403 => 403,
      Self::NotFound404 => 404,
      Self::MethodNotAllowed405 => 405,
      Self::NotAcceptable406 => 406,
      Self::ProxyAuthenticationRequired407 => 407,
      Self::RequestTimeout408 => 408,
      Self::Conflict409 => 409,
      Self::Gone410 => 410,
      Self::LengthRequired411 => 411,
      Self::PreconditionFailed412 => 412,
      Self::PayloadTooLarge413 => 413,
      Self::UnsupportedMediaType415 => 415,
      Self::RangeNotSatisfiable416 => 416,
      Self::ExpectationFailed417 => 417,
      Self::ImATeapot418 => 418,
      Self::MisdirectedRequest421 => 421,
      Self::UnprocessableEntity422 => 422,
      Self::Locked423 => 423,
      Self::FailedDependency424 => 424,
      Self::TooEarly425 => 425,
      Self::UpgradeRequired426 => 426,
      Self::PreconditionRequired428 => 428,
      Self::TooManyRequests429 => 429,
      // TODO: Return this when 'bytes_recieved' greater than MAX_REQ
      Self::RequestHeaderFieldsTooLarge431 => 431,
      Self::UnavailableForLegalReasons451 => 451,

      // * Server Errors: 500-599
      Self::InternalServerError500 => 500,
      Self::NotImplemented501 => 501,
      Self::BadGateway502 => 502,
      Self::ServiceUnavailable503 => 503,
      Self::GatewayTimeout504 => 504,
      Self::HttpVersionNotSupported505 => 505,
      Self::VariantAlsoNegotiates506 => 506,
      Self::InsufficientStorage507 => 507,
      Self::LoopDetected508 => 508,
      Self::NotExtended510 => 510,
      Self::NetworkAuthenticationRequired511 => 511,
    }
  }

  pub fn name(&self) -> String {
    match self {
      // * Information: 100-199
      Self::Continue100 => "Continue",
      Self::SwitchingProtocols101 => "Switching Protocols",
      Self::Processing102 => "Processing",
      Self::EarlyHints103 => "Early Hints",

      // * Success: 200-299
      Self::Ok200 => "Ok",
      Self::Created201 => "Created",
      Self::Accepted202 => "Accepted",
      Self::NonAuthoritativeInformation203 => "Non Authoritative Information",
      Self::NoContent204 => "No Content",
      Self::ResetContent205 => "Reset Content",
      Self::PartialContent206 => "Partial Content",
      Self::MultiStatus207 => "Multi-Status",
      Self::AlreadyReported208 => "Already Reported",
      Self::ImUsed226 => "IM Used",

      // * Redirection: 300-399
      Self::MultipleChoice300 => "Multiple Choice",
      Self::MovedPermantly301 => "Moved Permantly",
      Self::Found302 => "Found",
      Self::SeeOther303 => "See Other",
      Self::NotModified304 => "Not Modified",
      Self::UseProxy305 => "Use Proxy",
      Self::Unused306 => "Unused",
      Self::TemporaryRedirect307 => "Temporary Redirect",
      Self::PermanentRedirect308 => "Permanent Redirect",

      // * Client Errors: 400-499
      Self::BadRequest400 => "Bad Request",
      Self::Unauthorized401 => "Unauthorized",
      Self::PaymentRequired402 => "Payment Required",
      Self::Forbidden403 => "Forbidden",
      Self::NotFound404 => "Not Found",
      Self::MethodNotAllowed405 => "Method Not Allowed",
      Self::NotAcceptable406 => "Not Acceptable",
      Self::ProxyAuthenticationRequired407 => "Proxy Authentication Required",
      Self::RequestTimeout408 => "Request Timeout",
      Self::Conflict409 => "Conflict",
      Self::Gone410 => "Gone",
      Self::LengthRequired411 => "Length Required",
      Self::PreconditionFailed412 => "Precondition Failed",
      Self::PayloadTooLarge413 => "Payload Too Large",
      Self::UnsupportedMediaType415 => "Unsupported Media Type",
      Self::RangeNotSatisfiable416 => "Range Not Satisfiable",
      Self::ExpectationFailed417 => "Expectation Failed",
      Self::ImATeapot418 => "I'm a Teapot",
      Self::MisdirectedRequest421 => "Misdirected Request",
      Self::UnprocessableEntity422 => "Unprocessable Entity",
      Self::Locked423 => "Locked",
      Self::FailedDependency424 => "Failed Dependency",
      Self::TooEarly425 => "Too Early",
      Self::UpgradeRequired426 => "Upgrade Required",
      Self::PreconditionRequired428 => "Precondition Required",
      Self::TooManyRequests429 => "Too Many Requests",
      // TODO: Return this when 'bytes_recieved' greater than MAX_REQ
      Self::RequestHeaderFieldsTooLarge431 => "Request Header Fields Too Large",
      Self::UnavailableForLegalReasons451 => "Unavailable For Legal Reasons",

      // * Server Errors: 500-599
      Self::InternalServerError500 => "Internal Server Error",
      Self::NotImplemented501 => "Not Implemented",
      Self::BadGateway502 => "Bad Gateway",
      Self::ServiceUnavailable503 => "Service Unavailable",
      Self::GatewayTimeout504 => "Gateway Timeout",
      Self::HttpVersionNotSupported505 => "HTTP Version Not Supported",
      Self::VariantAlsoNegotiates506 => "Variant Also Negotiates",
      Self::InsufficientStorage507 => "Insufficient Storage",
      Self::LoopDetected508 => "Loop Detected",
      Self::NotExtended510 => "Not Extended",
      Self::NetworkAuthenticationRequired511 => "Network Authentication Required",
    }.to_string()
  }

  pub fn from(code: i32) -> Self {
    match code {
      // * Information: 100-199
      100 => Self::Continue100,
      101 => Self::SwitchingProtocols101,
      102 => Self::Processing102,
      103 => Self::EarlyHints103,

      // * Success: 200-299
      200 => Self::Ok200,
      201 => Self::Created201,
      202 => Self::Accepted202,
      203 => Self::NonAuthoritativeInformation203,
      204 => Self::NoContent204,
      205 => Self::ResetContent205,
      206 => Self::PartialContent206,
      207 => Self::MultiStatus207,
      208 => Self::AlreadyReported208,
      226 => Self::ImUsed226,

      // * Redirection: 300-399
      300 => Self::MultipleChoice300,
      301 => Self::MovedPermantly301,
      302 => Self::Found302,
      303 => Self::SeeOther303,
      304 => Self::NotModified304,
      305 => Self::UseProxy305,
      306 => Self::Unused306,
      307 => Self::TemporaryRedirect307,
      308 => Self::PermanentRedirect308,

      // * Client Errors: 400-499
      400 => Self::BadRequest400,
      401 => Self::Unauthorized401,
      402 => Self::PaymentRequired402,
      403 => Self::Forbidden403,
      404 => Self::NotFound404,
      405 => Self::MethodNotAllowed405,
      406 => Self::NotAcceptable406,
      407 => Self::ProxyAuthenticationRequired407,
      408 => Self::RequestTimeout408,
      409 => Self::Conflict409,
      410 => Self::Gone410,
      411 => Self::LengthRequired411,
      412 => Self::PreconditionFailed412,
      413 => Self::PayloadTooLarge413,
      415 => Self::UnsupportedMediaType415,
      416 => Self::RangeNotSatisfiable416,
      417 => Self::ExpectationFailed417,
      418 => Self::ImATeapot418,
      421 => Self::MisdirectedRequest421,
      422 => Self::UnprocessableEntity422,
      423 => Self::Locked423,
      424 => Self::FailedDependency424,
      425 => Self::TooEarly425,
      426 => Self::UpgradeRequired426,
      428 => Self::PreconditionRequired428,
      429 => Self::TooManyRequests429,
      // TODO: Return this when 'bytes_recieved' greater than MAX_REQ
      431 => Self::RequestHeaderFieldsTooLarge431,
      451 => Self::UnavailableForLegalReasons451,

      // * Server Errors: 500-599
      500 => Self::InternalServerError500,
      501 => Self::NotImplemented501,
      502 => Self::BadGateway502,
      503 => Self::ServiceUnavailable503,
      504 => Self::GatewayTimeout504,
      505 => Self::HttpVersionNotSupported505,
      506 => Self::VariantAlsoNegotiates506,
      507 => Self::InsufficientStorage507,
      508 => Self::LoopDetected508,
      510 => Self::NotExtended510,
      511 => Self::NetworkAuthenticationRequired511,
    }
  }
}
