#[macro_use]
extern crate rocket;

use inquire::Text;
use tokio::{spawn, time::sleep};
use std::time::Duration;

#[get("/")]
fn index() -> &'static str {
    "Rust CMS API"
}

fn start_server() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount("/", routes![index])
}

#[tokio::main]
async fn main() {
    let rocket = start_server();
    let server = spawn(async move {
        rocket.launch().await.unwrap();
    });

    // Wait for the server to start
    sleep(Duration::from_secs(2)).await;

    // Start your CLI here
    let name_prompt = Text::new("Please enter your name:");
    let name = name_prompt.prompt().expect("Failed to read line");
    println!("Hello, {}!", name);

    server.await.unwrap();
}