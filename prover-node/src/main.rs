#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;

#if cfg!(feature = "openapi") {
    use rocket_okapi::openapi_get_routes;
    use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
}


use prover_node::utils::config::AppConfig;

mod routes;
use routes::*;

#[cfg_attr(feature = "openapi")]
fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/openapi.json".to_string(),
        ..Default::default()
    }
}

#[launch]
fn rocket() -> _ {
    // openapi only on debug mode available
    rocket::build()
        .mount(
            "/",
            openapi_get_routes![
                health::health,
                compile::post_compile_zokrates,
                generate_proof::post_generate_proof,
                compute_witness::post_witness,
                proving_key::post_proving_key,
                compute_generate_proof::post_compute_generate_proof,
            ],
        )
        #[cfg_attr(feature = "openapi")]
        .mount("/docs", make_swagger_ui(&get_docs()))
        .attach(AdHoc::config::<AppConfig>())
}
