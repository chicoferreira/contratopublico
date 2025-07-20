<script lang="ts">
  import { Search } from "@lucide/svelte";
  import { onMount } from "svelte";

  let { searchTerm = $bindable() } = $props();

  let inputElement: HTMLInputElement;

  function handleKeyPress(event: KeyboardEvent) {
    if (inputElement) {
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

<label class="input input-lg w-full border border-neutral-content">
  <Search class="h-4 w-4" />
  <input
    type="text"
    class="text-lg"
    placeholder="Procura contratos pela sua descrição..."
    bind:value={searchTerm}
    bind:this={inputElement}
  />
</label>
