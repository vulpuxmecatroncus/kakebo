import { invoke } from "@tauri-apps/api/core";
import type { UserSessionDto } from "$bindings/UserSessionDto";
import type { CreateSessionDto } from "$bindings/CreateSessionDto";

/**
 * Service for managing active user sessions.
 */

/**
 * Validates whether a given session ID is active and not expired.
 */
export async function validateSession(sessionId: string): Promise<boolean> {
  try {
    return await invoke<boolean>("validate_session", { sessionId });
  } catch (error) {
    console.error("Failed to validate session:", error);
    throw new Error(typeof error === "string" ? error : "Unknown IPC error");
  }
}

/**
 * Creates a new user session record in the database.
 */
export async function createSession(dto: CreateSessionDto): Promise<UserSessionDto> {
  try {
    return await invoke<UserSessionDto>("create_session", { dto });
  } catch (error) {
    console.error("Failed to create session:", error);
    throw new Error(typeof error === "string" ? error : "Unknown IPC error");
  }
}

/**
 * Revokes a single user session (used for logging out of the current device).
 */
export async function revokeSession(sessionId: string): Promise<number> {
  try {
    return await invoke<number>("revoke_session", { sessionId });
  } catch (error) {
    console.error("Failed to revoke session:", error);
    throw new Error(typeof error === "string" ? error : "Unknown IPC error");
  }
}

/**
 * Revokes all sessions belonging to a user (used for logging out everywhere).
 */
export async function revokeAllSessionsForUser(userId: string): Promise<number> {
  try {
    return await invoke<number>("revoke_all_sessions_for_user", { userId });
  } catch (error) {
    console.error("Failed to revoke all sessions for user:", error);
    throw new Error(typeof error === "string" ? error : "Unknown IPC error");
  }
}
