<script lang="ts">
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { formatNumber } from "$lib/utils";
  import { fade } from "svelte/transition";

  let { searchResults, loading } = $props();
</script>

<div class="relative flex h-6 items-center">
  {#if loading}
    <div class="absolute" transition:fade={{ duration: 100 }}>
      <Skeleton class="h-[1em] w-[20em]" />
    </div>
  {:else}
    <p class="text-muted-foreground absolute" transition:fade={{ duration: 100 }}>
      {formatNumber(searchResults.total)}
      {searchResults.total === 1 ? "contrato encontrado" : "contratos encontrados"}
      em {searchResults.elapsedMillis}ms
    </p>
  {/if}
</div>
