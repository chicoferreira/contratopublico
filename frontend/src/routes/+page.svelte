<script lang="ts">
    import "../app.css";
    import { Icon, MagnifyingGlass } from "svelte-hero-icons";
    import type { SearchContractsResponse } from "$lib/types/api";

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

<label class="input w-full">
    <Icon src={MagnifyingGlass} micro class="h-[1em] w-[1em] opacity-50" />
    <input
        type="search"
        class="grow"
        placeholder="Procurar"
        bind:value={search}
    />
</label>

{JSON.stringify(searchResults)}
