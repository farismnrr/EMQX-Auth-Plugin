use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

use crate::services::create_user_service::CreateUserService;
use crate::dtos::user_dto::CreateUserDTO;
use crate::dtos::response_dto::ResponseDTO;
use crate::utils::app_error::AppError;

// AppState to hold services
pub struct AppState {
    pub create_user_service: Arc<CreateUserService>,
}

pub async fn create_user_handler(
    data: web::Data<AppState>,
) -> impl Responder {
    match data.create_user_service.create_user() {
        Ok((username, password)) => {
            let dto = CreateUserDTO { username, password };
            // Return the concrete DTO directly; Actix/Serde will serialize it.
            HttpResponse::Ok().json(ResponseDTO {
                success: true,
                message: "User created successfully",
                data: Some(dto),
            })
        },
        Err(e) => e.default_http_response(),
    }
}