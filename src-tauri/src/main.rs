// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// Binary entry point — delegates immediately to [`kakebo_lib::run`].
///
/// All application logic lives in the library crate (`lib.rs`) so that it can
/// be tested and referenced without spinning up a full Tauri process.
fn main() {
    kakebo_lib::run()
}
