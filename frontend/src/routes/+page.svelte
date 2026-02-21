<script lang="ts">
  import "../app.css";

  import Search from "$lib/components/Search.svelte";
  import ContractCard from "$lib/components/ContractCard.svelte";
  import SortDropdown from "$lib/components/SortDropdown.svelte";
  import ErrorDisplay from "$lib/components/ErrorDisplay.svelte";
  import { blur, fade, slide } from "svelte/transition";
  import { buildSearchParams } from "$lib";
  import ContractPagination from "$lib/components/ContractPagination.svelte";
  import { goto } from "$app/navigation";
  import { page as sveltePage } from "$app/state";
  import { type SearchContractsRequest } from "$lib/types/api";
  import { untrack } from "svelte";
  import FiltersComponent from "$lib/components/filter/FiltersComponent.svelte";
  import FiltersDropdown from "$lib/components/filter/FiltersDropdown.svelte";
  import ContractsFound from "$lib/components/ContractsFound.svelte";
  import StatisticsInsights from "$lib/components/StatisticsInsights.svelte";

  let { data } = $props();
  const getInitialSearchRequest = () => data.searchRequest;
  const getInitialSearchResponse = () => data.searchResponse;
  const getInitialStatistics = () => data.statisticsResponse;
  const getInitialError = () => data.error;

  const initialRequest = getInitialSearchRequest();

  let query = $state(initialRequest.query);
  let sort = $state(initialRequest.sort);
  let page = $state(initialRequest.page);
  let filters = $state(initialRequest.filters);
  let searchResults = $state(getInitialSearchResponse());
  let statistics = $state(getInitialStatistics());
  let error = $state<string | null>(getInitialError());

  const activeFiltersCount = $derived.by(
    () => Object.values(filters).filter((v) => v != null && v !== "").length,
  );

  let loading = $state(false);

  const initialActiveFiltersCount = () => activeFiltersCount;
  let filtersOpen = $state(initialActiveFiltersCount() > 0);

  let navigationTimeout: ReturnType<typeof setTimeout> | null = null;
  let lastNavigatedSearch = $state("");

  function snapshotRequest(): Required<SearchContractsRequest> {
    return {
      query,
      sort: $state.snapshot(sort),
      page,
      filters: $state.snapshot(filters),
    };
  }

  // Keep local mutable state synced with readonly `page.data`.
  $effect(() => {
    const request = data.searchRequest;
    query = request.query;
    sort = request.sort;
    page = request.page;
    filters = request.filters;
    searchResults = data.searchResponse;
    statistics = data.statisticsResponse;
    error = data.error;
    loading = false;
    lastNavigatedSearch = sveltePage.url.searchParams.toString();
  });

  // User edits update URL; URL updates rerun `load`, which refreshes `page.data`.
  $effect(() => {
    query;
    sort.field;
    sort.direction;
    page;
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

    const nextSearch = buildSearchParams(snapshotRequest()).toString();
    const currentSearch = untrack(() => sveltePage.url.searchParams.toString());
    const lastSubmitted = untrack(() => lastNavigatedSearch);

    if (nextSearch === currentSearch || nextSearch === lastSubmitted) {
      return;
    }

    if (navigationTimeout) clearTimeout(navigationTimeout);
    navigationTimeout = setTimeout(async () => {
      const latestSearch = buildSearchParams(snapshotRequest()).toString();
      const latestCurrent = sveltePage.url.searchParams.toString();

      if (latestSearch === latestCurrent) return;

      loading = true;
      lastNavigatedSearch = latestSearch;

      try {
        await goto(latestSearch ? `?${latestSearch}` : "/", {
          replaceState: true,
          keepFocus: true,
          noScroll: true,
        });
      } catch (navigationError) {
        loading = false;
        console.error("Navigation error:", navigationError);
      }
    }, 250);
  });

  // Keep page inside available bounds from latest results.
  $effect(() => {
    if (searchResults.totalPages > 0 && page > searchResults.totalPages) {
      page = searchResults.totalPages;
    }
    if (page < 1) {
      page = 1;
    }
  });
</script>

<svelte:head>
  <title>Contrato Público {query ? `- ${query}` : ""}</title>
</svelte:head>

<div class="space-y-4">
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
          <FiltersComponent bind:filters {activeFiltersCount} bind:filtersOpen />
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
        total={searchResults.total}
        hitsPerPage={searchResults.hitsPerPage} />
    {/if}

    {#each searchResults.contracts as contract (contract.id)}
      <div transition:fade={{ duration: 150 }}>
        <ContractCard {contract} />
      </div>
    {/each}

    <ContractPagination
      bind:page
      total={searchResults.total}
      hitsPerPage={searchResults.hitsPerPage}
      scrolToElement="[data-change-page-scroll-target]"
      scrollOffset={-10} />
  {/if}
</div>
