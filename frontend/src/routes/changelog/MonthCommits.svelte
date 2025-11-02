<script lang="ts">
    import Link from "../../components/Link.svelte";

    const { startDate, endDate } = $props();

  const url = $derived(`https://github.com/chicoferreira/contratopublico/commits?since=${startDate}&until=${endDate}`);
</script>

<Link {url}>Commits</Link>