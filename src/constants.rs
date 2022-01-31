#![allow(dead_code)]
pub const HTTP_VERSION: &str = "1.1";
pub const HTTP_EOL: &str = "\r\n";

pub const PORT: i32 = 5050;

pub const PRINT_REQUEST: bool = false;
pub const PRINT_RESPONSE: bool = false;
pub const PRINT_SERVE: bool = true;
pub const PRINT_ALL_ROUTES: bool = true;

pub const MAX_REQUEST_SIZE: usize = 1000;

pub const PUBLIC_FOLDER: &str = "public";
pub const PUBLIC_FILEPATH: &str = "./public/";

pub const PRINT_DATE_FORMAT: &str = "%H:%M:%S";
pub const RESPONSE_DATE_FORMAT: &str = "%a, %b %d %Y %H:%M:%S GMT";
