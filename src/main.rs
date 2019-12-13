#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket::http::RawStr;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/world/<place>")]
fn world(place: &RawStr) -> String {
    format!("Hello to {}!", place.as_str())
}

fn main() {
    rocket::ignite().mount("/", routes![index, world]).launch();
}
