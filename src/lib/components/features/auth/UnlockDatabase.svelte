<script lang="ts">
  import { KeyRound, Eye, EyeOff, Fingerprint, ArrowLeft } from "@lucide/svelte";
  
  interface Props {
    username: string;
    isPending: boolean;
    biometricsAvailable: boolean;
    onSubmit: (password: string) => Promise<void>;
    onBiometricUnlock: () => Promise<void>;
    onBack: () => void;
  }
  
  let { username, isPending, biometricsAvailable, onSubmit, onBiometricUnlock, onBack }: Props = $props();
  
  let password = $state("");
  let showPassword = $state(false);

  function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (password) {
      onSubmit(password);
      password = "";
    }
  }
</script>

<div>
  <div class="mb-6 text-center relative">
    <button 
      type="button" 
      onclick={onBack}
      disabled={isPending}
      class="absolute left-0 top-1/2 -translate-y-1/2 text-zinc-400 hover:text-zinc-200 transition-colors p-1 cursor-pointer disabled:opacity-50"
      title="Back to profiles"
    >
      <ArrowLeft class="h-4 w-4" />
    </button>
    
    <h2 class="text-lg font-semibold text-zinc-100 flex items-center justify-center gap-2 pl-6 pr-6">
      <KeyRound class="h-5 w-5 text-violet-400" /> Unlock {username}
    </h2>
    <p class="text-xs text-zinc-400 mt-1">Enter your password to open the ledger.</p>
  </div>

  <form onsubmit={handleSubmit} class="space-y-4">
    <div class="space-y-1.5">
      <div class="relative">
        <input 
          id="unlock-pwd"
          type={showPassword ? "text" : "password"}
          bind:value={password}
          placeholder="Enter password..."
          required
          disabled={isPending}
          class="w-full bg-zinc-950/80 border border-zinc-800 rounded-xl px-4 py-3 text-sm focus:outline-none focus:border-violet-500 focus:ring-1 focus:ring-violet-500/30 transition-all text-zinc-100 placeholder-zinc-600 disabled:opacity-50"
        />
        <button 
          type="button" 
          onclick={() => showPassword = !showPassword}
          class="absolute right-3.5 top-1/2 -translate-y-1/2 text-zinc-500 hover:text-zinc-300 transition-colors"
        >
          {#if showPassword}
            <EyeOff class="h-4 w-4" />
          {:else}
            <Eye class="h-4 w-4" />
          {/if}
        </button>
      </div>
    </div>

    <div class="flex gap-2.5 mt-6">
      <button 
        type="submit" 
        disabled={isPending}
        class="flex-1 bg-linear-to-r from-violet-600 to-indigo-600 hover:from-violet-500 hover:to-indigo-500 text-white font-medium text-sm rounded-xl py-3 shadow-lg shadow-indigo-500/10 active:scale-[0.98] transition-all flex items-center justify-center gap-2 cursor-pointer disabled:opacity-50"
      >
        {#if isPending}
          <div class="h-4 w-4 border-2 border-white/20 border-t-white rounded-full animate-spin"></div>
          Unlocking...
        {:else}
          Unlock Profile
        {/if}
      </button>

      {#if biometricsAvailable}
        <button 
          type="button"
          onclick={onBiometricUnlock}
          disabled={isPending}
          title="Unlock with biometric"
          class="bg-zinc-800 hover:bg-zinc-700 border border-zinc-700 text-zinc-300 hover:text-white rounded-xl px-4 py-3 transition-colors flex items-center justify-center cursor-pointer disabled:opacity-50"
        >
          <Fingerprint class="h-5 w-5" />
        </button>
      {/if}
    </div>
  </form>
</div>
