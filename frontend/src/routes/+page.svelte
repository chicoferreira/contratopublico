<script lang="ts">
  import "../app.css";

  import Search from "../components/Search.svelte";
  import ContractCard from "../components/ContractCard.svelte";
  import SortDropdown from "../components/SortDropdown.svelte";
  import ErrorDisplay from "../components/ErrorDisplay.svelte";
  import { blur, fade, slide } from "svelte/transition";
  import { DEFAULT_SEARCH_REQUEST, parseSearchRequestFromParams, searchContracts } from "$lib";
  import ContractPagination from "../components/ContractPagination.svelte";
  import { replaceState } from "$app/navigation";
  import { page as sveltePage } from "$app/state";
  import { Sort } from "$lib/types/api";
  import { untrack } from "svelte";
  import FiltersComponent from "../components/filter/FiltersComponent.svelte";
  import FiltersDropdown from "../components/filter/FiltersDropdown.svelte";
  import ContractsFound from "../components/ContractsFound.svelte";
  import StatisticsInsights from "../components/StatisticsInsights.svelte";
  import { TriangleAlert } from "@lucide/svelte";

  let { data } = $props();
  const {
    query: initialQuery,
    sort: initialSort,
    filters: initialFilters,
    page: initialPage,
  } = data.searchRequest;

  let statistics = data.statisticsResponse;

  let query = $state(initialQuery);
  let sort = $state(initialSort);
  let page = $state(initialPage);
  let filters = $state(initialFilters);
  let searchResults = $state(data.searchResponse);

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
    filters.minId;
    filters.maxId;
    filters.startPublicationDate;
    filters.endPublicationDate;
    filters.startSigningDate;
    filters.endSigningDate;
    filters.contracted;
    filters.contracting;
    filters.minPrice;
    filters.maxPrice;

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
  <!-- TODO: REMOVE -->
  <div
    class="mb-4 rounded-xl border border-yellow-200 bg-yellow-50 p-4 dark:border-yellow-800/30 dark:bg-yellow-950/20">
    <div class="flex items-start gap-3">
      <div class="mt-0.5 rounded-full bg-yellow-100 p-1.5 dark:bg-yellow-900/40">
        <TriangleAlert class="h-4 w-4 text-yellow-600 dark:text-yellow-400" />
      </div>
      <div class="flex-1 space-y-1">
        <div class="flex items-center gap-2">
          <span class="font-semibold text-yellow-800 dark:text-yellow-200">
            Alerta Reindexação da Base de Dados
          </span>
        </div>
        <p class="text-sm text-yellow-700 dark:text-yellow-200">
          Estamos a reindexar a base de dados do Portal BASE para incluir mais detalhes sobre os
          contratos, que ficarão disponíveis gradualmente à medida que são processados. Este
          processo poderá demorar alguns dias.
        </p>
      </div>
    </div>
  </div>

  <div class="space-y-2.5">
    <div class="space-y-0.5">
      <div class="text-2xl font-semibold">
        Procura por contratos públicos celebrados em Portugal
      </div>
      <StatisticsInsights {statistics} />
    </div>
    <Search bind:searchTerm={query} />

    <div class="flex flex-wrap justify-between gap-y-0.5">
      <div class="flex flex-wrap gap-2">
        <SortDropdown bind:sortBy={sort} />
        <FiltersDropdown bind:filtersOpen {activeFiltersCount} />
      </div>
    </div>

    {#if filtersOpen}
      <div transition:slide={{ duration: 200 }}>
        <div transition:blur={{ duration: 200 }}>
          <FiltersComponent bind:filters {activeFiltersCount} />
        </div>
      </div>
    {/if}

    <div data-change-page-scroll-target></div>

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
      bind:hitsPerPage={searchResults.hitsPerPage}
      scrolToElement="[data-change-page-scroll-target]"
      scrollOffset={-10} />
  {/if}
</div>
