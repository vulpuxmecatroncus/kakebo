<script lang="ts">
  import { User, ArrowRight, Eye, EyeOff } from "@lucide/svelte";
  
  interface Props {
    isPending: boolean;
    hasUsers: boolean;
    onSubmit: (username: string, email: string | null, password: string) => Promise<void>;
    onBack?: () => void;
  }
  
  let { isPending, hasUsers, onSubmit, onBack }: Props = $props();
  
  let username = $state("");
  let email = $state("");
  let password = $state("");
  let confirmPassword = $state("");
  let showPassword = $state(false);
  let validationError = $state("");

  function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (!username.trim()) {
      validationError = "Username is required";
      return;
    }
    if (!password) {
      validationError = "Password is required";
      return;
    }
    if (password !== confirmPassword) {
      validationError = "Passwords do not match";
      return;
    }
    validationError = "";
    onSubmit(username.trim(), email.trim() || null, password);
    username = "";
    email = "";
    password = "";
    confirmPassword = "";
  }
</script>

<div>
  <div class="mb-6 flex justify-between items-start">
    <div>
      <h2 class="text-lg font-semibold text-zinc-100 flex items-center gap-2">
        <User class="h-5 w-5 text-violet-400" /> Create Profile
      </h2>
      <p class="text-xs text-zinc-400 mt-1 leading-relaxed">
        Create your local budget profile and choose a password. Your ledger data will be protected under this profile.
      </p>
    </div>
    {#if hasUsers && onBack}
      <button 
        type="button"
        onclick={onBack}
        disabled={isPending}
        class="text-xs font-semibold text-zinc-400 hover:text-zinc-300 transition-colors cursor-pointer"
      >
        Cancel
      </button>
    {/if}
  </div>

  {#if validationError}
    <div class="mb-4 p-3 rounded-xl bg-red-950/40 border border-red-900/50 text-xs text-red-400">
      {validationError}
    </div>
  {/if}

  <form onsubmit={handleSubmit} class="space-y-4">
    <div class="space-y-1.5">
      <label for="profile-username" class="text-xs text-zinc-400 font-medium">Username</label>
      <input 
        id="profile-username"
        type="text"
        bind:value={username}
        placeholder="e.g. Alice"
        required
        disabled={isPending}
        class="w-full bg-zinc-950/80 border border-zinc-800 rounded-xl px-4 py-3 text-sm focus:outline-none focus:border-violet-500 focus:ring-1 focus:ring-violet-500/30 transition-all text-zinc-100 placeholder-zinc-600 disabled:opacity-50"
      />
    </div>

    <div class="space-y-1.5">
      <label for="profile-email" class="text-xs text-zinc-400 font-medium">Email Address (Optional)</label>
      <input 
        id="profile-email"
        type="email"
        bind:value={email}
        placeholder="e.g. alice@domain.local"
        disabled={isPending}
        class="w-full bg-zinc-950/80 border border-zinc-800 rounded-xl px-4 py-3 text-sm focus:outline-none focus:border-violet-500 focus:ring-1 focus:ring-violet-500/30 transition-all text-zinc-100 placeholder-zinc-600 disabled:opacity-50"
      />
    </div>

    <div class="space-y-1.5">
      <label for="profile-pwd" class="text-xs text-zinc-400 font-medium">User Password</label>
      <div class="relative">
        <input 
          id="profile-pwd"
          type={showPassword ? "text" : "password"}
          bind:value={password}
          placeholder="Choose a password..."
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
      <label for="profile-confirm-pwd" class="text-xs text-zinc-400 font-medium">Confirm Password</label>
      <input 
        id="profile-confirm-pwd"
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
      class="w-full bg-linear-to-r from-violet-600 to-indigo-600 hover:from-violet-500 hover:to-indigo-500 text-white font-medium text-sm rounded-xl py-3 shadow-lg shadow-indigo-500/10 active:scale-[0.98] transition-all flex items-center justify-center gap-2 mt-6 cursor-pointer disabled:opacity-50"
    >
      {#if isPending}
        <div class="h-4 w-4 border-2 border-white/20 border-t-white rounded-full animate-spin"></div>
        Creating Profile...
      {:else}
        Create & Log In <ArrowRight class="h-4 w-4" />
      {/if}
    </button>
  </form>
</div>
