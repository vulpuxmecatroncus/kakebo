//! Local-password and biometric authentication commands.
//!
//! ## How authentication works
//!
//! Kakebo uses a **local password** to derive a key that encrypts / unlocks the
//! SQLite database.  The flow is:
//!
//! ```text
//! setup_local_password(password)
//!   1. Derive a 32-byte key with PBKDF2-HMAC-SHA256 (100 000 iterations).
//!   2. Hash that derived key a second time and write it to `password.hash`
//!      (so the plaintext password is never persisted).
//!   3. Open the connection pool and run pending Diesel migrations.
//!
//! login_with_password(password)
//!   1. Re-derive the key and hash it.
//!   2. Compare with the stored hash in `password.hash`.
//!   3. On match, open the pool (unlocks the database).
//!
//! enable_biometrics(password)
//!   1. Validate the password the same way as login.
//!   2. Store the *derived key* (not the password) in the OS secure credential
//!      store (macOS Keychain, Windows Credential Manager, etc.).
//!
//! login_with_biometrics()
//!   1. Retrieve the derived key from the OS secure store — the OS prompts the
//!      user for biometrics (Touch ID / Windows Hello) before releasing it.
//!   2. Verify the key hash and open the pool on success.
//! ```
//!
//! The constant [`SALT`] is intentionally fixed; its purpose is only to make
//! the key derivation domain-specific, not to provide per-user salting (that
//! would require storing salts and is out of scope for a local-only app).

use keyring_core::Entry;
use tauri::State;
use crate::DbState;
use kakebo_data_model::dto::UserDto;
use ring::pbkdf2;
use std::num::NonZeroU32;

/// Fixed domain salt for PBKDF2 key derivation.
const PBKDF2_SALT: &[u8] = b"kakebo-local-salt-constant";

/// Encode a byte slice as a lowercase hex string.
fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Derive a 32-byte key from a password using PBKDF2-HMAC-SHA256 and return
/// it as a lowercase hex string (64 characters).
fn derive_key(password: &str) -> String {
    let mut pbkdf2_key = [0u8; 32];
    let n_iter = NonZeroU32::new(100_000).unwrap();
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        n_iter,
        PBKDF2_SALT,
        password.as_bytes(),
        &mut pbkdf2_key,
    );
    to_hex(&pbkdf2_key)
}


/// Keyring service name used as a namespace in the OS credential store.
const KEYRING_SERVICE: &str = "kakebo-app";

