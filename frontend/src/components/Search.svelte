<script lang="ts">
  import Input from "$lib/components/ui/input/input.svelte";
  import { Search as SearchIcon } from "@lucide/svelte";
  import { onMount } from "svelte";

  let { searchTerm = $bindable() } = $props();

  let inputElement = $state<HTMLInputElement | null>(null);

  function handleKeyPress(_event: KeyboardEvent) {
    const activeElement = document.activeElement;
    const hasFocusedElement =
      activeElement &&
      activeElement !== document.body &&
      activeElement !== document.documentElement;

    if (inputElement && !hasFocusedElement) {
      inputElement.focus();
    }
  }

  onMount(() => {
    document.addEventListener("keypress", handleKeyPress);
    return () => {
      document.removeEventListener("keypress", handleKeyPress);
    };
  });
</script>

<div class="relative">
  <div class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
    <SearchIcon class="text-muted-foreground h-4 w-4" />
  </div>
  <!-- only shown on small screens -->
  <Input
    class="h-12 pl-9 text-lg md:hidden md:text-lg"
    type="text"
    placeholder="Procura contratos pela sua descrição..."
    bind:value={searchTerm}
    bind:ref={inputElement} />
  <!-- only shown on large screens -->
  <Input
    class="hidden h-12 pl-9 text-lg md:block md:text-lg"
    type="text"
    placeholder="Procura contratos pela sua descrição, identificador ou entidades..."
    bind:value={searchTerm}
    bind:ref={inputElement} />
</div>
