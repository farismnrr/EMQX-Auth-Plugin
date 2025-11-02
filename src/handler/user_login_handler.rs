use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

use crate::services::user_login_service::UserLoginService;
use crate::dtos::user_dto::{CheckUserActiveDTO, UserJwtDTO};
use crate::dtos::response_dto::ResponseDTO;
use crate::handler::handler_error::handle_user_login;

pub struct AppState {
    pub login_with_credentials_service: Arc<UserLoginService>,
}

pub async fn login_with_credentials_handler(
    data: web::Data<AppState>,
    body: web::Json<CheckUserActiveDTO>,
) -> impl Responder {
    match data.login_with_credentials_service.login_with_credentials(body.into_inner()) {
        Ok((_, token)) => {
            if token.is_empty() {
                // Credentials login - return None data
                HttpResponse::Ok().json(ResponseDTO::<()> {
                    success: true,
                    message: "User is active",
                    data: None,
                    result: Some("allow"),
                })
            } else {
                HttpResponse::Ok().json(ResponseDTO::<UserJwtDTO> {
                    success: true,
                    message: "User is active",
                    data: Some(UserJwtDTO { token }),
                    result: Some("allow"),
                })
            }
        },
        Err(e) => handle_user_login(&e),
    }
}
