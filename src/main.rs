#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::http::{RawStr, Status};
use rocket::response::status::Custom;
use rocket::response::{Flash, Redirect};
use rocket_contrib::json::JsonValue;

#[get("/")]
fn index() -> JsonValue {
    json!({
        "status": "OK"
    })
}

#[get("/hello/<name>")]
fn hello(name: &RawStr) -> Result<String, Custom<Option<String>>> {
    let decoded = name.url_decode();
    match decoded {
        Ok(name) => Ok(format!("hello {}", name)),
        Err(_) => Err(Custom(Status::BadRequest, None)),
    }
}

#[get("/flash")]
fn flash() -> Flash<Redirect> {
    Flash::warning(Redirect::to("/"), "hurray")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, hello, flash])
        .launch();
}
