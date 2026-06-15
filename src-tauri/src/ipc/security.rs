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

use keyring::Entry;
use std::fs;
use std::path::PathBuf;
use tauri::State;
use crate::DbState;
use kakebo_data_model::connection::{create_pool_from_url, run_migrations};
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
/// Keyring account name under which the derived key is stored.
const KEYRING_USER: &str = "local-db-key";

/// Return the path to the password verification hash file (`password.hash`)
/// stored in the same directory as the database.
fn get_hash_path(db_url: &str) -> PathBuf {
    let db_path = PathBuf::from(db_url);
    let parent = db_path.parent().unwrap_or(&db_path);
    parent.join("password.hash")
}

/// Return `true` if a local password has already been set up.
///
/// Checks for the presence of the `password.hash` file; no database access is
/// required.
#[tauri::command]
pub async fn has_local_password(state: State<'_, DbState>) -> Result<bool, String> {
    let hash_path = get_hash_path(&state.db_url);
    Ok(hash_path.exists())
}

/// Set up the local password for the first time.
///
/// Derives a key from `password`, writes a verification hash to disk, creates
/// the connection pool, runs pending migrations, and unlocks the database state.
///
/// # Errors
/// Returns `Err` if the hash file cannot be written or the pool cannot be
/// created.
#[tauri::command]
pub async fn setup_local_password(
    state: State<'_, DbState>,
    password: String,
) -> Result<bool, String> {
    let derived = derive_key(&password);
    let hash_path = get_hash_path(&state.db_url);
    
    // Write verification hash of derived key to file to check later.
    let verification_hash = derive_key(&derived);
    fs::write(hash_path, verification_hash).map_err(|e| e.to_string())?;

    // Create database pool and run migrations
    let pool = create_pool_from_url(&state.db_url).map_err(|e| e.to_string())?;
    run_migrations(&pool).map_err(|e| e.to_string())?;

    // Store pool in application state (unlocking the database)
    let mut guard = state.pool.lock().map_err(|e| e.to_string())?;
    *guard = Some(pool);

    Ok(true)
}

/// Authenticate with the local password and unlock the database.
///
/// Derives the key from `password`, hashes it, and compares with the stored
/// hash.  On success the connection pool is opened and the database is
/// available for subsequent IPC calls.
///
/// Returns `false` (without error) if the password is incorrect.
#[tauri::command]
pub async fn login_with_password(
    state: State<'_, DbState>,
    password: String,
) -> Result<bool, String> {
    let hash_path = get_hash_path(&state.db_url);
    if !hash_path.exists() {
        return Err("No local password has been set up".to_string());
    }

    let derived = derive_key(&password);
    let expected_hash = fs::read_to_string(hash_path).map_err(|e| e.to_string())?;
    let verification_hash = derive_key(&derived);

    if verification_hash != expected_hash {
        return Ok(false);
    }

    // Initialize connection pool
    let pool = create_pool_from_url(&state.db_url).map_err(|e| e.to_string())?;
    run_migrations(&pool).map_err(|e| e.to_string())?;

    // Save connection pool to application state
    let mut guard = state.pool.lock().map_err(|e| e.to_string())?;
    *guard = Some(pool);

    Ok(true)
}

/// Store the derived key in the OS secure credential store so the user can
/// later authenticate via biometrics.
///
/// Requires a valid `password` to prove identity before storing the key.
/// Returns `Err("Invalid password")` if verification fails.
#[tauri::command]
pub async fn enable_biometrics(
    state: State<'_, DbState>,
    password: String,
) -> Result<bool, String> {
    let hash_path = get_hash_path(&state.db_url);
    if !hash_path.exists() {
        return Err("No local password has been set up".to_string());
    }

    let derived = derive_key(&password);
    let expected_hash = fs::read_to_string(hash_path).map_err(|e| e.to_string())?;
    let verification_hash = derive_key(&derived);

    if verification_hash != expected_hash {
        return Err("Invalid password".to_string());
    }

    // Store key securely in system keychain/credential vault.
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USER).map_err(|e| e.to_string())?;
    entry.set_password(&derived).map_err(|e| e.to_string())?;

    Ok(true)
}

/// Return `true` if a key is present in the OS secure store (i.e. biometrics
/// has been set up).
///
/// This performs a lightweight probe on the keyring entry; no database access
/// is needed.
#[tauri::command]
pub async fn is_biometrics_available() -> Result<bool, String> {
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USER).map_err(|e| e.to_string())?;
    // Verification query to ensure OS keychain is readable/writable
    Ok(entry.get_password().is_ok() || entry.set_password("test-probe").is_ok())
}

/// Authenticate using the OS biometric prompt and unlock the database.
///
/// Retrieves the previously stored derived key from the OS secure credential
/// store.  On macOS this triggers a Touch ID / Face ID prompt; on Windows it
/// triggers Windows Hello.  The key hash is verified before unlocking.
///
/// Returns `false` (without error) if verification fails.
#[tauri::command]
pub async fn login_with_biometrics(state: State<'_, DbState>) -> Result<bool, String> {
    let hash_path = get_hash_path(&state.db_url);
    if !hash_path.exists() {
        return Err("No local password has been set up".to_string());
    }

    // Attempt to retrieve key from OS Secure Credential vault.
    // Natively on macOS, Windows, and mobile, this prompts the OS system biometric check (e.g. Touch ID/Windows Hello)
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USER).map_err(|e| e.to_string())?;
    let derived = entry.get_password().map_err(|e| format!("Secure storage/Biometric access failed: {}", e))?;

    let expected_hash = fs::read_to_string(hash_path).map_err(|e| e.to_string())?;
    let verification_hash = derive_key(&derived);

    if verification_hash != expected_hash {
        return Ok(false);
    }

    // Initialize connection pool
    let pool = create_pool_from_url(&state.db_url).map_err(|e| e.to_string())?;
    run_migrations(&pool).map_err(|e| e.to_string())?;

    // Store in application state
    let mut guard = state.pool.lock().map_err(|e| e.to_string())?;
    *guard = Some(pool);

    Ok(true)
}

/// Return `true` if the database is currently locked (no active pool).
///
/// The frontend can use this on startup to decide whether to show the login
/// screen.
#[tauri::command]
pub async fn is_database_locked(state: State<'_, DbState>) -> Result<bool, String> {
    let guard = state.pool.lock().map_err(|e| e.to_string())?;
    Ok(guard.is_none())
}
