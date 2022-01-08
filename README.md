# Dungeness

A bare bones web server in Rust. It is meant to occupy the same domain as Flask or Express. Contributions welcome!

## Docs

### Getting Started:

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

## Documentation

(Dungeness is still in beta and the public API is subject to change.)
