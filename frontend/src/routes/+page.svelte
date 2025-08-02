<script lang="ts">
  import "../app.css";

  import Search from "../components/Search.svelte";
  import ContractCard from "../components/ContractCard.svelte";
  import SortDropdown from "../components/SortDropdown.svelte";
  import { fade } from "svelte/transition";
  import { DEFAULT_SEARCH_REQUEST, searchContracts } from "$lib";
  import ContractPagination from "../components/ContractPagination.svelte";
  import { replaceState } from "$app/navigation";
  import { page as sveltePage } from "$app/state";
  import { Sort } from "$lib/types/api";
  import { validateEnumOrDefault } from "$lib/utils";
  import { untrack } from "svelte";
  import { Skeleton } from "$lib/components/ui/skeleton";

  let { data } = $props();

  let initialQuery = data.query;
  let initialSort = data.sort;
  let initialPage = data.page;

  let query = $state(initialQuery);
  let sort = $state(initialSort);
  let page = $state(initialPage);
  let searchResults = $state(data.contracts);

  let lastRequest = $state({ query: initialQuery, sort: initialSort, page: initialPage });

  let loading = $state(false);

  async function updateUrl(query: string, sort: Sort.SortBy, page: number) {
    const params = new URLSearchParams();

    if (query) params.set("query", query);

    const defaultSort = DEFAULT_SEARCH_REQUEST.sort!;
    if (sort.field !== defaultSort.field || sort.direction !== defaultSort.direction) {
      params.set("sortField", sort.field);
      params.set("sortDirection", sort.direction);
    }

    if (page > DEFAULT_SEARCH_REQUEST.page!) {
      params.set("page", page.toString());
    }

    const paramsString = params.toString();
    replaceState(paramsString ? `?${paramsString}` : "", "");
  }

  let timeoutId: ReturnType<typeof setTimeout> | null = null;
  let currentController: AbortController | null = null;

  async function runSearchDebounced() {
    if (timeoutId) clearTimeout(timeoutId);
    timeoutId = setTimeout(async () => {
      const currentQuery = query;
      const currentSort = sort;
      const currentPage = page;

      if (currentController) currentController.abort("new search");
      currentController = new AbortController();
      loading = true;

      const request = { query: currentQuery, sort: currentSort, page: currentPage };
      try {
        const result = await searchContracts(request, undefined, currentController.signal);

        // Only update the search results if the result is from the current search
        if (
          query === currentQuery &&
          sort.direction === currentSort.direction &&
          sort.field === currentSort.field &&
          page === currentPage
        ) {
          searchResults = result;
          lastRequest = request;
          await updateUrl(currentQuery, currentSort, currentPage);
        }

        loading = false;
      } catch (error) {
        if (!(error instanceof Error) || error.name === "AbortError") {
          return;
        }
        console.error("Search error:", error);
        // TODO: show error
      }
    }, 150);
  }

  // Execute query whenever on each parameter change
  $effect(() => {
    const last = untrack(() => lastRequest);

    query;
    sort;
    page;

    // Only send the request if the parameters have changed
    // This also avoids double requesting on page load
    if (
      last.query !== query ||
      last.sort.direction !== sort.direction ||
      last.sort.field !== sort.field ||
      last.page !== page
    ) {
      runSearchDebounced();
    }
  });

  // On URL change, update the search parameters
  $effect(() => {
    const urlParams = sveltePage.url.searchParams;
    query = urlParams.get("query") || DEFAULT_SEARCH_REQUEST.query;

    const sortField = validateEnumOrDefault(
      urlParams.get("sortField"),
      Sort.fields,
      DEFAULT_SEARCH_REQUEST.sort!.field,
    );
    const sortDirection = validateEnumOrDefault(
      urlParams.get("sortDirection"),
      Sort.directions,
      DEFAULT_SEARCH_REQUEST.sort!.direction,
    );
    const pageParam = urlParams.get("page");

    page = pageParam ? parseInt(pageParam, 10) : DEFAULT_SEARCH_REQUEST.page!;
    page = untrack(() => Math.max(1, page));

    sort = { field: sortField, direction: sortDirection };
  });

  // Clamp page number to the maximum allowed value
  $effect(() => {
    if (page > searchResults.totalPages) {
      page = Math.max(1, searchResults.totalPages);
    }
  });
</script>

<svelte:head>
  <title>Contrato Público {query ? `- ${query}` : ""}</title>
</svelte:head>

<div class="space-y-4">
  <div class="text-2xl font-semibold">Procura por contratos públicos celebrados em Portugal</div>

  <div class="space-y-1">
    <Search bind:searchTerm={query} />

    <SortDropdown bind:sortBy={sort} />

    <div class="relative flex h-6 items-center">
      {#if loading}
        <div class="absolute" transition:fade={{ duration: 100 }}>
          <Skeleton class="h-[1em] w-[25em]" />
        </div>
      {:else}
        <p class="text-muted-foreground absolute" transition:fade={{ duration: 100 }}>
          {new Intl.NumberFormat("pt-PT").format(searchResults.total)}
          {searchResults.total === 1 ? "contrato encontrado" : "contratos encontrados"}
          em {searchResults.elapsedMillis}ms
        </p>
      {/if}
    </div>
  </div>

  {#if searchResults.totalPages > 1}
    <ContractPagination
      bind:page
      bind:total={searchResults.total}
      bind:hitsPerPage={searchResults.hitsPerPage} />
  {/if}

  {#each searchResults.contracts as contract (contract.id)}
    <div transition:fade={{ duration: 150 }}>
      <ContractCard {contract} />
    </div>
  {/each}

  <ContractPagination
    bind:page
    bind:total={searchResults.total}
    bind:hitsPerPage={searchResults.hitsPerPage} />
</div>
