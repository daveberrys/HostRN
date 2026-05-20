use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio;
use std::path::Path;
use std::fs;

// DATA TYPES
#[derive(Serialize, Deserialize, Clone)]
struct Service {
    uuid: u64,
    name: String,
    path: String,
    command: String,
}

#[derive(Serialize, Deserialize)]
struct StartServiceRequest {
    uuid: u64,
    command: String,
}

// HANDLERS
async fn start_service(Json(request): Json<StartServiceRequest>) -> Json<Service> {
    println!("TODO: actually make this work");
    Json(Service {
        uuid: request.uuid,
        name: "".to_string(),
        path: "".to_string(),
        command: request.command,
    })
}

async fn get_services() -> Json<Vec<Service>> {
    let data = fs::read_to_string("data/services.json").unwrap_or_else(|_| "[]".to_string());
    let services: Vec<Service> = serde_json::from_str(&data).unwrap();
    Json(services)
}

async fn save_service(Json(service): Json<Service>) -> Json<Service> {
    let services = fs::read_to_string("data/services.json").unwrap();
    let mut services: Vec<Service> = serde_json::from_str(&services).unwrap();
    services.push(service.clone());
    fs::write("data/services.json", serde_json::to_string(&services).unwrap()).unwrap();
    Json(service)
}

// APP ROUTER SETUP
fn app() -> Router {
    Router::new()
        .route("/start-service", post(start_service))
        .route("/get-services", get(get_services))
        .route("/save-service", post(save_service))
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