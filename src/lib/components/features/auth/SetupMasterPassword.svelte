<script lang="ts">
  import { Lock, Eye, EyeOff, ArrowRight } from "@lucide/svelte";
  
  interface Props {
    isPending: boolean;
    onSubmit: (password: string) => Promise<void>;
  }
  
  let { isPending, onSubmit }: Props = $props();
  
  let password = $state("");
  let confirmPassword = $state("");
  let showPassword = $state(false);
  let validationError = $state("");

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (!password) {
      validationError = "Password cannot be empty";
      return;
    }
    if (password !== confirmPassword) {
      validationError = "Passwords do not match";
      return;
    }
    validationError = "";
    await onSubmit(password);
  }
</script>

<div>
  <div class="mb-6">
    <h2 class="text-lg font-semibold text-zinc-100 flex items-center gap-2">
      <Lock class="h-5 w-5 text-violet-400" /> Secure Database Setup
    </h2>
    <p class="text-xs text-zinc-400 mt-1 leading-relaxed">
      Create a master password. This password derives the cryptographic key used to lock and encrypt your local budget database.
    </p>
  </div>

  {#if validationError}
    <div class="mb-4 p-3 rounded-lg bg-red-950/40 border border-red-900/50 text-xs text-red-400">
      {validationError}
    </div>
  {/if}

  <form onsubmit={handleSubmit} class="space-y-4">
    <div class="space-y-1.5">
      <label for="master-pwd" class="text-xs text-zinc-400 font-medium">Master Password</label>
      <div class="relative">
        <input 
          id="master-pwd"
          type={showPassword ? "text" : "password"}
          bind:value={password}
          placeholder="Choose a strong password..."
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

    <div class="space-y-1.5">
      <label for="confirm-pwd" class="text-xs text-zinc-400 font-medium">Confirm Password</label>
      <input 
        id="confirm-pwd"
        type={showPassword ? "text" : "password"}
        bind:value={confirmPassword}
        placeholder="Re-type your password..."
        required
        disabled={isPending}
        class="w-full bg-zinc-950/80 border border-zinc-800 rounded-xl px-4 py-3 text-sm focus:outline-none focus:border-violet-500 focus:ring-1 focus:ring-violet-500/30 transition-all text-zinc-100 placeholder-zinc-600 disabled:opacity-50"
      />
    </div>

    <button 
      type="submit" 
      disabled={isPending}
      class="w-full bg-linear-to-r from-violet-600 to-indigo-600 hover:from-violet-500 hover:to-indigo-500 text-white font-medium text-sm rounded-xl py-3 shadow-lg shadow-indigo-500/10 hover:shadow-indigo-500/20 active:scale-[0.98] transition-all flex items-center justify-center gap-2 mt-6 cursor-pointer disabled:opacity-50"
    >
      {#if isPending}
        <div class="h-4 w-4 border-2 border-white/20 border-t-white rounded-full animate-spin"></div>
        Encrypting Database...
      {:else}
        Initialize Database <ArrowRight class="h-4 w-4" />
      {/if}
    </button>
  </form>
</div>
