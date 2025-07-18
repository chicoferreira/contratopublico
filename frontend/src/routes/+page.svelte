<script lang="ts">
    import type { SearchContractsResponse } from "$lib/types/api";

    let { data } = $props();
</script>

{JSON.stringify(data)}
