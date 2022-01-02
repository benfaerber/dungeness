use std::io::{Result};
use std::net::{TcpListener, TcpStream, Shutdown};

const PORT: i32 = 5050;

mod response;
mod request;

fn handle_client(stream: &mut TcpStream) -> Result<()> {
    let req = request::get_request(stream).unwrap();

    let test_body = "Hello world!".to_string();

    let res = response::Response {
        status_code: 200,
        content_type: response::ContentType::TextHtml,
        content: test_body
    };
    res.send(stream)?;

    stream.shutdown(Shutdown::Both)?;
    Ok(())
}

fn listen_on(port: i32) -> Result<()> {
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(address)?;
    for stream in listener.incoming() {
        handle_client(&mut stream?)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    println!("Dungeness");
    println!("---------");
    println!("Listening on http://localhost:{}", PORT);

    listen_on(PORT)
}
