<script lang="ts">
  import { User, Plus, ArrowRight } from "@lucide/svelte";
  import type { UserDto } from "$bindings/UserDto";
  
  interface Props {
    usersList: UserDto[];
    isPending: boolean;
    onSelect: (user: UserDto) => void;
    onAddProfile: () => void;
  }
  
  let { usersList, isPending, onSelect, onAddProfile }: Props = $props();
</script>

<div>
  <div class="mb-6 flex justify-between items-center">
    <div>
      <h2 class="text-lg font-semibold text-zinc-100">Select Profile</h2>
      <p class="text-xs text-zinc-400 mt-1">Choose a local profile to open the ledger.</p>
    </div>
    <button 
      onclick={onAddProfile} 
      class="text-xs font-semibold text-violet-400 hover:text-violet-300 transition-colors flex items-center gap-1 cursor-pointer"
    >
      <Plus class="h-3.5 w-3.5" /> Add Profile
    </button>
  </div>

  <div class="space-y-2 max-h-60 overflow-y-auto pr-1">
    {#each usersList as user}
      <button 
        onclick={() => onSelect(user)}
        disabled={isPending}
        class="w-full bg-zinc-900/80 hover:bg-zinc-800/90 border border-zinc-800/80 hover:border-zinc-700/80 text-left rounded-xl p-4 flex items-center justify-between transition-all group disabled:opacity-50 cursor-pointer"
      >
        <div class="flex items-center gap-3">
          <div class="h-9 w-9 rounded-lg bg-zinc-800 flex items-center justify-center group-hover:bg-violet-950 transition-colors border border-zinc-700 group-hover:border-violet-800">
            <User class="h-4 w-4 text-zinc-400 group-hover:text-violet-300" />
          </div>
          <div>
            <h3 class="text-sm font-semibold text-zinc-200 group-hover:text-white transition-colors">{user.username}</h3>
            {#if user.email}
              <p class="text-xs text-zinc-500 mt-0.5">{user.email}</p>
            {:else}
              <p class="text-xs text-zinc-600 mt-0.5">Local profile</p>
            {/if}
          </div>
        </div>
        <ArrowRight class="h-4 w-4 text-zinc-600 group-hover:text-violet-400 transform group-hover:translate-x-1 transition-all" />
      </button>
    {/each}
  </div>
</div>
