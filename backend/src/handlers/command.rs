use axum::Json;
use std::process::{Command, Child};
use super::files::Service;

use axum::extract::{State, Query};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(serde::Deserialize)]
pub struct CheckParams {
    pub uuid: String,
}

#[cfg(unix)]
use std::os::unix::process::CommandExt;

pub async fn start_service(
    State(state): State<Arc<Mutex<HashMap<String, Child>>>>,
    Json(service): Json<Service>
) -> Json<Service> {
    let uuid = service.uuid.clone().unwrap();
    let command_str = find_command(uuid.clone()).await;
    
    #[cfg(windows)]
    let mut cmd = Command::new("cmd");
    #[cfg(windows)]
    cmd.args(["/C", &command_str]);

    #[cfg(unix)]
    let mut cmd = Command::new("sh");
    #[cfg(unix)]
    cmd.args(["-c", &command_str]);

    cmd.current_dir(&service.path);

    #[cfg(unix)] unsafe {
        cmd.pre_exec(|| {
            { libc::setpgid(0, 0) };
            Ok(())
        });
    }

    let child = cmd.spawn().expect("failed to execute process");

    let mut map = state.lock().unwrap();
    map.insert(uuid.clone(), child);

    write_running(uuid, true);
    Json(service)
}

pub async fn stop_service(
    State(state): State<Arc<Mutex<HashMap<String, Child>>>>,
    Json(service): Json<Service>
) -> Json<Service> {
    let uuid = service.uuid.clone().unwrap();
    let mut map = state.lock().unwrap();
    
    if let Some(mut child) = map.remove(&uuid) {
        let pid = child.id();
        #[cfg(windows)] {
            let _ = Command::new("taskkill")
                .args(["/F", "/T", "/PID", &pid.to_string()])
                .status();
        }

        #[cfg(unix)] {
            unsafe { libc::kill(-(pid as i32), libc::SIGTERM); }
        }

        let _ = child.kill();
    }
    
    write_running(uuid, false);
    Json(service)
}

pub async fn check_service(
    State(state): State<Arc<Mutex<HashMap<String, Child>>>>,
    Query(params): Query<CheckParams>
) -> Json<bool> {
    let mut map = state.lock().unwrap();
    
    let is_running = if let Some(child) = map.get_mut(&params.uuid) {
        match child.try_wait() {
            Ok(None) => true,
            _ => false,
        }
    } else {
        false
    };

    write_running(params.uuid, is_running);
    
    Json(is_running)
}

async fn find_command(target_uuid: String) -> String {
    let data = std::fs::read_to_string("data/services.json").unwrap();
    let services: Vec<Service> = serde_json::from_str(&data).unwrap();
    let found_command = services.into_iter()
        .find(|s| s.uuid.as_deref() == Some(&target_uuid))
        .map(|s| s.command.clone())
        .unwrap_or_else(|| "Command was not found.".to_string());
    found_command
}

fn write_running(target_uuid: String, running: bool) {
    let mut data = std::fs::read_to_string("data/services.json").unwrap();
    let mut services: Vec<Service> = serde_json::from_str(&data).unwrap();
    if let Some(service) = services.iter_mut().find(|s| s.uuid.as_deref() == Some(&target_uuid)) {
        service.running = Some(running);
    }
    data = serde_json::to_string(&services).unwrap();
    std::fs::write("data/services.json", data).unwrap();
}