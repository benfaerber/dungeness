use std::collections::HashMap;

#[path = "./content_type.rs"]
mod content_type;

#[path = "./constants.rs"]
mod constants;

pub type ContentType = content_type::ContentType;

#[derive(Debug, Clone)]
pub struct Response {
    pub status_code: i32,
    pub content_type: ContentType,
    pub content: String,
    pub headers: HashMap<String, String>,
}

// TODO: Add ability to pass custom response headers
impl Response {
    fn default() -> Self {
        Response {
            status_code: 200,
            content_type: ContentType::TextPlain,
            content: "".to_string(),
            headers: HashMap::new(),
        }
    }

    pub fn file(content_type: ContentType) -> Self {
        Response {
            content_type,
            status_code: 200,
            content: "".to_string(),
            headers: HashMap::new(),
        }
    }

    pub fn status(&self, status_code: i32) -> Self {
        Response {
            status_code,
            content_type: self.content_type,
            content: self.content.clone(),
            headers: self.headers.clone(),
        }
    }

    pub fn headers(&self, headers: HashMap<String, String>) -> Self {
        Response {
            status_code: self.status_code,
            content_type: self.content_type,
            content: self.content.clone(),
            headers,
        }
    }

    fn modify_content(&self, content_type: ContentType, content: String) -> Self {
        Response {
            content,
            content_type,
            status_code: self.status_code,
            headers: self.headers.clone(),
        }
    }

    pub fn text(&self, content: String) -> Self {
        self.modify_content(ContentType::TextPlain, content)
    }

    pub fn html(&self, content: String) -> Self {
        self.modify_content(ContentType::TextHtml, content)
    }

    pub fn html_body(&self, content: String) -> Self {
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

    pub fn content(&self, content_type: ContentType, content: String) -> Self {
        self.modify_content(content_type, content)
    }

    pub fn get_header(&self) -> String {
        let prefix = format!("HTTP/1.1 {} OK", self.status_code);
        let content_type = format!("Content-Type: {}; charset=utf-8", self.content_type);

        let headers_list: String = self
            .headers
            .iter()
            .map(|(key, val)| format!("{}: {}", key, val))
            .collect::<Vec<String>>()
            .join(constants::HTTP_EOL);

        if headers_list.len() == 0 {
            vec![prefix, content_type].join(constants::HTTP_EOL)
        } else {
            vec![prefix, content_type, headers_list].join(constants::HTTP_EOL)
        }
    }

    pub fn prepend_header_bytes(&self, body_bytes: Vec<u8>) -> Vec<u8> {
        let header = self.get_header();
        let header_padded = format!("{}{}", header, constants::HTTP_EOL.repeat(2));
        let raw_header: &[u8] = header_padded.as_bytes();
        let raw_body: &[u8] = &body_bytes[..];

        let mut raw_res = vec![];
        raw_res.extend_from_slice(raw_header);
        raw_res.extend_from_slice(raw_body);

        raw_res
    }

    pub fn get_raw(&self) -> String {
        let header = self.get_header();
        format!(
            "{}{}{}",
            header,
            constants::HTTP_EOL.repeat(2),
            self.content
        )
    }

    pub fn repr(&self) -> String {
        format!(
            "(Status: {}, Content Type: {})",
            self.status_code, self.content_type
        )
    }
}

pub fn response() -> Response {
    Response::default()
}
