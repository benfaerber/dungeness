use chrono::offset::Local;
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

    pub fn file(filepath: &str, is_download: bool) -> Self {
        let filepath_chunk: Vec<&str> = filepath.split("/").collect();
        let filename = filepath_chunk[filepath_chunk.len() - 1];

        let content_type = ContentType::from_filename(filename);
        let mut headers = HashMap::new();

        if is_download {
            headers.insert(
                "Content-Disposition".to_string(),
                format!("attachment; filename=\"{}\"", filename),
            );
            headers.insert(
                "Content-Transfer-Encoding".to_string(),
                "binary".to_string(),
            );
        }

        Response {
            content_type,
            headers,

            status_code: 200,
            content: "".to_string(),
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

    pub fn get_header(&self, content_len: usize) -> String {
        let prefix = format!("HTTP/{} {} OK", constants::HTTP_VERSION, self.status_code);

        let mut extra_headers = HashMap::new();

        extra_headers.insert(
            "Content-Type".to_string(),
            format!("{}; charset=utf-8", self.content_type),
        );
        extra_headers.insert("Content-Length".to_string(), content_len.to_string());

        //let content_len = self.content.as_bytes().len();

        let timestamp = Local::now()
            .format(constants::RESPONSE_DATE_FORMAT)
            .to_string();
        extra_headers.insert("Date".to_string(), timestamp);
        //Date: Mon, 10 Jan 2022 01:36:33 GMT

        let all_headers: HashMap<String, String> = extra_headers
            .into_iter()
            .chain(self.headers.clone())
            .collect();

        let headers_list: String = all_headers
            .iter()
            .map(|(key, val)| format!("{}: {}", key, val))
            .collect::<Vec<String>>()
            .join(constants::HTTP_EOL);

        vec![prefix, headers_list].join(constants::HTTP_EOL)
    }

    pub fn prepend_header_bytes(&self, body_bytes: Vec<u8>) -> Vec<u8> {
        let header = self.get_header(body_bytes.len());
        let header_padded = format!("{}{}", header, constants::HTTP_EOL.repeat(2));
        let raw_header: &[u8] = header_padded.as_bytes();
        let raw_body: &[u8] = &body_bytes[..];

        let mut raw_res = vec![];
        raw_res.extend_from_slice(raw_header);
        raw_res.extend_from_slice(raw_body);

        raw_res
    }

    pub fn get_raw(&self) -> String {
        let header = self.get_header(self.content.len());
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
