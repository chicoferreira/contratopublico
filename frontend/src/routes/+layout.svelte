<script>
  let { children } = $props();
  import "../app.css";

  import { NotebookText } from "@lucide/svelte";
  import { goto } from "$app/navigation";

  function handleLogoClick() {
    goto("/", { replaceState: false });
  }
</script>

<svelte:head>
  <title>Contrato Público</title>
</svelte:head>

<div class="bg-primary text-primary-foreground">
  <div class="container mx-auto px-4 py-5">
    <button
      type="button"
      class="flex cursor-pointer items-center gap-3 transition-opacity hover:opacity-80"
      onclick={handleLogoClick}>
      <NotebookText class="h-8 w-8" />
      <div>
        <h1 class="text-3xl font-bold">Contrato Público</h1>
      </div>
    </button>
  </div>
</div>

<div class="bg-muted min-h-screen">
  <main class="container mx-auto">
    <div class="px-4 py-5">
      {@render children()}
    </div>
  </main>
</div>
