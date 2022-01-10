use chrono::offset::Local;
use std::collections::HashMap;
use std::io::{Result, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

#[path = "./response.rs"]
pub mod res;

#[path = "./constants.rs"]
mod constants;
#[path = "./intro.rs"]
mod intro;
#[path = "./public.rs"]
pub mod public;
#[path = "./request.rs"]
mod request;

pub type Response = res::Response;
pub type ContentType = res::ContentType;
pub type HttpMethod = request::HttpMethod;
pub type Request = request::Request;
pub type RouteInfo = request::RouteInfo;
pub type Handler = fn(Request) -> Response;

pub struct Route {
    name: String,
    handler: Handler,
    method: HttpMethod,
}

impl Route {
    fn get_404() -> Self {
        Self {
            name: "404".to_string(),
            handler: |_| res::response().status(404).text("Error 404".to_string()),
            method: request::HttpMethod::GET,
        }
    }

    fn repr(&self) -> String {
        format!("{} {}", self.method, self.name)
    }
}

pub struct Router {
    routes: Vec<Route>,
    error_404: Route,
}

impl Router {
    fn find(&self, route_info: &RouteInfo, method: HttpMethod) -> &Route {
        self.routes
            .iter()
            .find(|route| {
                route.name == route_info.path
                    && (route.method == method || route.method == HttpMethod::ANY)
            })
            .unwrap_or(&self.error_404)
    }
}

fn serve_text_response(stream: &mut TcpStream, req: Request, router: &Router) -> Result<()> {
    let route = router.find(&req.route, req.method);
    let handler = route.handler;

    let con = &req.connection.clone();
    let res = handler(req);
    send_response(stream, &res)?;

    print_serve(&route, &res, &con);

    Ok(())
}

fn send_response(stream: &mut TcpStream, res: &Response) -> Result<()> {
    let raw_response = res.get_raw();
    let response_bytes = raw_response.as_bytes();
    stream.write(response_bytes)?;
    Ok(())
}

fn send_file_response(
    stream: &mut TcpStream,
    filename: &str,
    file: Vec<u8>,
    is_download: bool,
) -> Result<()> {
    let res = Response::file(filename, is_download);
    print_file_serve(filename, &res);

    let raw_res: Vec<u8> = res.prepend_header_bytes(file);
    stream.write(&raw_res[..])?;
    Ok(())
}

fn serve_file(stream: &mut TcpStream, req: Request) -> Result<()> {
    let filename = req.route.path.as_str();

    match public::get_file(filename) {
        Ok(file) => {
            let is_download = match req.get("download") {
                Some(_) => true,
                None => false,
            };

            send_file_response(stream, filename, file, is_download)?;
            Ok(())
        }
        Err(_) => {
            let error_404 = res::response()
                .status(404)
                .text(format!("Error 404: file \"{}\" does not exist!", filename));

            print_file_error_404(&error_404);
            send_response(stream, &error_404)?;
            Ok(())
        }
    }
}

fn get_timestamp_repr() -> String {
    Local::now()
        .format(constants::PRINT_DATE_FORMAT)
        .to_string()
}

fn get_con_repr(con: &HashMap<String, String>) -> String {
    format!("(Peer Addr: {})", con.get("Peer-Address").unwrap())
}

fn print_serve(route: &Route, res: &Response, con: &HashMap<String, String>) {
    if constants::PRINT_SERVE {
        println!(
            "{} - {} {} {}",
            get_timestamp_repr(),
            route.repr(),
            res.repr(),
            get_con_repr(&con)
        );
    }

    if constants::PRINT_RESPONSE {
        println!("{:?}\n", res)
    }
}

fn print_file_error_404(res: &Response) {
    if constants::PRINT_SERVE {
        println!(
            "{} - Public File Error 404 {}",
            get_timestamp_repr(),
            res.repr()
        );
    }
}

fn print_file_serve(filename: &str, res: &Response) {
    if constants::PRINT_SERVE {
        let req_print = format!("Public File {}", filename);
        println!("{} - {} {}", get_timestamp_repr(), req_print, res.repr());
    }
}

fn print_request(req: &Request) {
    if constants::PRINT_REQUEST {
        println!("{:?}\n", req)
    }
}

fn serve(stream: &mut TcpStream, router: &Router) -> Result<()> {
    let req = request::get_request(stream)?;

    print_request(&req);

    let is_public_file = req.route.paths[0] == constants::PUBLIC_FOLDER;
    if is_public_file {
        serve_file(stream, req)?;
    } else {
        serve_text_response(stream, req, router)?;
    }

    stream.shutdown(Shutdown::Both)?;
    Ok(())
}

fn listen_on(port: i32, router: Router) -> Result<()> {
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(address)?;
    for stream in listener.incoming() {
        serve(&mut stream?, &router)?;
    }
    Ok(())
}

// * Public API

pub fn start(router: Router) -> Result<()> {
    intro::display(constants::PORT);
    listen_on(constants::PORT, router)
}

pub fn routes(all_routes: Vec<Route>) -> Router {
    Router {
        routes: all_routes,
        error_404: Route::get_404(),
    }
}

// * Route Factories
pub fn route(method: HttpMethod, name: &str, handler: Handler) -> Route {
    Route {
        name: name.to_string(),
        handler: handler,
        method: method,
    }
}

pub fn get(name: &str, handler: Handler) -> Route {
    route(HttpMethod::GET, name, handler)
}

pub fn post(name: &str, handler: Handler) -> Route {
    route(HttpMethod::POST, name, handler)
}

pub fn put(name: &str, handler: Handler) -> Route {
    route(HttpMethod::PUT, name, handler)
}

pub fn delete(name: &str, handler: Handler) -> Route {
    route(HttpMethod::DELETE, name, handler)
}

pub fn any(name: &str, handler: Handler) -> Route {
    route(HttpMethod::ANY, name, handler)
}
