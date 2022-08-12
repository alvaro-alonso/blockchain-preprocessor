#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_okapi::{openapi, openapi_get_routes, JsonSchema};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

mod routes;
use routes::*;


#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct Task {
    message: String,
}

#[cfg(debug_assertions)] #[openapi]
#[get("/", format = "json")]
fn index() -> Json<Task> {
    Json(Task {
        message: String::from("Hello, world!"),
    })
}

fn get_docs() -> SwaggerUIConfig {

    SwaggerUIConfig {
        url: "/openapi.json".to_string(),
        ..Default::default()
    }
}

#[launch]
fn rocket() -> _ {
    // openapi only on debug mode available
    match cfg!(debug_assertions) {
        true => rocket::build()
            .mount(
                "/",
                openapi_get_routes![
                    index, 
                    compile::post_compile_zokrates, 
                    generate_proof::post_generate_proof,
                    compute_witness::post_witness,
                    proving_key::post_proving_key,
                    compute_generate_proof::post_compute_generate_proof,
                ],
            )
            .mount("/docs", make_swagger_ui(&get_docs())),
        false => rocket::build()
            .mount(
                "/",
                routes![
                    index, 
                    compile::post_compile_zokrates, 
                    generate_proof::post_generate_proof,
                    compute_witness::post_witness,
                    proving_key::post_proving_key,
                    compute_generate_proof::post_compute_generate_proof,
                ],
            )
    }       
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
