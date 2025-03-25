use tauri::{AppHandle, Manager, State};

use super::{state::DroneState, Drone};

#[tauri::command]
pub fn drone_play(state: State<'_, DroneState>, d: Drone) {
    state.controls_sender.send(super::Control::Play(d)).unwrap();
}
#[tauri::command]
pub fn drone_pause(state: State<'_, DroneState>) {
    state.controls_sender.send(super::Control::Pause).unwrap();
}
#[tauri::command]
pub fn drone_volume(state: State<'_, DroneState>, f: f32) {
    state
        .controls_sender
        .send(super::Control::Volume(f))
        .unwrap();
}

pub fn stop_drone(app_handle: AppHandle) {
    let state = app_handle.state::<DroneState>();
    state.controls_sender.send(super::Control::Stop).unwrap();
}
