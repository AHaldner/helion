#[macro_use]
extern crate rocket;
use std::io::{self, Write};
use std::thread;

#[get("/")]
fn index() -> &'static str {
    "Rust CMS API"
}

fn main() {
    // Spawn a new thread for the CLI
    thread::spawn(move || {
        cli_task();
    });

    // Start the Rocket web server on the main thread
    rocket::build().mount("/", routes![index]).launch();
}

fn cli_task() {
    loop {
        print!("Enter command: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Process the input
        println!("You entered: {}", input.trim());
        // Add logic to interact with shared resources or perform actions
    }
}
