use std::collections::HashMap;
use std::io::Result;
mod app;

fn adder_route(req: app::Request) -> app::Response {
    // Prioritizes query over body
    let use_query = req.query_exists("a");

    let zero = "0".to_string();
    let get_int = |name: &str| -> i32 {
        let val = req.get(name).unwrap_or(&zero);
        val.parse::<i32>().unwrap_or(0)
    };

    let parts: Vec<&str> = if !use_query {
        req.raw_body.split("+").collect()
    } else {
        vec![]
    };
    let get_part = |index: usize| parts.get(index).unwrap().parse::<i32>().unwrap_or(0);

    let get_num = |name: &str, index: usize| {
        if use_query {
            get_int(name)
        } else {
            get_part(index)
        }
    };
    let a = get_num("a", 0);
    let b = get_num("b", 1);

    let output = format!("{} + {} = {}", a, b, a + b);

    app::res().status(200).text(output)
}

fn server() -> Result<()> {
    // A router is a vector of routes
    let router = app::routes(vec![
        // Methods have their own functions (get, post, put, etc.)
        app::get("index", |req| {
            let unknown = "Unknown".to_string();
            let ua = req.headers.get("User-Agent").unwrap_or(&unknown);
            let text = format!("This is the home page\nUser Agent: {}", ua);
            app::res().status(200).text(text)
        }),
        // This is the verbose way
        app::route(
            app::HttpMethod::GET,
            "test",
            |req: app::Request| -> app::Response {
                let test_text = format!(
                    "Welcome to the test route!\nHere is some info about your request:\n{:?}",
                    req
                )
                .replace(", ", ",\n")
                .replace("{ ", "{ \n")
                .replace("},", "\n},");

                let ok_res = app::res().status(200);
                ok_res.text(test_text)
            },
        ),
        app::get("emoji", |_| {
            // It works with UTF-8
            let emojis = "<h1>These are my emojis</h1>\n 😃 😂 😊 😍 😜 😎 ".to_string();
            app::res().status(200).html_body(emojis)
        }),
        // External functions can be used
        app::any("add", adder_route),
        app::post("post-test", |req| {
            let res_text = format!(
                "You sent this data using post:\nForm: {:?}\nRaw Body{:?}",
                req.form, req.raw_body
            );

            let mut headers: HashMap<String, String> = HashMap::new();
            headers.insert("x-served-with".to_string(), "Dungeness".to_string());

            app::res().status(200).text(res_text).headers(headers)
        }),
        app::any("any-test", |req| {
            let text = format!(
                "You sent a request to 'any-test' using method {}.\nQuery: {:?}, Body: {:?}",
                req.method, req.route.query, req.raw_body
            );

            app::res().status(200).text(text)
        }),
        app::get("greet", |req| {
            // Access the search parameter "name"
            match req.get("name") {
                None => app::res()
                    .status(401)
                    .text("You must add a name to use this route!".to_string()),
                Some(name) => app::res().status(200).text(format!("Hello, {}!", name)),
            }
        }),
    ]);

    app::start(router)
}

fn main() -> Result<()> {
    server()
}
