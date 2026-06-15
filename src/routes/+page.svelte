<script lang="ts">
  import { onMount } from "svelte";
  import { 
    hasLocalUsers, 
    loginWithPassword, 
    createLocalUser, 
    getLocalUsers,
    isBiometricsAvailable,
    enableBiometrics,
    disableBiometrics,
    loginWithBiometrics,
    lockDatabase
  } from "$lib/services/security";
  import { createSession, validateSession, revokeSession } from "$lib/services/session";
  import type { UserDto } from "$bindings/UserDto";
  
  // Feature views
  import UnlockDatabase from "$lib/components/features/auth/UnlockDatabase.svelte";
  import SetupProfile from "$lib/components/features/auth/SetupProfile.svelte";
  import UserSelect from "$lib/components/features/auth/UserSelect.svelte";
  import AuthDashboard from "$lib/components/features/auth/AuthDashboard.svelte";

  import { ShieldAlert, Check, Coins } from "@lucide/svelte";

  // App States: loading, setup_profile, user_select, unlock_user, authenticated
  let appState = $state<"loading" | "setup_profile" | "user_select" | "unlock_user" | "authenticated">("loading");
  let isTauri = $state(true);

  // Visual states
  let errorMessage = $state("");
  let successMessage = $state("");
  let isPending = $state(false);
  let biometricsAvailable = $state(false);
  let biometricsEnabled = $state(false);
  
  // Loaded records
  let usersList = $state<UserDto[]>([]);
  let currentUser = $state<UserDto | null>(null);
  let selectedUserForUnlock = $state<UserDto | null>(null);
  let activeSessionId = $state<string | null>(null);

  // Initialize and check states
  onMount(async () => {
    // Check if we are running inside Tauri webview
    isTauri = typeof window !== "undefined" && (("__TAURI__" in window) || ("__TAURI_INTERNALS__" in window));
    if (!isTauri) {
      appState = "loading";
      errorMessage = "Tauri environment not detected. Secure database access and local keychain credentials are only available when running inside the Tauri shell. Please launch the application using: bun tauri dev";
      isPending = false;
      return;
    }

    try {
      await checkAppState();
    } catch (err) {
      showError(err);
    }
  });

  async function checkAppState() {
    errorMessage = "";
    isPending = true;
    try {
      const hasUsers = await hasLocalUsers();
      if (!hasUsers) {
        appState = "setup_profile";
        isPending = false;
        return;
      }

      // Check if we have an active session in localStorage
      const savedSessionId = localStorage.getItem("kakebo_session_id");
      const savedUserStr = localStorage.getItem("kakebo_user");
      
      if (savedSessionId && savedUserStr) {
        try {
          const isValid = await validateSession(savedSessionId);
          if (isValid) {
            currentUser = JSON.parse(savedUserStr);
            activeSessionId = savedSessionId;
            biometricsEnabled = localStorage.getItem("kakebo_biometrics_enabled") === "true";
            appState = "authenticated";
            isPending = false;
            return;
          }
        } catch (e) {
          console.warn("Failed to validate session:", e);
        }
      }

      // Clear invalid session info if any
      localStorage.removeItem("kakebo_session_id");
      localStorage.removeItem("kakebo_user");

      await refreshUsers();
    } catch (err) {
      showError(err);
    } finally {
      isPending = false;
    }
  }

  async function refreshUsers() {
    try {
      const users = await getLocalUsers();
      usersList = users.filter(u => !u.is_shadow);
      
      if (usersList.length === 0) {
        appState = "setup_profile";
      } else {
        appState = "user_select";
      }
    } catch (err) {
      showError(err);
    }
  }

  // Action Handlers passed to components
  async function handleUserSelected(user: UserDto) {
    selectedUserForUnlock = user;
    isPending = true;
    try {
      const hardwareAvailable = await isBiometricsAvailable();
      const userEnabled = await isBiometricsAvailable(user.id);
      biometricsAvailable = hardwareAvailable && userEnabled;
      appState = "unlock_user";
    } catch (err) {
      showError(err);
    } finally {
      isPending = false;
    }
  }

  async function onSubmitUnlockPassword(password: string) {
    if (!selectedUserForUnlock) return;
    errorMessage = "";
    isPending = true;
    try {
      const success = await loginWithPassword(selectedUserForUnlock.id, password);
      if (success) {
        await handleUserLogin(selectedUserForUnlock);
      } else {
        errorMessage = "Incorrect password";
      }
    } catch (err) {
      showError(err);
    } finally {
      isPending = false;
    }
  }

  async function onBiometricUnlock() {
    if (!selectedUserForUnlock) return;
    errorMessage = "";
    isPending = true;
    try {
      const success = await loginWithBiometrics(selectedUserForUnlock.id);
      if (success) {
        await handleUserLogin(selectedUserForUnlock);
      } else {
        errorMessage = "Biometric authentication failed";
      }
    } catch (err) {
      showError(err);
    } finally {
      isPending = false;
    }
  }

  async function onSubmitProfile(username: string, email: string | null, password: string) {
    errorMessage = "";
    isPending = true;
    try {
      const newProfile = await createLocalUser(username, email, password);
      await handleUserLogin(newProfile);
    } catch (err) {
      showError(err);
    } finally {
      isPending = false;
    }
  }

  async function handleUserLogin(user: UserDto) {
    isPending = true;
    errorMessage = "";
    try {
      const sessionId = crypto.randomUUID();
      const expiresAt = new Date(Date.now() + 1000 * 60 * 60 * 24 * 7).toISOString();
      
      const session = await createSession({
        id: sessionId,
        user_id: user.id,
        user_agent: navigator.userAgent,
        ip_address: "127.0.0.1",
        expires_at: expiresAt
      });

      currentUser = user;
      activeSessionId = session.id;

      // Persist session in localStorage
      localStorage.setItem("kakebo_session_id", session.id);
      localStorage.setItem("kakebo_user", JSON.stringify(user));

      // Read biometric state from local storage/keyring
      biometricsEnabled = await isBiometricsAvailable(user.id);
      localStorage.setItem("kakebo_biometrics_enabled", biometricsEnabled ? "true" : "false");

      appState = "authenticated";
    } catch (err) {
      showError(err);
    } finally {
      isPending = false;
    }
  }

  async function onEnableBiometrics(password: string) {
    if (!currentUser) return;
    const success = await enableBiometrics(currentUser.id, password);
    if (success) {
      biometricsEnabled = true;
      localStorage.setItem("kakebo_biometrics_enabled", "true");
    } else {
      throw new Error("Incorrect password. Biometrics could not be enabled.");
    }
  }

  async function onDisableBiometrics() {
    if (!currentUser) return;
    const success = await disableBiometrics(currentUser.id);
    if (success) {
      biometricsEnabled = false;
      localStorage.setItem("kakebo_biometrics_enabled", "false");
    } else {
      throw new Error("Failed to disable biometrics.");
    }
  }

  async function handleLogout() {
    isPending = true;
    try {
      if (activeSessionId) {
        await revokeSession(activeSessionId);
      }
    } catch (e) {
      console.error("Failed to revoke session on logout:", e);
    }

    try {
      await lockDatabase();
    } catch (e) {
      console.error("Failed to lock database on logout:", e);
    }

    currentUser = null;
    activeSessionId = null;
    selectedUserForUnlock = null;
    biometricsAvailable = false;
    localStorage.removeItem("kakebo_session_id");
    localStorage.removeItem("kakebo_user");
    
    await refreshUsers();
    isPending = false;
  }

  function showError(err: unknown) {
    console.error(err);
    if (err instanceof Error) {
      errorMessage = err.message;
    } else if (typeof err === "string") {
      errorMessage = err;
    } else {
      errorMessage = "An unexpected error occurred";
    }
  }
