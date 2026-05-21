mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tokio;
use std::path::Path;
use std::fs;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tower_http::cors;

// handlers
use handlers::files::{get_services, save_service, delete_service};
use handlers::command::{start_service, stop_service};

// === APP ROUTER SETUP ===
fn app() -> Router {
    let state = Arc::new(Mutex::new(HashMap::new()));
    let cors = cors::CorsLayer::new()
        .allow_origin(cors::Any);
    
    Router::new()
        .route("/start-service", post(start_service))
        .route("/get-services", get(get_services))
        .route("/save-service", post(save_service))
        .route("/stop-service", post(stop_service))
        .route("/delete-service", post(delete_service))
        .with_state(state.clone())
        .layer(cors)
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

    let mut services = fs::read_to_string("data/services.json").unwrap();
    services = services.replace("\"running\": true", "\"running\": false");
    fs::write("data/services.json", services).unwrap();
    
    axum::serve(listener, app()).await.unwrap();
}