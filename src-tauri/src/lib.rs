//! # Kakebo — Tauri backend
//!
//! This crate is the Rust backend for the Kakebo personal-finance app built with
//! [Tauri](https://tauri.app/).  It owns two responsibilities:
//!
//! 1. **Application state** — [`DbState`] wraps the SQLite connection pool and
//!    is managed by Tauri so every IPC command can reach the database.
//! 2. **IPC surface** — the [`ipc`] module exposes all Tauri commands that the
//!    frontend can invoke via `invoke(...)`.
//!
//! ## Architecture
//!
//! ```text
//! Frontend (Svelte/JS)
//!        │  invoke(command, args)
//!        ▼
//! Tauri IPC layer  ──►  src/ipc/
//!                            ├── session.rs       (session lifecycle)
//!                            ├── security.rs      (local password & biometrics)
//!                            ├── mfa.rs           (TOTP-based MFA)
//!                            └── access_control.rs (per-account permissions)
//!        │
//!        ▼
//! kakebo-data-model (Diesel + SQLite)
//! ```

extern crate alloc;

use kakebo_data_model::connection::{Connection, DbPool};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use tauri::Manager;

pub mod ipc;

/// Shared application state injected into every Tauri command via [`tauri::State`].
pub struct DbState {
    /// The r2d2 connection pool.
    pub pool: DbPool,
    /// Filesystem path to the SQLite database file.
    pub db_url: String,
}

impl DbState {
    /// Create a new `DbState` for the given database path and connection pool.
    pub fn new(db_url: String, pool: DbPool) -> Self {
        Self {
            pool,
            db_url,
        }
    }

    /// Return a clone of the inner [`DbPool`].
    pub fn get_pool(&self) -> Result<DbPool, String> {
        Ok(self.pool.clone())
    }

    /// Obtain a single pooled connection from [`Self::get_pool`].
    ///
    /// Combines `get_pool()` and `pool.get()` into one call with unified error
    /// handling, saving each command from repeating the same boilerplate.
    ///
    /// # Errors
    /// Propagates any error from [`Self::get_pool`] or from the pool's
    /// connection-checkout logic.
    pub fn get_conn(&self) -> Result<PooledConnection<ConnectionManager<Connection>>, String> {
        self.get_pool()?.get().map_err(|e| e.to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir().unwrap_or_else(|_| {
                std::env::current_dir().expect("Failed to get current directory")
            });
            let _ = std::fs::create_dir_all(&app_data_dir);
            let db_path = app_data_dir.join("kakebo.db");
            let db_url = db_path.to_string_lossy().into_owned();

            keyring::use_native_store(false).map_err(|e| e.to_string())?;

            // Initialize database pool and run migrations immediately on startup
            let pool = kakebo_data_model::connection::create_pool_from_url(&db_url).map_err(|e| e.to_string())?;
            kakebo_data_model::connection::run_migrations(&pool).map_err(|e| e.to_string())?;

            app.manage(DbState::new(db_url, pool));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ipc::session::validate_session,
            ipc::session::create_session,
            ipc::session::revoke_session,
            ipc::session::revoke_all_sessions_for_user,
            ipc::access_control::check_read_access,
            ipc::access_control::check_write_access,
            ipc::access_control::check_admin_access,
            ipc::access_control::grant_permission,
            ipc::access_control::revoke_permission,
            ipc::security::has_local_users,
            ipc::security::login_with_password,
            ipc::security::create_local_user,
            ipc::security::get_local_users,
            ipc::security::enable_biometrics,
            ipc::security::disable_biometrics,
            ipc::security::is_biometrics_available,
            ipc::security::login_with_biometrics,
            ipc::security::is_database_locked,
            ipc::security::lock_database,
            ipc::mfa::generate_mfa_setup,
            ipc::mfa::enable_mfa,
            ipc::mfa::verify_mfa,
            ipc::mfa::disable_mfa,
            ipc::mfa::is_mfa_enabled,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
