use axum::Json;
use std::process::{Command, Child};
use super::files::Service;

use axum::extract::State;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

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
    map.insert(uuid, child);

    Json(Service {
        uuid: service.uuid.clone(),
        name: service.name.clone(),
        path: service.path.clone(),
        command: service.command.clone(),
        running: Some(true),
    })
}

pub async fn stop_service(
    State(state): State<Arc<Mutex<HashMap<String, Child>>>>,
    Json(service): Json<Service>
) -> Json<Service> {
    let uuid = service.uuid.clone().unwrap();
    let mut map = state.lock().unwrap();
    
    if let Some(mut child) = map.remove(&uuid) {
        #[cfg(windows)] {
            let pid = child.id();
            let _ = Command::new("taskkill")
                .args(["/F", "/T", "/PID", &pid.to_string()])
                .status();
        }

        #[cfg(unix)] {
            let pid = child.id();
            unsafe { libc::kill(-(pid as i32), libc::SIGTERM); }
        }

        let _ = child.kill();
    }
    
    Json(Service {
        uuid: service.uuid.clone(),
        name: service.name.clone(),
        path: service.path.clone(),
        command: service.command.clone(),
        running: Some(false),
    })
}

pub async fn find_command(target_uuid: String) -> String {
    let data = std::fs::read_to_string("data/services.json").unwrap();
    let services: Vec<Service> = serde_json::from_str(&data).unwrap();
    let found_command = services.into_iter()
        .find(|s| s.uuid.as_deref() == Some(&target_uuid))
        .map(|s| s.command.clone())
        .unwrap_or_else(|| "Command was not found.".to_string());
    found_command
}