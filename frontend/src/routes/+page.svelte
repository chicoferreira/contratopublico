<script lang="ts">
  import "../app.css";
  import type { SearchContractsResponse } from "$lib/types/api";
  import Search from "../components/Search.svelte";
  import ContractCard from "../components/ContractCard.svelte";
  import SortBy from "../components/SortBy.svelte";
  import type { SortBy as SortByType } from "$lib/types/api";

  let search = $state("");
  let loading = $state(false);

  let sortBy = $state<SortByType>({
    direction: "descending",
    field: "publicationDate",
  });

  let { data } = $props();
  let searchResults = $state<SearchContractsResponse>(data);

  $effect(() => {
    async function searchContracts(searchTerm: string, sortBy: SortByType) {
      loading = true;
      try {
        console.log("searchContracts", searchTerm, sortBy);
        const response = await fetch(
          `/api/search?query=${encodeURIComponent(searchTerm)}&sort[direction]=${sortBy?.direction}&sort[field]=${sortBy?.field}`,
        );
        if (response.ok) {
          searchResults = await response.json();
        } else {
          console.error("Search failed:", response.statusText);
        }
      } catch (error) {
        console.error("Search error:", error);
      } finally {
        loading = false;
      }
    }

    searchContracts(search, sortBy);
  });
</script>

<div class="space-y-4">
  <div class="text-2xl font-semibold">Procura por contratos públicos</div>

  <div class="space-y-1">
    <Search bind:searchTerm={search}></Search>

    <SortBy bind:sortBy></SortBy>

    <p class="text-muted-foreground">
      {searchResults.total}
      {searchResults.total === 1
        ? "contrato encontrado"
        : "contratos encontrados"}
    </p>
  </div>

  {#each searchResults.contracts as contract}
    <ContractCard {contract} />
  {/each}
</div>
