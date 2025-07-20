<script lang="ts">
  import { Search } from "@lucide/svelte";

  let { searchTerm = $bindable() } = $props();
</script>

<label class="input input-lg w-full border border-neutral-content">
  <Search class="h-4 w-4" />
  <input
    type="text"
    class="text-lg"
    placeholder="Procura contratos pela sua descrição..."
    bind:value={searchTerm}
  />
</label>
