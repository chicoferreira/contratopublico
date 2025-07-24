<script lang="ts">
  import "../app.css";
  import type {
    SearchContractsRequest,
    SearchContractsResponse,
  } from "$lib/types/api";
  import Search from "../components/Search.svelte";
  import ContractCard from "../components/ContractCard.svelte";
  import SortDropdown from "../components/SortDropdown.svelte";
  import type { Sort } from "$lib/types/api";
  import { searchContracts } from "$lib";

  let search = $state("");

  let { data } = $props();
  let searchResults = $state<SearchContractsResponse>(data.contracts);
  let sortBy = $state<Sort.SortBy>(data.sort);

  $effect(() => {
    if (
      search === "" &&
      sortBy.direction === data.sort.direction &&
      sortBy.field === data.sort.field
    ) {
      searchResults = data.contracts;
      return;
    }

    const request: SearchContractsRequest = {
      query: search,
      sort: sortBy,
    };

    async function run() {
      try {
        const response = await searchContracts(request);
        searchResults = response;
      } catch (error) {
        console.error("Error searching contracts:", error);
      }
    }

    run();
  });
</script>

<div class="space-y-4">
  <div class="text-2xl font-semibold">Procura por contratos públicos</div>

  <div class="space-y-1">
    <Search bind:searchTerm={search}></Search>

    <SortDropdown bind:sortBy />

    <p class="text-muted-foreground">
      {searchResults.total}
      {searchResults.total === 1
        ? "contrato encontrado"
        : "contratos encontrados"}

      em {searchResults.elapsedMillis}ms
    </p>
  </div>

  {#each searchResults.contracts as contract}
    <ContractCard {contract} />
  {/each}
</div>
