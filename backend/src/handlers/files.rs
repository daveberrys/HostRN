use axum::Json;
use serde::{Deserialize, Serialize};
use std::fs;
use rand::prelude::*;
use rand::distr::Alphanumeric;

#[derive(Serialize, Deserialize, Clone)]
pub struct Service {
    pub uuid: Option<String>,
    pub name: String,
    pub path: String,
    pub command: String,
    pub running: Option<String>,
}

pub async fn get_services() -> Json<Vec<Service>> {
    let data = fs::read_to_string("data/services.json").unwrap_or_else(|_| "[]".to_string());
    let services: Vec<Service> = serde_json::from_str(&data).unwrap();
    Json(services)
}

pub async fn save_service(Json(service): Json<Service>) -> Json<Service> {
    let uuid: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
    let mut service = service;
    service.uuid = Some(uuid);
    let services = fs::read_to_string("data/services.json").unwrap();
    let mut services: Vec<Service> = serde_json::from_str(&services).unwrap();
    services.push(service.clone());
    fs::write("data/services.json", serde_json::to_string(&services).unwrap()).unwrap();
    println!("A new service has been added:\n   {}", serde_json::to_string(&service).unwrap());
    Json(service)
}

pub async fn delete_service(Json(service): Json<Service>) -> Json<Service> {
    let services = fs::read_to_string("data/services.json").unwrap();
    let mut services: Vec<Service> = serde_json::from_str(&services).unwrap();
    services.retain(|s| s.uuid.as_deref() != service.uuid.as_deref());
    fs::write("data/services.json", serde_json::to_string(&services).unwrap()).unwrap();
    Json(service)
}

