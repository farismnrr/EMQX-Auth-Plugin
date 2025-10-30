use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

use crate::services::user_create_service::UserService;
use crate::utils::escape_json_str;

#[derive(serde::Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}

pub struct AppState {
    pub service: Arc<UserService>,
}

pub async fn create_user_handler(
    data: web::Data<AppState>,
    body: web::Json<CreateUserRequest>,
) -> impl Responder {
    let response = match data.service.create_user(&body.username, &body.password) {
        Ok(_) => build_user_create_response(Some(&body.username), true, "User created successfully"),
        Err(e) => build_user_create_response(None, false, &e),
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .body(response)
}

fn build_user_create_response(username: Option<&str>, is_success: bool, message: &str) -> String {
    // Use shared utility to escape JSON string values.
    let msg = escape_json_str(message);
    let data = if let Some(u) = username {
        format!(r#"{{"username":"{}"}}"#, escape_json_str(u))
    } else {
        "null".to_string()
    };

    format!(r#"{{"success":{},"message":"{}","data":{}}}"#, is_success, msg, data)
}
