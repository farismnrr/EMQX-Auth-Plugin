use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

use crate::services::check_user_active_service::CheckUserActiveService;
use crate::dtos::user_dto::CheckUserActiveDTO;
use crate::dtos::response_dto::ResponseDTO;
use crate::handler::handler_error::handle_check_user_active_error;

pub struct AppState {
    pub check_user_active_service: Arc<CheckUserActiveService>,
}

pub async fn check_user_active_handler(
    data: web::Data<AppState>,
    body: web::Json<CheckUserActiveDTO>,
) -> impl Responder {
    match data.check_user_active_service.check_user_active(body.into_inner()) {
        Ok(_) => {
            HttpResponse::Ok().json(ResponseDTO::<()> {
                success: true,
                message: "User is active",
                data: None,
                result: Some("allow"),
            })
        },
        Err(e) => handle_check_user_active_error(&e),
    }
}
