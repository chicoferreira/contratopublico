<script lang="ts">
  import { buttonVariants } from "$lib/components/ui/button";
  import { cn } from "$lib/utils";
  import { ChevronDown, Funnel } from "@lucide/svelte";

  let { filtersOpen = $bindable(), activeFiltersCount } = $props();

  function toggleFilters() {
    filtersOpen = !filtersOpen;
  }
</script>

{#snippet extra()}
  {#if activeFiltersCount}
    : {activeFiltersCount}
    {activeFiltersCount === 1 ? "ativo" : "ativos"}
  {:else}
    {" "}Avançados
  {/if}
{/snippet}

<button
  type="button"
  class={cn(buttonVariants({ variant: "outline" }), "flex items-center justify-between")}
  onclick={toggleFilters}>
  <span class="flex items-center gap-2">
    <Funnel class="h-4 w-4" />
    Filtros{@render extra()}
  </span>
  <ChevronDown class="h-4 w-4 opacity-50 transition-transform" />
</button>
