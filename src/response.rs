#[path = "./content_type.rs"]
mod content_type;

pub type ContentType = content_type::ContentType;

const HTTP_EOL: &str = "\r\n";

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

    lines.join(HTTP_EOL)
  }

  pub fn get_raw(&self) -> String {
    let header = self.get_header();
    format!("{}{}{}", header, HTTP_EOL.repeat(2), self.content)
  }
}

pub struct ResponseStatus {
  status_code: i32
}

impl ResponseStatus {
  fn respond(&self, content_type: ContentType, content: String) -> Response {
    Response {
      status_code: self.status_code,
      content_type,
      content
    }
  }

  pub fn text(&self, content: String) -> Response {
    self.respond(ContentType::TextPlain, content)
  }

  pub fn html(&self, content: String) -> Response {
    self.respond(ContentType::TextHtml, content)
  }

  pub fn html_body(&self, content: String) -> Response {
    let lines = vec![
      "<!DOCTYPE html>",
      "<html lang=\"en\">",
      "<head>",
      "  <meta charset=\"UTF-8\">",
      "  <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">",
      "  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">",
      "  <title>Document</title>",
      "</head>",
      "<body>",
      content.as_str(),
      "</body>",
      "</html>",
    ];

    self.html(lines.join("\n"))
  }

  pub fn content(&self, content_type: ContentType, content: String) -> Response {
    self.respond(content_type, content)
  }
}

pub fn status(status_code: i32) -> ResponseStatus {
  ResponseStatus { status_code }
}