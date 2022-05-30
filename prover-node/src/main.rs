#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, Deserialize, json::Json};

#[cfg(test)] mod tests;
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

#[post("/", data = "<task>", format = "json")]
fn new(task: Json<Task>) -> Json<Task> { 
    task
 }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, new])
        .mount("/generate-proof", routes![post_generate_proof])
}