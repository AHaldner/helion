#[macro_use]
extern crate rocket;
extern crate clap;

use clap::Parser;

#[derive(Parser)]
struct CliArgs {
    #[clap(long, default_value = "default value")]
    input: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, Docker!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[get("/cli")]
fn cli() -> String {
    let args = CliArgs::parse();
    format!("Hello, {}!", args.input)
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", routes![index, hello, cli])
}
