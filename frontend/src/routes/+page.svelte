<script lang="ts">
  import "../app.css";

  import Search from "../components/Search.svelte";
  import ContractCard from "../components/ContractCard.svelte";
  import SortDropdown from "../components/SortDropdown.svelte";
  import ErrorDisplay from "../components/ErrorDisplay.svelte";
  import { blur, fade, scale, slide } from "svelte/transition";
  import { DEFAULT_SEARCH_REQUEST, parseSearchRequestFromParams, searchContracts } from "$lib";
  import ContractPagination from "../components/ContractPagination.svelte";
  import { replaceState } from "$app/navigation";
  import { page as sveltePage } from "$app/state";
  import { Sort } from "$lib/types/api";
  import { untrack } from "svelte";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import FiltersComponent from "../components/filter/FiltersComponent.svelte";
  import FiltersDropdown from "../components/filter/FiltersDropdown.svelte";
  import ContractsFound from "../components/ContractsFound.svelte";

  let { data } = $props();
  const {
    query: initialQuery,
    sort: initialSort,
    filters: initialFilters,
    page: initialPage,
  } = data.request;

  let query = $state(initialQuery);
  let sort = $state(initialSort);
  let page = $state(initialPage);
  let filters = $state(initialFilters);
  let searchResults = $state(data.response);

  let lastRequest = $state({
    query: initialQuery,
    sort: initialSort,
    page: initialPage,
    filters: initialFilters,
  });

  let error = $state<string | null>(data.error);
  let loading = $state(false);

  let filtersOpen = $state(false);

  const activeFiltersCount = $derived.by(
    () => Object.values(filters).filter((v) => v != null && v !== "").length,
  );

  async function updateUrl(query: string, sort: Sort.SortBy, page: number) {
    const params = new URLSearchParams();

    if (query) params.set("query", query);

    const defaultSort = DEFAULT_SEARCH_REQUEST.sort;
    if (sort.field !== defaultSort.field || sort.direction !== defaultSort.direction) {
      params.set("sortField", sort.field);
      params.set("sortDirection", sort.direction);
    }

    if (page > DEFAULT_SEARCH_REQUEST.page) {
      params.set("page", page.toString());
    }

    Object.entries(filters).forEach(([field, value]) => {
      if (value || value === 0) {
        params.set(field, value);
      }
    });

    const paramsString = params.toString();
    replaceState(paramsString ? `?${paramsString}` : "/", "");
  }

  let timeoutId: ReturnType<typeof setTimeout> | null = null;
  let currentController: AbortController | null = null;

  async function runSearchDebounced() {
    if (timeoutId) clearTimeout(timeoutId);
    timeoutId = setTimeout(async () => {
      const currentQuery = query;
      const currentSort = $state.snapshot(sort);
      const currentPage = page;
      const currentFilters = $state.snapshot(filters);

      if (currentController) currentController.abort("new search");
      currentController = new AbortController();
      loading = true;

      const request = {
        query: currentQuery,
        sort: currentSort,
        filters: currentFilters,
        page: currentPage,
      };
      try {
        const result = await searchContracts(request, undefined, currentController.signal);

        // Only update the search results if the result is from the current search
        if (
          query === currentQuery &&
          sort.direction === currentSort.direction &&
          sort.field === currentSort.field &&
          page === currentPage &&
          JSON.stringify(filters) === JSON.stringify(currentFilters)
        ) {
          searchResults = result;
          lastRequest = request;
          await updateUrl(currentQuery, currentSort, currentPage);
          // Clear any previous errors on successful request
          error = null;
        }

        loading = false;
      } catch (err) {
        loading = false;
        if (!(err instanceof Error) || err.name === "AbortError") {
          return;
        }
        console.error("Search error:", err);

        error = err.message || "Erro desconhecido";
      }
    }, 150);
  }

  // Execute query whenever on each parameter change
  $effect(() => {
    const last = untrack(() => lastRequest);

    query;
    sort;
    page;
    filters;

    // Only send the request if the parameters have changed
    // This also avoids double requesting on page load
    if (
      last.query !== query ||
      last.sort.direction !== sort.direction ||
      last.sort.field !== sort.field ||
      last.page !== page ||
      JSON.stringify(last.filters) !== JSON.stringify(filters)
    ) {
      runSearchDebounced();
    }
  });

  // On URL change, update the search parameters
  $effect(() => {
    const urlParams = sveltePage.url.searchParams;
    const request = parseSearchRequestFromParams(urlParams);

    query = request.query;
    sort = request.sort;
    page = request.page;
    filters = request.filters;
  });

  // Clamp page number to the maximum allowed value
  $effect(() => {
    if (searchResults && page > searchResults.totalPages) {
      page = Math.max(1, searchResults.totalPages);
    }
  });
</script>

<svelte:head>
  <title>Contrato Público {query ? `- ${query}` : ""}</title>
</svelte:head>

<div class="space-y-4">
  <div class="text-2xl font-semibold">Procura por contratos públicos celebrados em Portugal</div>

  <div class="space-y-2">
    <Search bind:searchTerm={query} />

    <div class="flex flex-wrap items-center gap-2">
      <SortDropdown bind:sortBy={sort} />
      <FiltersDropdown bind:filtersOpen {activeFiltersCount} />
    </div>

    {#if filtersOpen}
      <div transition:slide={{ duration: 200 }}>
        <div transition:blur={{ duration: 200 }}>
          <FiltersComponent bind:filters {activeFiltersCount} />
        </div>
      </div>
    {/if}

    <ContractsFound {loading} {searchResults} />
  </div>

  {#if error}
    <ErrorDisplay message={error} />
  {:else}
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
  {/if}
</div>
