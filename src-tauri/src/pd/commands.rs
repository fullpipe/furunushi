use std::sync::{Arc, Mutex};

use super::{DetectorState, Note};
use crossbeam::channel::Receiver;
use log::info;
use tauri::{AppHandle, Emitter, Manager, State};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub fn pd_pause(state: State<'_, DetectorState>) {
    state.controls_sender.send(super::Control::Pause).unwrap();
}

#[tauri::command]
pub fn pd_start(state: State<'_, DetectorState>) {
    state.controls_sender.send(super::Control::Start).unwrap();
}

#[tauri::command]
pub fn pd_base(f: f32, state: State<'_, DetectorState>) {
    state.controls_sender.send(super::Control::Base(f)).unwrap();
}

pub async fn tunner_emiter(app_handle: AppHandle) {
    let r = app_handle.state::<DetectorState>();
    while let Ok(note) = r.data_receiver.recv() {
        info!("note: {:?}", note);
        app_handle.emit("tuner::note", note).unwrap();
    }
    info!("DONE");
}

pub fn stop_detector(app_handle: AppHandle) {
    let state = app_handle.state::<DetectorState>();
    state.controls_sender.send(super::Control::Stop).unwrap();
}
