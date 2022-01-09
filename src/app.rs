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
}

pub struct Router {
  routes: Vec<Route>,
  error_404: Route,
}

impl Router {
  fn find(&self, route_info: &RouteInfo, method: HttpMethod) -> &Route {
    self
      .routes
      .iter()
      .find(|route| {
        route.name == route_info.path && (route.method == method || route.method == HttpMethod::ANY)
      })
      .unwrap_or(&self.error_404)
  }
}

fn send_response(stream: &mut TcpStream, res: &Response) -> Result<()> {
  let raw_response = res.get_raw();
  let response_bytes = raw_response.as_bytes();
  stream.write(response_bytes)?;
  Ok(())
}

fn request_printer(route: &Route, res: &Response, con: &HashMap<String, String>) {
  let timestamp = Local::now().format("%H:%M:%S");
  let req_print = format!("{} {}", route.method, route.name);
  let res_print = format!(
    "(Status: {}, Content Type: {})",
    res.status_code, res.content_type
  );

  let con_print = format!("(Peer Addr: {})", con.get("Peer-Address").unwrap());
  println!("{} - {} {} {}", timestamp, req_print, res_print, con_print);
}

fn serve(stream: &mut TcpStream, router: &Router) -> Result<()> {
  let req = request::get_request(stream)?;

  if constants::PRINT_REQUEST {
    println!("{:?}\n", req)
  }

  let route = router.find(&req.route, req.method);
  let handler = route.handler;

  let con = &req.connection.clone();
  let res = handler(req);
  send_response(stream, &res)?;

  if constants::PRINT_SERVE {
    request_printer(&route, &res, &con);
  }

  if constants::PRINT_RESPONSE {
    println!("{:?}\n", res)
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
