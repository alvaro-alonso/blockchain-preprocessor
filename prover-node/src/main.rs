#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, Deserialize, json::Json};

mod compile;
use compile::post_compile_zokrates;
mod generate_proof;
use generate_proof::post_generate_proof;

#[derive(Serialize)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Task {
    message: String
}

#[get("/", format = "json")]
fn index() -> Json<Task> {
    Json(Task { 
        message: String::from("Hello, world!") 
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            index,
            post_compile_zokrates,
            post_generate_proof,
        ])
}