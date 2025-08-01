<script>
  let { children } = $props();
  import "../app.css";

  import { ModeWatcher } from "mode-watcher";
  import { NotebookText } from "@lucide/svelte";
  import { goto } from "$app/navigation";
  import DarkModeToggle from "../components/DarkModeToggle.svelte";
  import Footer from "../components/Footer.svelte";

  function handleLogoClick() {
    goto("/", { replaceState: false });
  }
</script>

<ModeWatcher />
<svelte:head>
  <title>Contrato Público</title>
</svelte:head>

<div class="bg-background flex min-h-screen flex-col">
  <div class="bg-muted/60 border-y">
    <div class="container mx-auto px-4 py-5">
      <div class="flex justify-between">
        <button
          type="button"
          class="flex cursor-pointer items-center gap-3 transition-opacity hover:opacity-80"
          onclick={handleLogoClick}>
          <NotebookText class="h-8 w-8" />
          <h1 class="text-primary-background text-3xl font-semibold">
            Contrato Público
          </h1>
        </button>

        <DarkModeToggle />
      </div>
    </div>
  </div>

  <main class="flex-1">
    <div class="container mx-auto">
      <div class="px-4 py-5">
        {@render children()}
      </div>
    </div>
  </main>

  <Footer />
</div>
