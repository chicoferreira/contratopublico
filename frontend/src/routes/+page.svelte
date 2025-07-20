<script lang="ts">
  import "../app.css";
  import type { SearchContractsResponse } from "$lib/types/api";

  import Search from "../components/Search.svelte";
  import ContractCard from "../components/ContractCard.svelte";

  let search = $state("");
  let loading = $state(false);

  let { data } = $props();
  let searchResults = $state<SearchContractsResponse>(data);

  $effect(() => {
    if (search.trim() === "") {
      searchResults = data;
      return;
    }

    async function searchContracts(searchTerm: string) {
      loading = true;
      try {
        const response = await fetch(
          `/api/search?query=${encodeURIComponent(searchTerm)}`,
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

    searchContracts(search);
  });
</script>

<div class="space-y-4">
  <div class="text-2xl font-semibold">Procura por contratos públicos</div>

  <div class="space-y-1">
    <Search bind:searchTerm={search}></Search>

    <p class="text-base-content/50">
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
