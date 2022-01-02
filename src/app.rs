use std::io::{Result, Write};
use std::net::{TcpListener, TcpStream, Shutdown};

const PORT: i32 = 5050;
const PRINT_REQUEST: bool = true;
const PRINT_RESPONSE: bool = true;

#[path = "./response.rs"]
pub mod response;

#[path = "./request.rs"]
mod request;

#[path = "./intro.rs"]
pub mod intro;

pub type Response = response::Response;
pub type ContentType = response::ContentType;
pub type Request = request::Request;
pub type RouteInfo = request::RouteInfo;
pub type Handler = fn(req: Request) -> Response;

pub struct Route {
  name: String,
  handler: Handler,
  method: request::HttpMethod
}

impl Route {
  fn get_404() -> Self {
    Self {
      name: "404".to_string(),
      handler: |req: Request| {
        response::status(404).text("Error 404".to_string())
      },
      method: request::HttpMethod::GET
    }
  }
}

pub struct Router {
  routes: Vec<Route>,
  error_404: Route
}

impl Router {
  fn find(&self, route_info: &RouteInfo) -> &Route {
    self.routes
    .iter()
    .find(|route| {
      route.name == route_info.path
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

fn serve(stream: &mut TcpStream, router: &Router) -> Result<()> {
  let req = request::get_request(stream).unwrap();

  if PRINT_REQUEST { println!("{:?}\n", req) }

  let route = router.find(&req.route);
  let handler = route.handler;

  let res = handler(req);
  send_response(stream, &res)?;

  if PRINT_RESPONSE {  println!("{:?}\n", res) }

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

pub fn start(router: Router) -> Result<()> {
  intro::display(PORT);
  listen_on(PORT, router)
}

pub fn routes(all_routes: Vec<Route>) -> Router {
  Router {
    routes: all_routes,
    error_404: Route::get_404()
  }
}

pub fn get(name: &str, handler: Handler) -> Route {
  Route {
    name: name.to_string(),
    handler: handler,
    method: request::HttpMethod::GET
  }
}