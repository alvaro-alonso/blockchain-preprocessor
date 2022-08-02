use rocket::serde::{json::Json, Serialize};
use rocket::{response, response::{Response, Responder}};
use rocket::http::{ContentType, Status};
use rocket::request::Request;


pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;

#[derive(Debug)]
pub struct ApiResponse<T> {
    pub response: T,
    pub status: Status,
}

impl<'r, 'o: 'r, T: Serialize> Responder<'r, 'o> for ApiResponse<T> {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        Response::build_from(Json(self.response).respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[derive(Debug)]
pub enum ApiError {
    ResourceAlreadyExists(String),
    ResourceNotFound(String),
    CompilationError(String),
    InternalError(String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    pub error_message: String,
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        // log `self` to your favored error tracker, e.g.
        // sentry::capture_error(&self);
        let mut res = Json(ErrorResponse {
			error_message: format!("{:?}", self).to_string()
		}).respond_to(req)?;

        match self {
            ApiError::CompilationError(_) => res.set_status(Status::BadRequest),
            ApiError::ResourceNotFound(_) => res.set_status(Status::NotFound),
            ApiError::ResourceAlreadyExists(_) => res.set_status(Status::Conflict),
            _ => res.set_status(Status::InternalServerError),
        };
        Ok(res)
    }
}
