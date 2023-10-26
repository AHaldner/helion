#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, Docker!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![hello])
}