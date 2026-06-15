<script lang="ts">
  import { Unlock, LogOut, Fingerprint, CheckCircle2 } from "@lucide/svelte";
  import type { UserDto } from "$bindings/UserDto";
  
  interface Props {
    currentUser: UserDto | null;
    activeSessionId: string | null;
    biometricsEnabled: boolean;
    onLogout: () => void;
    onEnableBiometrics: (password: string) => Promise<void>;
    onDisableBiometrics: () => Promise<void>;
  }
  
  let { currentUser, activeSessionId, biometricsEnabled, onLogout, onEnableBiometrics, onDisableBiometrics }: Props = $props();
  
  let password = $state("");
  let isPending = $state(false);
  let errorMsg = $state("");
  let successMsg = $state("");

  async function handleEnableBiometrics(e: SubmitEvent) {
    e.preventDefault();
    if (!password) return;
    isPending = true;
    errorMsg = "";
    successMsg = "";
    try {
      await onEnableBiometrics(password);
      successMsg = "Biometric unlock enabled successfully!";
      password = "";
    } catch (err) {
      if (err instanceof Error) {
        errorMsg = err.message;
      } else {
        errorMsg = String(err);
      }
    } finally {
      isPending = false;
    }
  }

  async function handleDisableBiometrics() {
    isPending = true;
    errorMsg = "";
    successMsg = "";
    try {
      await onDisableBiometrics();
      successMsg = "Biometric unlock disabled successfully!";
    } catch (err) {
      if (err instanceof Error) {
        errorMsg = err.message;
      } else {
        errorMsg = String(err);
      }
    } finally {
      isPending = false;
    }
  }
</script>

<div class="text-center animate-in fade-in duration-300">
  <div class="h-16 w-16 rounded-full bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 flex items-center justify-center mx-auto mb-4">
    <Unlock class="h-7 w-7" />
  </div>
  
  <h2 class="text-xl font-bold text-zinc-50">Authorized Access</h2>
  <p class="text-sm text-zinc-400 mt-1">Welcome back, <span class="text-zinc-200 font-semibold">{currentUser?.username}</span>!</p>

  <div class="mt-8 bg-zinc-950/60 border border-zinc-800/60 rounded-xl p-5 text-left space-y-4">
    <div class="flex justify-between items-center border-b border-zinc-800/80 pb-3">
      <span class="text-xs text-zinc-500 font-medium">DATABASE STATUS</span>
      <span class="text-xs px-2 py-0.5 bg-emerald-950 text-emerald-400 border border-emerald-900/50 rounded-full font-bold">UNLOCKED</span>
    </div>

    <div class="space-y-2.5">
      <div class="flex justify-between items-center text-xs">
         <span class="text-zinc-500">Active Profile</span>
        <span class="text-zinc-300 font-medium">{currentUser?.username}</span>
      </div>
      <div class="flex justify-between items-center text-xs">
        <span class="text-zinc-500">Profile ID</span>
        <span class="text-zinc-500 font-mono text-[10px]">{currentUser?.id.substring(0, 18)}...</span>
      </div>
      <div class="flex justify-between items-center text-xs">
        <span class="text-zinc-500">Active Session</span>
        <span class="text-zinc-500 font-mono text-[10px]">{activeSessionId?.substring(0, 18)}...</span>
      </div>
    </div>

    <!-- Biometric Setup Section -->
    <div class="border-t border-zinc-800/80 pt-4 mt-2 space-y-3">
      <div class="flex items-center justify-between">
        <span class="text-xs text-zinc-400 font-medium flex items-center gap-1.5">
          <Fingerprint class="h-4 w-4 text-violet-400" /> Biometric Unlock
        </span>
        {#if biometricsEnabled}
          <span class="text-[10px] text-emerald-400 font-medium flex items-center gap-1">
            <CheckCircle2 class="h-3.5 w-3.5" /> Enabled
          </span>
        {/if}
      </div>

      {#if errorMsg}
        <div class="text-[10px] text-red-400 font-medium">{errorMsg}</div>
      {/if}
      {#if successMsg}
        <div class="text-[10px] text-emerald-400 font-medium">{successMsg}</div>
      {/if}

      {#if !biometricsEnabled}
        <form onsubmit={handleEnableBiometrics} class="space-y-2">
          <div class="flex gap-2">
            <input 
              type="password" 
              bind:value={password} 
              placeholder="Confirm password..." 
              required
              disabled={isPending}
              class="flex-1 bg-zinc-900/60 border border-zinc-800 rounded-lg px-3 py-1.5 text-xs text-zinc-100 placeholder-zinc-600 focus:outline-none focus:border-violet-500"
            />
            <button 
              type="submit"
              disabled={isPending}
              class="bg-violet-600 hover:bg-violet-500 text-white font-semibold text-xs px-3 py-1.5 rounded-lg active:scale-95 transition-all cursor-pointer disabled:opacity-50 animate-in fade-in"
            >
              Enable
            </button>
          </div>
        </form>
      {:else}
        <div class="flex items-center justify-between gap-4 mt-1 animate-in fade-in">
          <p class="text-[11px] text-zinc-500 flex-1">Biometric login is enabled. You can log in using Touch ID next time the app opens.</p>
          <button 
            type="button"
            onclick={handleDisableBiometrics}
            disabled={isPending}
            class="bg-red-950/40 hover:bg-red-900/40 text-red-400 border border-red-900/30 font-semibold text-xs px-2.5 py-1.5 rounded-lg active:scale-95 transition-all cursor-pointer disabled:opacity-50 shrink-0"
          >
            Disable
          </button>
        </div>
      {/if}
    </div>
  </div>


  <button 
    onclick={onLogout}
    class="mt-8 w-full bg-zinc-800 hover:bg-zinc-700 text-zinc-300 hover:text-white font-medium text-sm rounded-xl py-3 border border-zinc-700/80 transition-all flex items-center justify-center gap-2 cursor-pointer active:scale-[0.98]"
  >
    <LogOut class="h-4 w-4" /> Lock & Log Out
  </button>
</div>
