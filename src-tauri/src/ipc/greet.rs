//! Development smoke-test command.
//!
//! [`greet`] is a minimal round-trip IPC call used during development to verify
//! that the Tauri bridge between the frontend and the Rust backend is working.
//! It has no production purpose and can be removed once the app matures.
//!
//! # Example (frontend)
//! ```js
//! const msg = await invoke("greet", { name: "Alice" });
//! // → "Hello, Alice! You've been greeted from Rust!"
//! ```

/// Return a greeting string for the given `name`.
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
