# Using shadcn-svelte with Svelte 5 & Tailwind CSS v4

This project is configured with Svelte 5 (using runes), Tailwind CSS v4 (CSS-first architecture), and `shadcn-svelte` components.

Unlike standard component libraries, **shadcn-svelte** does not install components to your `node_modules`. Instead, it places the raw component code directly into your repository (`src/lib/components/ui/`) so you have 100% ownership and control over the styling and behavior.

---

## 📂 Project Structure

*   `src/lib/components/ui/` — Where your UI components (like Button, Input, Table, Dialog, etc.) live.
*   `src/lib/utils.ts` — Contains styling helpers like `cn` (for combining Tailwind classes safely) and TypeScript types for Svelte 5.
*   `src/app.css` — Global CSS entry point importing Tailwind v4.
*   `components.json` — Configuration file for the `shadcn-svelte` CLI tool.

---

## ⚡ How to Add New Components

To add a new component to the project, run the `add` command using `bun x`:

```bash
bun x shadcn-svelte@latest add <component-name>
```

For example, to add a **Card** or **Switch** component:
```bash
bun x shadcn-svelte@latest add card switch
```
This downloads the files directly into your `src/lib/components/ui/` directory.

---

## 🚀 How to Use Components

### 1. Basic Component (e.g. Button)
```svelte
<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
</script>

<Button variant="default" size="sm" onclick={() => console.log("Clicked!")}>
  Click Me
</Button>
```

### 2. Multi-part Component (e.g. Tabs)
```svelte
<script lang="ts">
  import * as Tabs from "$lib/components/ui/tabs/index.js";
</script>

<Tabs.Root value="account" class="w-[400px]">
  <Tabs.List>
    <Tabs.Trigger value="account">Account</Tabs.Trigger>
    <Tabs.Trigger value="password">Password</Tabs.Trigger>
  </Tabs.List>
  <Tabs.Content value="account">
    <p>Manage your account settings here.</p>
  </Tabs.Content>
  <Tabs.Content value="password">
    <p>Change your password here.</p>
  </Tabs.Content>
</Tabs.Root>
```

### 3. Creating a Combobox
Since the Combobox in `shadcn-svelte` is built by composing other primitives, you can combine the `Popover`, `Command`, and `Button` components:

```svelte
<script lang="ts">
  import * as Popover from "$lib/components/ui/popover/index.js";
  import * as Command from "$lib/components/ui/command/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Check, ChevronsUpDown } from "lucide-svelte";
  import { cn } from "$lib/utils.js";

  const items = [
    { value: "usd", label: "US Dollar (USD)" },
    { value: "eur", label: "Euro (EUR)" },
    { value: "jpy", label: "Japanese Yen (JPY)" }
  ];

  let open = $state(false);
  let value = $state("");
  let triggerRef = $state<HTMLButtonElement | null>(null);

  const selectedLabel = $derived(
    items.find((i) => i.value === value)?.label ?? "Select currency..."
  );
</script>

<Popover.Root bind:open>
  <Popover.Trigger asChild>
    {#snippet child({ props })}
      <Button
        variant="outline"
        role="combobox"
        aria-expanded={open}
        class="w-[200px] justify-between"
        bind:ref={triggerRef}
        {...props}
      >
        {selectedLabel}
        <ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
      </Button>
    {/snippet}
  </Popover.Trigger>
  <Popover.Content class="w-[200px] p-0" {triggerRef}>
    <Command.Root>
      <Command.Input placeholder="Search currency..." />
      <Command.Empty>No currency found.</Command.Empty>
      <Command.Group>
        {#each items as item}
          <Command.Item
            value={item.value}
            onSelect={(currentValue) => {
              value = currentValue;
              open = false;
            }}
          >
            <Check
              class={cn(
                "mr-2 h-4 w-4",
                value !== item.value && "text-transparent"
              )}
            />
            {item.label}
          </Command.Item>
        {/each}
      </Command.Group>
    </Command.Root>
  </Popover.Content>
</Popover.Root>
```

---

## 🎨 Theme & Customization (Tailwind CSS v4)

Tailwind CSS v4 uses a **CSS-first** configuration architecture. To customize your design tokens, colors, or variables, modify `src/app.css` instead of a JavaScript config file.

For example, to tweak theme colors or borders, use the `@theme` directive inside [src/app.css](file:///Users/yasuomaidana/Projects/kakebo/src/app.css):

```css
@import "tailwindcss";

@theme {
  /* Override or add theme values */
  --color-primary: #1e3a8a;
  --radius-lg: 0.75rem;
}
```

---

## 🛠️ Diagnostics & Type Checking

To verify that your imports and component props are perfectly aligned with TypeScript, run:
```bash
bun run check
```
This executes SvelteKit sync and checks for any TypeScript diagnostics.
