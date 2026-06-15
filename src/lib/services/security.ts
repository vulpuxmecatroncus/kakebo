import { invoke } from "@tauri-apps/api/core";
import type { UserDto } from "$bindings/UserDto";

/**
 * Service for local password, biometric credentials, and local user management.
 */

/**
 * Checks if any local users have already been set up.
 */
export async function hasLocalUsers(): Promise<boolean> {
  try {
    return await invoke<boolean>("has_local_users");
  } catch (error) {
    console.error("Failed to check if local users exist:", error);
    throw new Error(typeof error === "string" ? error : "Unknown IPC error");
  }
}

/**
 * Authenticates a user using their password.
 */
export async function loginWithPassword(userId: string, password: string): Promise<boolean> {
  try {
    return await invoke<boolean>("login_with_password", { userId, password });
  } catch (error) {
    console.error("Failed to login with password:", error);
    throw new Error(typeof error === "string" ? error : "Unknown IPC error");
  }
}

/**
 * Checks if the database is currently locked.
 */
export async function isDatabaseLocked(): Promise<boolean> {
  try {
    return await invoke<boolean>("is_database_locked");
  } catch (error) {
    console.error("Failed to check if database is locked:", error);
    throw new Error(typeof error === "string" ? error : "Unknown IPC error");
  }
}

/**
 * Creates a new local user with a password.
 */
export async function createLocalUser(
  username: string,
  email: string | null,
  password: string
): Promise<UserDto> {
  try {
    return await invoke<UserDto>("create_local_user", { username, email, password });
  } catch (error) {
    console.error("Failed to create local user:", error);
    throw new Error(typeof error === "string" ? error : "Unknown IPC error");
  }
}

/**
 * Retrieves all registered local users.
 */
export async function getLocalUsers(): Promise<UserDto[]> {
  try {
    return await invoke<UserDto[]>("get_local_users");
  } catch (error) {
    console.error("Failed to get local users:", error);
    throw new Error(typeof error === "string" ? error : "Unknown IPC error");
  }
}

/**
 * Checks if biometric authentication is set up and available on this device for the user.
 */
export async function isBiometricsAvailable(userId: string | null = null): Promise<boolean> {
  try {
    return await invoke<boolean>("is_biometrics_available", { userId });
  } catch (error) {
    console.error("Failed to check biometrics availability:", error);
    throw new Error(typeof error === "string" ? error : "Unknown IPC error");
  }
}

/**
 * Enables biometrics for the local user account, storing credentials in OS secure store.
 */
export async function enableBiometrics(userId: string, password: string): Promise<boolean> {
  try {
    return await invoke<boolean>("enable_biometrics", { userId, password });
  } catch (error) {
    console.error("Failed to enable biometrics:", error);
    throw new Error(typeof error === "string" ? error : "Unknown IPC error");
  }
}

/**
 * Disables biometrics for the local user account, removing credentials from OS secure store.
 */
export async function disableBiometrics(userId: string): Promise<boolean> {
  try {
    return await invoke<boolean>("disable_biometrics", { userId });
  } catch (error) {
    console.error("Failed to disable biometrics:", error);
    throw new Error(typeof error === "string" ? error : "Unknown IPC error");
  }
}


/**
 * Authenticates a user using OS biometrics (TouchID / Windows Hello).
 */
export async function loginWithBiometrics(userId: string): Promise<boolean> {
  try {
    return await invoke<boolean>("login_with_biometrics", { userId });
  } catch (error) {
    console.error("Failed to login with biometrics:", error);
    throw new Error(typeof error === "string" ? error : "Unknown IPC error");
  }
}

/**
 * Locks the database pool.
 */
export async function lockDatabase(): Promise<boolean> {
  try {
    return await invoke<boolean>("lock_database");
  } catch (error) {
    console.error("Failed to lock database:", error);
    throw new Error(typeof error === "string" ? error : "Unknown IPC error");
  }
}
