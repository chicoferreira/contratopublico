<script lang="ts">
  import { buttonVariants } from "$lib/components/ui/button";
  import { cn } from "$lib/utils";
  import { ChevronDown, Funnel } from "@lucide/svelte";
  import { blur } from "svelte/transition";
  import NewBadge from "$lib/components/NewBadge.svelte";

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
  class={cn(buttonVariants({ variant: "outline" }), "flex items-center gap-2")}
  onclick={toggleFilters}>
  <Funnel class="h-4 w-4" />
  {#key activeFiltersCount}
    <p>
      Filtros<span in:blur={{ duration: 200 }}>{@render extra()}</span>
    </p>
  {/key}
  <NewBadge addedDate="2026-11-13" link={false} />
  <ChevronDown class="h-4 w-4 opacity-50 transition-transform" />
</button>
