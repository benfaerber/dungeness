use std::io::{Result};

mod app;

fn add_route(req: app::Request) -> app::Response {
    let zero = "0".to_string();
    let get_int = |name: &str| -> i32 {
        let val = req.get(name).unwrap_or(&zero);
        val.parse::<i32>().unwrap_or(0)
    };

    let a = get_int("a");
    let b = get_int("b");

    let output = format!("{} + {} = {}", a, b, a + b);

    app::res::status(200).text(output)
}

fn main() -> Result<()> {
    // A router is a vector of routes
    let router = app::routes(vec![

        // Methods have their own functions (get, post, put, etc.)
        app::get("index", |req| {
            let unknown = "Unknown".to_string();
            let ua = req.headers.get("User-Agent").unwrap_or(&unknown);
            let text = format!("This is the home page\nUser Agent: {}", ua);
            app::res::status(200).text(text)
        }),

        // This is the verbose way
        app::route(app::HttpMethod::GET, "test", |req: app::Request| -> app::Response {
            let test_text = format!("Welcome to the test route!\nHere is some info about your request:\n{:?}", req);
            let ok_res = app::res::status(200);
            ok_res.text(test_text)
        }),

        app::get("emoji", |_| {
            // It works with UTF-8
            let emojis = "<h1>These are my emojis</h1>\n 😃 😂 😊 😍 😜 😎 ".to_string();
            app::res::status(200).html_body(emojis)
        }),

        // External functions can be used
        app::get("add", add_route)
    ]);

    app::start(router)
}