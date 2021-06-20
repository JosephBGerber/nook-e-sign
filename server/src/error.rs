use actix_web::{ResponseError, HttpResponse};
use actix_web::http::StatusCode;
use serde::Serialize;
use log::error;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("You are forbidden to access requested endpoint.")]
    Forbidden,
    #[error("Unknown internal error occurred.")]
    Unknown,
    #[error("The request syntax is invalid.")]
    BadRequest,
}

impl Error {
    pub fn name(&self) -> String {
        match self {
            Self::Forbidden => "Forbidden".to_string(),
            Self::Unknown => "Unknown".to_string(),
            Self::BadRequest => "BadRequest".to_string(),
        }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadRequest => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}

pub fn map_sqlx_error(e: sqlx::Error) -> Error {
    error!("{:?}", e);
    Error::Unknown
}

pub fn map_multipart_error(e: actix_multipart::MultipartError) -> Error {
    error!("{:?}", e);
    Error::BadRequest
}

pub fn map_image_error(e: image::ImageError) -> Error {
    error!("{:?}", e);
    Error::BadRequest
}