// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  app_lib::run();
}
#[tauri::command]
fn pause_recording(state: State<RecorderState>) {
    state.pause();
}
.invoke_handler(tauri::generate_handler![
    start_recording,
    pause_recording,
    stop_recording
])

