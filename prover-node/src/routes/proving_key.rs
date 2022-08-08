use rocket::serde::{json::Json, Serialize};
use rocket::fs::{relative};
use rocket::Data;
use rocket::data::ToByteUnit;
use rocket_okapi::openapi;
use rocket_okapi::okapi::schemars::JsonSchema;
use std::path::{Path};
use prover_node::utils::responses::{ApiResult, ApiError};


#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct ProvingKeyResponseBody {
    message: String,
}

#[openapi]
#[post("/<hash>/proving-key", data = "<upload>")]
pub async fn post_proving_key(hash: &str, upload: Data<'_>) -> ApiResult<ProvingKeyResponseBody> {
    // create a hash for the .zok code, if the hash exists return err
    let path = Path::new(relative!("out")).join(&hash);
    if !path.is_dir() {
        return Err(ApiError::ResourceNotFound(format!("Proof {} have not been registered", hash)))
    }

    let permanent_location = path.join("proving.key");
    upload.open(200.megabytes()).into_file(permanent_location).await
        .map_err(|e| ApiError::InternalError(e.to_string()))?;

    Ok(Json(
        ProvingKeyResponseBody {
            message: format!("proving key recorded for proof {}", hash)
        }
    ))
}