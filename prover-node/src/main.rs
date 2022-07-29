#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};

mod routes;
use routes::*;


#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Task {
    message: String,
}

#[get("/", format = "json")]
fn index() -> Json<Task> {
    Json(Task {
        message: String::from("Hello, world!"),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            index, 
            compile::post_compile_zokrates, 
            generate_proof::post_generate_proof,
            compute_witness::post_witness,
        ],
    )
}

#[cfg(test)]
mod test {
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn json_test_index() {
        let client = Client::tracked(super::rocket()).unwrap();
        let res = client.get("/").header(ContentType::JSON).dispatch();
        assert_eq!(res.status(), Status::Ok);
    }
}
