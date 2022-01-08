pub fn display(port: i32) {
  let title = |name: &str| println!("{}\n{}", name, "-".repeat(name.len()));
  title("Dungeness Web Server");

  println!("Listening on http://localhost:{}", port);
}
