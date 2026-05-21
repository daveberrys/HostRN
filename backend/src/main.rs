use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio;
use std::path::Path;
use std::fs;
use rand::prelude::*;
use rand::distr::Alphanumeric;

// === DATA TYPES ===
#[derive(Serialize, Deserialize, Clone)]
struct Service {
    uuid: Option<String>,
    name: String,
    path: String,
    command: String,
}

#[derive(Serialize, Deserialize)]
struct ServiceRequest {
    uuid: String,
}

// === HANDLERS ===
// COMMAND HANDLERS
async fn start_service(Json(request): Json<ServiceRequest>) -> Json<ServiceRequest> {
    println!("TODO: make start_service work");
    Json(ServiceRequest {
        uuid: "".to_string(),
    })
}

async fn stop_service(Json(request): Json<ServiceRequest>) -> Json<ServiceRequest> {
    println!("TODO: make stop_service work");
    Json(ServiceRequest {
        uuid: "".to_string(),
    })
}

async fn delete_service(Json(request): Json<ServiceRequest>) -> Json<ServiceRequest> {
    println!("TODO: make delete_service work");
    Json(ServiceRequest {
        uuid: "".to_string(),
    })
}

// FILE HANDLERS
async fn get_services() -> Json<Vec<Service>> {
    let data = fs::read_to_string("data/services.json").unwrap_or_else(|_| "[]".to_string());
    let services: Vec<Service> = serde_json::from_str(&data).unwrap();
    Json(services)
}

async fn save_service(Json(service): Json<Service>) -> Json<Service> {
    let uuid: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(32)
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

// === APP ROUTER SETUP ===
fn app() -> Router {
    Router::new()
        .route("/start-service", post(start_service))
        .route("/get-services", get(get_services))
        .route("/save-service", post(save_service))
        .route("/stop-service", post(stop_service))
        .route("/delete-service", post(delete_service))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {}", addr);

    if !Path::new("data/services.json").exists() {
        fs::create_dir_all("data").unwrap();
        fs::write("data/services.json", "[]").unwrap();
    }
    
    axum::serve(listener, app()).await.unwrap();
}