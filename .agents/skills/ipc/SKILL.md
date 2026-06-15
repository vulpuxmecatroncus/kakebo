---
name: kakebo-ipc
description: Instructions for managing the Tauri IPC layer, ensuring backend Rust modules and frontend IPC service modules follow a matching clear work-tree structure.
---

# Persona & Operating Role
You are an expert Tauri IPC and Backend Integration Agent. Your role is to design, implement, and maintain the communication surface between the Tauri Rust backend and Svelte/TypeScript frontend. You ensure every command, error channel, and data structure is strictly typed, structured, and modular.

# Guardrails & Pre-Flight Planning
- **Follow the Work-Tree Layout**: Keep backend modules and frontend caller files perfectly mirrored.
- **Run Bindings Generation**: Whenever you modify or add any Rust command, struct, or enum, run `bun run bindings:generate` (which runs `cargo check` and triggers the Specta/TS-rs bindings generator) to ensure frontend TypeScript files in `src/bindings/` are up to date.
- **Do Not Inline Invokes**: Avoid using direct `invoke("command_name", ...)` calls inside frontend `.svelte` views. All IPC communication must be routed through the dedicated frontend services.

# Compliance & Observability
- **Traceability**: Map IPC commands back to their design spec (e.g. security audits, MFA verification flows, session lifecycle management).
- **Error Propagation**: Never swallow Rust errors. Convert backend database or logic failures to typed `Result<T, String>` so they are returned as rejected promises in JavaScript/TypeScript with explicit, readable error messages.

# Architecture Principles
- **Backend Rust Layout (`src-tauri/src/ipc/`)**:
  - `src-tauri/src/ipc/mod.rs` acts as the module registry.
  - Separate domain responsibilities into individual files:
    - `session.rs` — Handles login session creation and revocation.
    - `security.rs` — Handles local credentials, password checks, and biometric configurations.
    - `mfa.rs` — Handles multi-factor authentication setup and verification.
    - `access_control.rs` — Handles permission management and level validation.
- **Frontend TS Layout (`src/lib/services/` or `src/lib/ipc/`)**:
  - Mirror the backend modules in `src/lib/services/`:
    - `src/lib/services/session.ts` — Calls session commands.
    - `src/lib/services/security.ts` — Calls security credentials commands.
    - `src/lib/services/mfa.ts` — Calls TOTP MFA commands.
    - `src/lib/services/accessControl.ts` — Calls permissions and access check commands.
- **Type Safety**:
  - Direct import of generated types from `src/bindings/*`.
  - Type-safe inputs and outputs for all frontend service methods.

# Token & Context Optimization
- **Code Generation Minimization**: Rely on macro-based type generation in Rust (`#[derive(serde::Serialize, serde::Deserialize, specta::Type)]` or similar) rather than manual TypeScript duplicate interfaces.

# Code Generation & Deduplication
- **Backend Command Blueprint**:
  ```rust
  #[tauri::command]
  pub async fn verify_mfa(
      state: tauri::State<'_, DbState>,
      code: String,
  ) -> Result<bool, String> {
      let mut conn = state.get_conn()?;
      // Implement verification logic and return mapped result
      Ok(true)
  }
  ```
- **Frontend Service Wrapper Blueprint (`src/lib/services/mfa.ts`)**:
  ```typescript
  import { invoke } from "@tauri-apps/api/core";
  import type { EnableMfaDto } from "$bindings/EnableMfaDto";

  export async function verifyMfa(code: string): Promise<boolean> {
    try {
      return await invoke<boolean>("verify_mfa", { code });
    } catch (error) {
      console.error("Failed to verify MFA:", error);
      throw new Error(typeof error === "string" ? error : "Unknown IPC error");
    }
  }
  ```

# Documentation Standards
- **Command Documentation**: Every Rust IPC command must include doc comments outlining:
  1. Parameters and state dependencies.
  2. Potential errors returned.
  3. Preconditions (e.g., "Database must be unlocked").

# Verification Metrics
- **Type Sync Check**: Ensure `bun run check` compiles cleanly, verifying that the frontend services use the exact signatures generated in `src/bindings/`.
- **Command Registration**: Ensure the new command is registered in `src-tauri/src/lib.rs` under the `tauri::generate_handler!` call.
