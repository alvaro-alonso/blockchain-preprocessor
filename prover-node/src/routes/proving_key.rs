use rocket::serde::Serialize;
use rocket::fs::{relative};
use rocket::http::Status;
use rocket::Data;
use rocket::data::ToByteUnit;
use std::path::{Path};
use prover_node::utils::responses::{ApiResult, ApiResponse, ApiError};


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ProvingKeyResponseBody {
    message: String,
}

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

    Ok(ApiResponse {
        response: ProvingKeyResponseBody {
            message: format!("proving key recorded for proof {}", hash)
        },
        status: Status::Created,
    })
}