use std::io::{Result};

mod app;

fn main() -> Result<()> {
    let router = app::routes(vec![
        app::get("index", |req: app::Request| {
            app::response::status(200).text("This is the home page".to_string())
        }),
        app::get("test", |req: app::Request| {
            let test_text = format!("Welcome to the test route!\nHere is some info about your request: {:?}", req);
            let ok_res = app::response::status(200);
            ok_res.text(test_text)
        }),
        app::get("emoji", |req: app::Request| {
            // It works with UTF-8
            let emojis = "😃 😂 😊 😍 😜 😎 ".to_string();
            app::response::status(200).text(emojis)
        }),
        app::get("add", |req: app::Request| {
            // It works with UTF-8
            let zero = "0".to_string();
            let get_int = |name: &str| -> i32 {
                let val = req.get(name).unwrap_or(&zero);
                val.parse::<i32>().unwrap_or(0)
            };

            let a = get_int("a");
            let b = get_int("b");

            let output = format!("{} + {} = {}", a, b, a + b);

            app::response::status(200).text(output)
        })
    ]);

    app::start(router)
}

// app::status(200)::text("Hello")
// app::status(400)::json("Apple");
// app::status(231)::html("HI!");