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

  pub fn get_raw(&self) -> String {
    let header = self.get_header();
    format!("{}\r\n\r\n{}", header, self.content)
  }
}

pub struct ResponseStatus {
  status_code: i32
}

impl ResponseStatus {
  pub fn text(&self, content: String) -> Response {
    Response {
      status_code: self.status_code,
      content: content,
      content_type: ContentType::TextPlain
    }
  }
}

pub fn status(status_code: i32) -> ResponseStatus {
  ResponseStatus { status_code }
}

// TODO: Implement all these content types