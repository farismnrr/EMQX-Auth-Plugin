use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;
use crate::services::soft_delete_mqtt_service::SoftDeleteMqttService;
use crate::services::service_error::MqttServiceError;
use crate::dtos::response_dto::ResponseDTO;
use crate::dtos::mqtt_dto::DeleteMqttDTO;
use crate::handler::handler_error::AppError;

pub struct AppState {
    pub soft_delete_mqtt_service: Arc<SoftDeleteMqttService>,
}

pub async fn soft_delete_mqtt(
    data: web::Data<AppState>,
    params: web::Path<DeleteMqttDTO>
) -> impl Responder {
    let username = &params.username;
    match data.soft_delete_mqtt_service.soft_delete_mqtt(username) {
        Ok(_) => HttpResponse::Ok().json(ResponseDTO::<()> {
            success: true,
            message: "User mqtt deleted successfully",
            data: None,
            result: None
        }),
        Err(e) => match &e {
            MqttServiceError::BadRequest(validation_errors) => {
                e.to_http_response_with_details(Some(validation_errors))
            }
            _ => e.to_http_response_with_details(None::<String>),
        },
    }
}