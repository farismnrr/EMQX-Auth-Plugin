use actix_web::{http::StatusCode, HttpResponse};

use crate::dtos::response_dto::ErrorResponseDTO;

pub trait AppError {
    fn status_code(&self) -> StatusCode;
    fn message(&self) -> String;

    fn default_http_response(&self) -> HttpResponse {
        let res = ErrorResponseDTO {
            success: false,
            message: &self.message(),
            details: None::<()>,
        };
        HttpResponse::build(self.status_code()).json(res)
    }

    fn http_response_with_details<T>(&self, details: T) -> HttpResponse
    where
        T: serde::Serialize,
    {
        let res = ErrorResponseDTO {
            success: false,
            message: &self.message(),
            details: Some(details),
        };
        HttpResponse::build(self.status_code()).json(res)
    }
}
