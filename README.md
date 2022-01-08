# Dungeness

A bare bones web server in Rust. It is meant to occupy the same domain as Flask or Express. Contributions welcome!

## Documentation

(Dungeness is still in beta and the public API is subject to change.)

### Create Router

```

app::routes(all_routes: Vec<Route>) -> Router

```

Takes a vector of routes and returns a router.
This allows you to add routes to your application in a functional way.

## Creating Routes

All routes have a method, a name and a handler. The name is the path the route can be request on. A handler is a function with the following signature:
`fn(Request) -> Response`

Routes can be created using the various route factories found in the `app` namespace.

### GET Route

```

app::get(name: &str, handler: Handler) -> Route

```

Creates a new route using HTTP GET method. A request struct is passed into the handler which gives you access to request headers and query parameters.

### POST Route

```

app::post(name: &str, handler: Handler) -> Route

```

Creates a new route using HTTP POST method. A request struct is passed into the handler which gives you access to request headers, query parameters, and response body.

### PUT Route

```

app::put(name: &str, handler: Handler) -> Route

```

Creates a new route using HTTP PUT method. A request struct is passed into the handler which gives you access to request headers, query parameters, and response body.

### Any Route

```

app::any(name: &str, handler: Handler) -> Route

```

Creates a new route that accepts any HTTP Method. The method can be detected using the request struct. One thing to take into consideration is if the route is accessed via the GET method, query parameters will be passed in instead of a response body.

### Generic Route

```

app::route(method: HttpMethod, name: &str, handler: Handler) -> Route

```

This works the same as the other route factories, the only difference is you are able to pass in the method directly.

# Getting Started:

Here is a simple example:

```
use std::io::{Result};
mod app;

fn main() -> Result<()> {
    // First we must create a router.
    // This will give our web app context
    //
    // A router is a vector of routes
    let router = app::routes(vec![

        // Our first route will be looked up when a user access '/' on the server
        // It will only return a response for a GET request
        // Normally, a request struct is passed into the route handler
        // In this case, we don't need this so it is replaced with an underscore
        app::get("index", |_| {
            app::res::status(200).text("Welcome to Dungeness!".to_string())
        }),

        app::get("greet", |req| {
            let failure = app::res::status(401);
            let success = app::res::status(200);

            // Access the search parameter "name"
            match req.get("name") {
              None => {
                failure.text("You must add a name to use this route!".to_string())
              },
              Some(name) => {
                success.text(format!("Hello, {}!", name))
              }
            }
        })

    ]);

    app::start(router)
}
```
