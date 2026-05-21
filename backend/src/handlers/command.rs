use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ServiceRequest {
    pub uuid: String,
}

pub async fn start_service(Json(request): Json<ServiceRequest>) -> Json<ServiceRequest> {
    println!("TODO: make start_service work");
    Json(ServiceRequest {
        uuid: "".to_string(),
    })
}

pub async fn stop_service(Json(request): Json<ServiceRequest>) -> Json<ServiceRequest> {
    println!("TODO: make stop_service work");
    Json(ServiceRequest {
        uuid: "".to_string(),
    })
}