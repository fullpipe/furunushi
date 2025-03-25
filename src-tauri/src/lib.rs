mod drone;
mod pd;

use anyhow::Result;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    let detector_state = pd::DetectorState::new();
    let controls_receiver = detector_state.controls_receiver.clone();
    let data_sender = detector_state.data_sender.clone();

    let drone_state = drone::state::DroneState::new();
    let drone_controls_receiver = drone_state.controls_receiver.clone();

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(detector_state);
            app.manage(drone_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            pd::commands::pd_start,
            pd::commands::pd_pause,
            pd::commands::pd_base,
            drone::commands::drone_play,
            drone::commands::drone_pause,
            drone::commands::drone_volume,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    std::thread::spawn(move || {
        pd::init(controls_receiver, data_sender);
    });
    std::thread::spawn(move || {
        drone::init(drone_controls_receiver);
    });

    app.run(move |app_handle, event| match event {
        tauri::RunEvent::Ready => {
            tauri::async_runtime::spawn(pd::commands::tunner_emiter(app_handle.clone()));
        }
        tauri::RunEvent::ExitRequested { api, .. } => {
            pd::commands::stop_detector(app_handle.clone());
            drone::commands::stop_drone(app_handle.clone());
        }
        _ => {}
    });

    Ok(())
}
