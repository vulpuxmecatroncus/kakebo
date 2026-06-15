//! Session lifecycle commands.
//!
//! Sessions are the primary authentication token used after login.  Each session
//! is identified by a UUID and tied to a `user_id`.  The frontend is responsible
//! for generating the `session_id` and passing an RFC 3339 expiry timestamp.
//!
//! ## Typical flow
//!
//! ```text
//! 1. login_with_password / login_with_biometrics   (security module)
//! 2. create_session(session_id, user_id, expires_at, ...)
//! 3. validate_session(session_id)   ← checked on each sensitive action
//! 4. revoke_session(session_id)     ← on logout
//!    revoke_all_sessions_for_user(user_id)  ← on "log out everywhere"
//! ```
//!
//! Session records are persisted via
//! `kakebo_data_model::repositories::identity::SessionRepository`.

use crate::DbState;
use kakebo_data_model::dto::{UserSessionDto, CreateSessionDto};
use kakebo_data_model::repositories::identity::SessionRepository;
use tauri::State;

/// Return `true` if `session_id` exists in the database and has not expired.
#[tauri::command]
pub async fn validate_session(
    state: State<'_, DbState>,
    session_id: String,
) -> Result<bool, String> {
    let pool = state.get_pool()?;
    let mut conn = pool.get().map_err(|e| e.to_string())?;
    tokio::task::spawn_blocking(move || {
        SessionRepository::validate(&mut conn, &session_id)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Persist a new session record and return it as a [`UserSessionDto`].
#[tauri::command]
pub async fn create_session(
    state: State<'_, DbState>,
    dto: CreateSessionDto,
) -> Result<UserSessionDto, String> {
    let pool = state.get_pool()?;
    let mut conn = pool.get().map_err(|e| e.to_string())?;
    tokio::task::spawn_blocking(move || {
        SessionRepository::create(&mut conn, &dto)
            .map(UserSessionDto::from)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Invalidate a single session by `session_id`.
///
/// Returns the number of rows deleted (0 if the session was not found).
#[tauri::command]
pub async fn revoke_session(
    state: State<'_, DbState>,
    session_id: String,
) -> Result<usize, String> {
    let pool = state.get_pool()?;
    let mut conn = pool.get().map_err(|e| e.to_string())?;
    tokio::task::spawn_blocking(move || {
        SessionRepository::revoke(&mut conn, &session_id)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Invalidate **all** sessions belonging to `user_id` (e.g. "log out everywhere").
///
/// Returns the number of rows deleted.
#[tauri::command]
pub async fn revoke_all_sessions_for_user(
    state: State<'_, DbState>,
    user_id: String,
) -> Result<usize, String> {
    let pool = state.get_pool()?;
    let mut conn = pool.get().map_err(|e| e.to_string())?;
    tokio::task::spawn_blocking(move || {
        SessionRepository::revoke_all_for_user(&mut conn, &user_id)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}
