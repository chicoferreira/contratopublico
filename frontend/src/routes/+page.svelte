<script lang="ts">
  import "../app.css";

  import Search from "../components/Search.svelte";
  import ContractCard from "../components/ContractCard.svelte";
  import SortDropdown from "../components/SortDropdown.svelte";
  import { DEFAULT_SEARCH_REQUEST } from "$lib";
  import ContractPagination from "../components/ContractPagination.svelte";
  import { goto } from "$app/navigation";

  let { data } = $props();

  let search = $state(data.query);
  let sortBy = $state(data.sort);
  let page = $state(data.page);
  let searchResults = $state(data.contracts);

  $effect(() => {
    search = data.query;
    sortBy = data.sort;
    page = data.page;
  });

  $effect(() => {
    const params = new URLSearchParams();

    if (search) {
      params.set("query", search);
    }

    if (
      sortBy.field !== DEFAULT_SEARCH_REQUEST.sort!.field ||
      sortBy.direction !== DEFAULT_SEARCH_REQUEST.sort!.direction
    ) {
      params.set("sortField", sortBy.field);
      params.set("sortDirection", sortBy.direction);
    }

    if (page > DEFAULT_SEARCH_REQUEST.page!) {
      params.set("page", page.toString());
    }

    goto(`?${params.toString()}`, {
      replaceState: true,
      noScroll: true,
      keepFocus: true,
    });
  });

  // Update search results when data changes
  $effect(() => {
    searchResults = data.contracts;
  });

  $effect(() => {
    if (searchResults.totalPages > 0 && page > searchResults.totalPages) {
      page = searchResults.totalPages;
    }
  });
</script>

<svelte:head>
  <title>Contrato Público {search ? `- ${search}` : ""}</title>
</svelte:head>

<div class="space-y-4">
  <div class="text-2xl font-semibold">
    Procura por contratos públicos celebrados em Portugal
  </div>

  <div class="space-y-1">
    <Search bind:searchTerm={search} />

    <SortDropdown bind:sortBy />

    <p class="text-muted-foreground">
      {searchResults.total}
      {searchResults.total === 1
        ? "contrato encontrado"
        : "contratos encontrados"}

      ({searchResults.totalPages}
      {searchResults.totalPages === 1 ? "página" : "páginas"}) em {searchResults.elapsedMillis}ms
    </p>
  </div>

  {#each searchResults.contracts as contract}
    <ContractCard {contract} />
  {/each}

  <ContractPagination
    bind:page
    bind:total={searchResults.total}
    bind:hitsPerPage={searchResults.hitsPerPage} />
</div>
