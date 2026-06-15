//! IPC command modules exposed to the Tauri frontend.
//!
//! Each sub-module groups related [`tauri::command`] functions and is
//! registered in [`crate::run`] via `tauri::generate_handler!`.
//!
//! | Module            | Responsibility                                      |
//! |-------------------|-----------------------------------------------------|
//! | [`session`]       | Create, validate, and revoke user sessions          |
//! | [`security`]      | Local password setup, login, and biometric auth     |
//! | [`mfa`]           | TOTP-based multi-factor authentication              |
//! | [`access_control`]| Per-account read / write / admin permission checks  |

pub mod session;
pub mod access_control;
pub mod security;
pub mod mfa;
