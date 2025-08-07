<script lang="ts">
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { blur, fade } from "svelte/transition";

  let { searchResults, loading } = $props();

  const formatter = new Intl.NumberFormat("pt-PT");
</script>

<div class="flex min-h-[1.5rem]">
  {#if loading}
    <div in:fade={{ duration: 300 }}>
      <Skeleton class="h-[1em] w-[15em]" />
    </div>
  {:else}
    <p class="text-muted-foreground text-right" in:blur={{ duration: 300 }}>
      {formatter.format(searchResults.total)}
      {searchResults.total === 1 ? "contrato encontrado" : "contratos encontrados"}
      em {searchResults.elapsedMillis}ms
    </p>
  {/if}
</div>