</script>

<main class="min-h-screen w-full flex items-center justify-center bg-gradient-to-br from-zinc-950 via-zinc-900 to-zinc-950 text-zinc-100 p-4 font-sans select-none overflow-hidden relative">
  <!-- Glowing Background Orbs -->
  <div class="absolute top-[-20%] left-[-10%] w-[500px] h-[500px] rounded-full bg-violet-600/10 blur-[120px] pointer-events-none"></div>
  <div class="absolute bottom-[-20%] right-[-10%] w-[500px] h-[500px] rounded-full bg-indigo-600/10 blur-[120px] pointer-events-none"></div>

  <!-- Main Container -->
  <div class="w-full max-w-md bg-zinc-900/50 backdrop-blur-xl border border-zinc-800/80 rounded-2xl p-8 shadow-2xl relative z-10 transition-all duration-300">
    <!-- Header -->
    <div class="flex flex-col items-center mb-8">
      <div class="h-14 w-14 rounded-2xl bg-gradient-to-tr from-violet-600 to-indigo-600 flex items-center justify-center shadow-lg shadow-indigo-500/20 mb-4 transform hover:rotate-12 transition-transform duration-300">
        <Coins class="h-8 w-8 text-white" />
      </div>
      <h1 class="text-2xl font-bold tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-zinc-50 via-zinc-200 to-zinc-400">Kakebo</h1>
      <p class="text-xs text-zinc-500 mt-1 uppercase tracking-widest font-semibold font-mono">OFFLINE PERSONAL LEDGER</p>
    </div>

    <!-- Error/Success Alerts -->
    {#if errorMessage}
      <div class="mb-6 p-4 rounded-xl bg-red-950/40 border border-red-900/50 flex items-start gap-3 animate-in fade-in slide-in-from-top-2 duration-250">
        <ShieldAlert class="h-5 w-5 text-red-400 shrink-0 mt-0.5" />
        <p class="text-xs text-red-300 leading-relaxed font-medium">{errorMessage}</p>
      </div>
    {/if}

    {#if successMessage}
      <div class="mb-6 p-4 rounded-xl bg-emerald-950/40 border border-emerald-900/50 flex items-start gap-3 animate-in fade-in slide-in-from-top-2 duration-250">
        <Check class="h-5 w-5 text-emerald-400 shrink-0 mt-0.5" />
        <p class="text-xs text-emerald-300 leading-relaxed font-medium">{successMessage}</p>
      </div>
    {/if}

    <!-- View Switcher -->
    {#if !isTauri}
      <!-- Safe fallback when not inside Tauri webview -->
      <div class="text-center py-6">
        <p class="text-xs text-zinc-400 leading-relaxed">
          To run and test the app with full database and secure keychain features, make sure the Tauri dev process is active.
        </p>
      </div>
    {:else if appState === "loading"}
      <div class="flex flex-col items-center justify-center py-12">
        <div class="h-10 w-10 border-2 border-zinc-800 border-t-violet-500 rounded-full animate-spin"></div>
        <p class="text-sm text-zinc-400 mt-4 font-medium">Initializing secure database...</p>
      </div>
    {:else if appState === "setup_profile"}
      <SetupProfile 
        {isPending} 
        hasUsers={usersList.length > 0}
        onSubmit={onSubmitProfile} 
        onBack={() => appState = "user_select"}
      />
    {:else if appState === "user_select"}
      <UserSelect 
        {usersList} 
        {isPending} 
        onSelect={handleUserSelected} 
        onAddProfile={() => appState = "setup_profile"} 
      />
    {:else if appState === "unlock_user"}
      <UnlockDatabase 
        username={selectedUserForUnlock?.username ?? ""}
        {isPending} 
        {biometricsAvailable} 
        onSubmit={onSubmitUnlockPassword} 
        {onBiometricUnlock} 
        onBack={() => appState = "user_select"}
      />
    {:else if appState === "authenticated"}
      <AuthDashboard 
        {currentUser} 
        {activeSessionId} 
        {biometricsEnabled}
        onLogout={handleLogout} 
        {onEnableBiometrics}
        {onDisableBiometrics}
      />
    {/if}
  </div>
</main>
