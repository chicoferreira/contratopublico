<script lang="ts">
  import Link from "$lib/components/Link.svelte";

  const { startDate, endDate } = $props();

  const url = $derived(
    `https://github.com/chicoferreira/contratopublico/commits?since=${startDate}&until=${endDate}`,
  );
</script>

<Link {url}>Commits</Link>
