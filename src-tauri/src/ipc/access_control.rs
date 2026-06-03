//! Per-account permission check and management commands.
//!
//! Kakebo uses a role-based access-control model where each user can hold a
//! [`PermissionLevel`] on a given account:
//!
//! | Level   | Can read | Can write | Can manage permissions |
//! |---------|----------|-----------|------------------------|
//! | Read    | ✔        |           |                        |
//! | Write   | ✔        | ✔         |                        |
//! | Admin   | ✔        | ✔         | ✔                      |
//!
//! Permission records are stored and queried via
//! `kakebo_data_model::repositories::identity::AccountRepository`.
//!
//! ## How to use
//!
//! 1. After creating an account, call [`grant_permission`] to give the owning
//!    user `Admin` level access.
//! 2. Before any sensitive read / write operation call the corresponding
//!    `check_*_access` command and reject the action if it returns `false`.
//! 3. Admins can call [`grant_permission`] / [`revoke_permission`] to share
//!    access with other users.

use crate::DbState;
use kakebo_data_model::dto::AccountPermissionDto;
use kakebo_data_model::models::PermissionLevel;
use kakebo_data_model::repositories::identity::AccountRepository;
use tauri::State;

/// Return `true` if `user_id` has at least **Read** access to `account_id`.
#[tauri::command]
pub async fn check_read_access(
    state: State<'_, DbState>,
    account_id: String,
    user_id: String,
) -> Result<bool, String> {
    let pool = state.get_pool()?;
    let mut conn = pool.get().map_err(|e| e.to_string())?;
    tokio::task::spawn_blocking(move || {
        AccountRepository::check_read_access(&mut conn, &account_id, &user_id)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Return `true` if `user_id` has at least **Write** access to `account_id`.
#[tauri::command]
pub async fn check_write_access(
    state: State<'_, DbState>,
    account_id: String,
    user_id: String,
) -> Result<bool, String> {
    let pool = state.get_pool()?;
    let mut conn = pool.get().map_err(|e| e.to_string())?;
    tokio::task::spawn_blocking(move || {
        AccountRepository::check_write_access(&mut conn, &account_id, &user_id)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Return `true` if `user_id` has **Admin** access to `account_id`.
#[tauri::command]
pub async fn check_admin_access(
    state: State<'_, DbState>,
    account_id: String,
    user_id: String,
) -> Result<bool, String> {
    let pool = state.get_pool()?;
    let mut conn = pool.get().map_err(|e| e.to_string())?;
    tokio::task::spawn_blocking(move || {
        AccountRepository::check_admin_access(&mut conn, &account_id, &user_id)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Grant `level` permission on `account_id` to `target_user_id`.
///
/// Requires that `admin_user_id` already holds **Admin** access to the account.
/// Returns the newly created [`AccountPermissionDto`].
#[tauri::command]
pub async fn grant_permission(
    state: State<'_, DbState>,
    admin_user_id: String,
    target_user_id: String,
    account_id: String,
    level: PermissionLevel,
) -> Result<AccountPermissionDto, String> {
    let pool = state.get_pool()?;
    let mut conn = pool.get().map_err(|e| e.to_string())?;
    tokio::task::spawn_blocking(move || {
        AccountRepository::grant_permission(
            &mut conn,
            &admin_user_id,
            &target_user_id,
            &account_id,
            level,
        )
        .map(AccountPermissionDto::from)
        .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Remove `target_user_id`'s permission on `account_id`.
///
/// Requires that `admin_user_id` holds **Admin** access to the account.
/// Returns the number of rows deleted (0 if no matching permission existed).
#[tauri::command]
pub async fn revoke_permission(
    state: State<'_, DbState>,
    admin_user_id: String,
    target_user_id: String,
    account_id: String,
) -> Result<usize, String> {
    let pool = state.get_pool()?;
    let mut conn = pool.get().map_err(|e| e.to_string())?;
    tokio::task::spawn_blocking(move || {
        AccountRepository::revoke_permission(
            &mut conn,
            &admin_user_id,
            &target_user_id,
            &account_id,
        )
        .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}
