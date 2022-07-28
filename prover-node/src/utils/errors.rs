use rocket::http::Status;
use rocket::{response, response::Responder};
use rocket::request::Request;
use rocket::serde::{json::Json, Serialize};


#[derive(Debug)]
pub enum ApiError {
    ProofAlreadyExists(String),
    ProofNotFound(String),
    CompilationError(String),
    InternalError(String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    pub error_message: String,
}

pub type ApiResult<T> = Result<Json<T>, ApiError>;


impl<'r, 'o: 'r> Responder<'r, 'o> for ApiError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        // log `self` to your favored error tracker, e.g.
        // sentry::capture_error(&self);
        let mut res = Json(ErrorResponse {
			error_message: format!("{:?}", self).to_string()
		}).respond_to(req)?;

        match self {
            ApiError::CompilationError(_) => res.set_status(Status::BadRequest),
            ApiError::ProofNotFound(_) => res.set_status(Status::NotFound),
            ApiError::ProofAlreadyExists(_) => res.set_status(Status::Conflict),
            _ => res.set_status(Status::InternalServerError),
        };
        Ok(res)
    }
}
