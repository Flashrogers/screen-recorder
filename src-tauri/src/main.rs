mod recorder;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use tauri::State;
use recorder::start_pipeline;

pub struct RecorderState {
    pub is_recording: Arc<AtomicBool>,
    pub is_paused: Arc<AtomicBool>,
}

impl RecorderState {
    pub fn new() -> Self {
        Self {
            is_recording: Arc::new(AtomicBool::new(false)),
            is_paused: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&self) {
        self.is_recording.store(true, Ordering::SeqCst);
        self.is_paused.store(false, Ordering::SeqCst);
    }

    pub fn pause(&self) {
        self.is_paused.store(true, Ordering::SeqCst);
    }

    pub fn stop(&self) {
        self.is_recording.store(false, Ordering::SeqCst);
        self.is_paused.store(false, Ordering::SeqCst);
    }

    pub fn is_running(&self) -> bool {
        self.is_recording.load(Ordering::SeqCst)
    }
}

#[tauri::command]
fn start_recording(state: State<RecorderState>) {
    if state.is_running() {
        println!("Already recording");
        return;
    }

    println!("Starting recording...");
    state.start();

    let running_flag = state.is_recording.clone();
    let pause_flag = state.is_paused.clone();

    start_pipeline(running_flag, pause_flag);
}

#[tauri::command]
fn pause_recording(state: State<RecorderState>) {
    println!("Pausing recording...");
    state.pause();
}

#[tauri::command]
fn stop_recording(state: State<RecorderState>) {
    println!("Stopping recording...");
    state.stop();
}

fn main() {
    tauri::Builder::default()
        .manage(RecorderState::new())
        .invoke_handler(tauri::generate_handler![
            start_recording,
            pause_recording,
            stop_recording
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