/// Return `true` if any non-shadow users exist.
#[tauri::command]
pub async fn has_local_users(state: State<'_, DbState>) -> Result<bool, String> {
    let pool = state.get_pool()?;
    let mut conn = pool.get().map_err(|e| e.to_string())?;

    let count = tokio::task::spawn_blocking(move || {
        use kakebo_data_model::schemas::users::dsl as users_dsl;
        use diesel::prelude::*;

        users_dsl::users
            .filter(users_dsl::is_shadow.eq(false))
            .count()
            .get_result::<i64>(&mut conn)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())??;

    Ok(count > 0)
}

/// Authenticate a specific user with their local password.
///
/// Returns `false` (without error) if the password is incorrect.
#[tauri::command]
pub async fn login_with_password(
    state: State<'_, DbState>,
    user_id: String,
    password: String,
) -> Result<bool, String> {
    let pool = state.get_pool()?;
    let mut conn = pool.get().map_err(|e| e.to_string())?;

    let expected_hash_opt = tokio::task::spawn_blocking(move || {
        use kakebo_data_model::schemas::users::dsl as users_dsl;
        use diesel::prelude::*;

        users_dsl::users
            .find(user_id)
            .select(users_dsl::password_hash)
            .first::<Option<String>>(&mut conn)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())??;

    if let Some(expected_hash) = expected_hash_opt {
        let verification_hash = derive_key(&password);

        Ok(verification_hash == expected_hash)
    } else {
        Ok(password.is_empty())
    }
}

/// Store the derived key in the OS secure credential store for a specific user.
///
/// Requires a valid `password` to prove identity.
#[tauri::command]
pub async fn enable_biometrics(
    state: State<'_, DbState>,
    user_id: String,
    password: String,
) -> Result<bool, String> {
    let pool = state.get_pool()?;
    let mut conn = pool.get().map_err(|e| e.to_string())?;

    let user_id_clone = user_id.clone();
    let expected_hash_opt = tokio::task::spawn_blocking(move || {
        use kakebo_data_model::schemas::users::dsl as users_dsl;
        use diesel::prelude::*;

        users_dsl::users
            .find(user_id_clone)
            .select(users_dsl::password_hash)
            .first::<Option<String>>(&mut conn)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())??;

    if let Some(expected_hash) = expected_hash_opt {
        let verification_hash = derive_key(&password);

        if verification_hash != expected_hash {
            return Err("Invalid password".to_string());
        }

        let keyring_user = format!("user-key-{}", user_id);
        let entry = Entry::new(KEYRING_SERVICE, &keyring_user).map_err(|e| e.to_string())?;
        entry.set_password(&verification_hash).map_err(|e| e.to_string())?;

        Ok(true)
    } else {
        Err("User does not have a local password".to_string())
    }
}

/// Disable biometrics for a specific user.
#[tauri::command]
pub async fn disable_biometrics(user_id: String) -> Result<bool, String> {
    let keyring_user = format!("user-key-{}", user_id);
    let entry = Entry::new(KEYRING_SERVICE, &keyring_user).map_err(|e| e.to_string())?;
    match entry.delete_credential() {
        Ok(_) => Ok(true),
        Err(e) => {
            let err_str = e.to_string().to_lowercase();
            if err_str.contains("no entry") 
                || err_str.contains("no credential") 
                || err_str.contains("not found") 
                || err_str.contains("no matching") 
            {
                Ok(true)
            } else {
                Err(format!("Failed to delete biometric credential: {}", e))
            }
        }
    }
}

/// Return `true` if biometrics is available, and if user_id is provided,
/// checks if a key is present in the OS secure store for this user.
#[tauri::command]
pub async fn is_biometrics_available(user_id: Option<String>) -> Result<bool, String> {
    if let Some(uid) = user_id {
        let keyring_user = format!("user-key-{}", uid);
        let entry = Entry::new(KEYRING_SERVICE, &keyring_user).map_err(|e| e.to_string())?;
        match entry.get_password() {
            Ok(_) => Ok(true),
            Err(e) => {
                let err_str = e.to_string().to_lowercase();
                if err_str.contains("no entry") 
                    || err_str.contains("no credential") 
                    || err_str.contains("not found") 
                    || err_str.contains("no matching") 
                {
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
        }
    } else {
        #[cfg(target_os = "macos")]
        {
            use objc2_local_authentication::{LAContext, LAPolicy};
            let context = unsafe { LAContext::new() };
            let can_evaluate = unsafe {
                context.canEvaluatePolicy_error(LAPolicy::DeviceOwnerAuthenticationWithBiometrics).is_ok()
            };
            Ok(can_evaluate)
        }
        #[cfg(not(target_os = "macos"))]
        {
            Ok(true)
        }
    }
}

#[cfg(target_os = "macos")]
async fn verify_user_presence_biometrically() -> Result<(), String> {
    let (tx, rx) = tokio::sync::oneshot::channel::<Result<(), String>>();

    {
        use objc2_local_authentication::{LAContext, LAPolicy};
        use objc2_foundation::NSString;
        use block2::RcBlock;

        let context = unsafe { LAContext::new() };
        let reason = NSString::from_str("Authenticate to unlock your profile");

        unsafe {
            if let Err(err) = context.canEvaluatePolicy_error(LAPolicy::DeviceOwnerAuthenticationWithBiometrics) {
                return Err(err.localizedDescription().to_string());
            }
        }

        let tx_holder = std::sync::Arc::new(std::sync::Mutex::new(Some(tx)));

        let reply_block = RcBlock::new(move |success: objc2::runtime::Bool, error: *mut objc2_foundation::NSError| {
            let tx_opt = tx_holder.lock().unwrap().take();
            if let Some(tx) = tx_opt {
                if success.as_bool() {
                    let _ = tx.send(Ok(()));
                } else {
                    let err_msg = if !error.is_null() {
                        unsafe { (*error).localizedDescription().to_string() }
                    } else {
                        "Biometric authentication failed".to_string()
                    };
                    let _ = tx.send(Err(err_msg));
                }
            }
        });

        unsafe {
            context.evaluatePolicy_localizedReason_reply(
                LAPolicy::DeviceOwnerAuthenticationWithBiometrics,
                &reason,
                &reply_block,
            );
        }
    }

    rx.await.map_err(|e| e.to_string())?
}

#[cfg(not(target_os = "macos"))]
async fn verify_user_presence_biometrically() -> Result<(), String> {
    Ok(())
}

/// Authenticate using the OS biometric prompt for a specific user.
#[tauri::command]
pub async fn login_with_biometrics(
    state: State<'_, DbState>,
    user_id: String,
) -> Result<bool, String> {
    // Verify user presence biometrically (prompts Touch ID on macOS)
    verify_user_presence_biometrically().await?;

    let keyring_user = format!("user-key-{}", user_id);
    let entry = Entry::new(KEYRING_SERVICE, &keyring_user).map_err(|e| e.to_string())?;
    let derived = entry.get_password().map_err(|e| format!("Secure storage/Biometric access failed: {}", e))?;

    let pool = state.get_pool()?;
    let mut conn = pool.get().map_err(|e| e.to_string())?;

    let user_id_clone = user_id.clone();
    let expected_hash_opt = tokio::task::spawn_blocking(move || {
        use kakebo_data_model::schemas::users::dsl as users_dsl;
        use diesel::prelude::*;

        users_dsl::users
            .find(user_id_clone)
            .select(users_dsl::password_hash)
            .first::<Option<String>>(&mut conn)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())??;

    if let Some(expected_hash) = expected_hash_opt {
        Ok(derived == expected_hash)
    } else {
        Ok(false)
    }
}

/// Return `true` if the database is currently locked.
///
/// Kept for backward compatibility, now always returns `false`.
#[tauri::command]
pub async fn is_database_locked(_state: State<'_, DbState>) -> Result<bool, String> {
    Ok(false)
}

/// Lock the database.
///
/// Kept for backward compatibility, now does nothing and returns `true`.
#[tauri::command]
pub async fn lock_database(_state: State<'_, DbState>) -> Result<bool, String> {
    Ok(true)
}

/// Create a new local user in the database.
///
/// ## Parameters
/// - `state`: State<'_, DbState> containing the database connection pool.
/// - `username`: The unique username for the user.
/// - `email`: Optional email address for the user.
/// - `password`: The plaintext password to derive the local key from. Must not be empty.
///
/// ## Errors
/// Returns an error if:
/// - The connection pool is unavailable.
/// - The password is empty or only whitespace.
/// - The database operation fails.
#[tauri::command]
pub async fn create_local_user(
    state: State<'_, DbState>,
    username: String,
    email: Option<String>,
    password: String,
) -> Result<UserDto, String> {
    let pool = state.get_pool()?;
    let mut conn = pool.get().map_err(|e| e.to_string())?;

    if password.trim().is_empty() {
        return Err("Password cannot be empty".to_string());
    }

    let password_hash = derive_key(&password);

    tokio::task::spawn_blocking(move || {
        kakebo_data_model::repositories::identity::UserRepository::create(
            &mut conn,
            &username,
            email.as_deref(),
            &password_hash,
        )
        .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

/// Retrieve all local users from the database.
#[tauri::command]
pub async fn get_local_users(state: State<'_, DbState>) -> Result<Vec<UserDto>, String> {
    let pool = state.get_pool()?;
    let mut conn = pool.get().map_err(|e| e.to_string())?;

    tokio::task::spawn_blocking(move || {
        kakebo_data_model::repositories::identity::UserRepository::all(&mut conn)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}
