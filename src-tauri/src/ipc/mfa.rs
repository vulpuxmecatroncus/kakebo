//! TOTP-based multi-factor authentication commands.
//!
//! ## Flow
//!
//! ```text
//! 1. generate_mfa_setup(username)
//!        → (base32_secret, otpauth_url)   ← show QR code to user
//!
//! 2. enable_mfa(user_id, secret, code)
//!        → bool   ← verifies the first TOTP code, then persists the secret
//!
//! 3. verify_mfa(user_id, code)
//!        → bool   ← called on every login when MFA is active
//!
//! 4. disable_mfa(user_id) / is_mfa_enabled(user_id)
//!        → bool   ← management helpers
//! ```
//!
//! TOTP parameters: **SHA-1, 6-digit code, 30-second window** (RFC 6238 defaults,
//! compatible with Google Authenticator, Authy, etc.).
//!
//! Secrets are stored in the database as Base32-encoded strings via
//! `kakebo_data_model::repositories::identity::UserRepository`.

use kakebo_data_model::repositories::identity::UserRepository;
use kakebo_data_model::dto::EnableMfaDto;
use tauri::State;
use crate::DbState;
use kakebo_data_model::security::{generate_mfa_setup as gen_mfa_setup, verify_totp_code};

/// Run a blocking database closure on a dedicated thread pool managed by Tokio.
///
/// `spawn_blocking` is required because Diesel's synchronous API must not be
/// called from an async context directly.  All errors — both join failures and
/// closure errors — are mapped to `String` for uniform IPC error handling.
async fn spawn_db<F, T>(f: F) -> Result<T, String>
where
    F: FnOnce() -> Result<T, String> + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(f)
        .await
        .map_err(|e| e.to_string())?
}

/// Generate a new TOTP secret and provisioning URI for `username`.
///
/// Returns `(base32_secret, otpauth_url)`.  The URL can be rendered as a QR
/// code so the user can scan it with any RFC 6238-compliant authenticator app.
/// The secret must be passed back to [`enable_mfa`] along with the first code
/// to activate MFA for the account.
///
/// The secret is **not** saved here — it is only confirmed and persisted by
/// [`enable_mfa`].
#[tauri::command]
pub fn generate_mfa_setup(username: String) -> Result<(String, String), String> {
    gen_mfa_setup(username)
}

/// Confirm a TOTP code and persist the MFA secret for `user_id`.
///
/// 1. Verifies `code` against `secret` before writing anything to the database.
/// 2. Only on success stores `secret` and marks MFA as enabled for the user.
///
/// Returns `false` (without error) if the code is invalid.
#[tauri::command]
pub async fn enable_mfa(
    state: State<'_, DbState>,
    dto: EnableMfaDto,
    code: String,
) -> Result<bool, String> {
    if !verify_totp_code(&dto.secret, &code)? {
        return Ok(false);
    }

    let mut conn = state.get_conn()?;
    spawn_db(move || {
        UserRepository::enable_mfa(&mut conn, &dto)
            .map_err(|e| e.to_string())
    })
    .await?;

    Ok(true)
}

/// Verify a TOTP `code` for `user_id` during login.
///
/// If MFA is not enabled for the user this returns `true` immediately, so
/// callers do not need to check `is_mfa_enabled` separately.
///
/// Returns `false` (without error) if the code is invalid.
#[tauri::command]
pub async fn verify_mfa(
    state: State<'_, DbState>,
    user_id: String,
    code: String,
) -> Result<bool, String> {
    let mut conn = state.get_conn()?;
    let (is_enabled, secret_opt) = spawn_db(move || {
        UserRepository::get_mfa_settings(&mut conn, &user_id)
            .map_err(|e| e.to_string())
    })
    .await?;

    if !is_enabled {
        // If MFA is not enabled for the user, verification is implicitly true
        return Ok(true);
    }

    let secret = secret_opt.ok_or("MFA is enabled but no secret is stored")?;
    verify_totp_code(&secret, &code)
}

/// Disable MFA for `user_id` and remove the stored secret from the database.
#[tauri::command]
pub async fn disable_mfa(
    state: State<'_, DbState>,
    user_id: String,
) -> Result<bool, String> {
    let mut conn = state.get_conn()?;
    spawn_db(move || {
        UserRepository::disable_mfa(&mut conn, &user_id)
            .map_err(|e| e.to_string())
    })
    .await?;

    Ok(true)
}

/// Return `true` if MFA is currently enabled for `user_id`.
#[tauri::command]
pub async fn is_mfa_enabled(
    state: State<'_, DbState>,
    user_id: String,
) -> Result<bool, String> {
    let mut conn = state.get_conn()?;
    let (is_enabled, _) = spawn_db(move || {
        UserRepository::get_mfa_settings(&mut conn, &user_id)
            .map_err(|e| e.to_string())
    })
    .await?;

    Ok(is_enabled)
}
