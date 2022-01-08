#[path = "./response.rs"]
mod res;

#[path = "./intro.rs"]
mod intro;

#[path = "./constants.rs"]
mod constants;

#[path = "./request.rs"]
mod request;

#[path = "./route.rs"]
mod route;

pub type Response = res::Response;
pub type ContentType = res::ContentType;
pub type HttpMethod = request::HttpMethod;
pub type Request = request::Request;
pub type RouteInfo = request::RouteInfo;
pub type Handler = fn(req: Request) -> Response;
pub type Route = route::Route;
pub type Router = route::Router;
