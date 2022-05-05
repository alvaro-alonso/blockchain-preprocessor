#[macro_use] extern crate rocket;

use rocket::serde::json::{Value, json};

#[cfg(test)] mod tests;


#[get("/", format = "json")]
fn index() -> Value {
    json!({ "message": "Hello, world!" })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}