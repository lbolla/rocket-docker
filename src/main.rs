#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::{Json, Value};
use rocket::http::{RawStr, Status};
use rocket::response::{Flash, Redirect};
use rocket::response::status::Custom;

#[get("/")]
fn index() -> Json<Value> {
    Json(json!({
        "status": "OK"
    }))
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
