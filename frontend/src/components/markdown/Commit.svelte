<script lang="ts">
  import { Badge } from "$lib/components/ui/badge/index.js";
  const { commit } = $props();

  const url = $derived(`https://github.com/chicoferreira/contratopublico/commit/${commit}`);
  const smallCommitId = $derived(commit.slice(0, 7));
</script>

<Badge variant="secondary" class="bg-blue-500 text-white dark:bg-blue-600">
  <a href={url} target="_blank" class="text-white hover:text-white/80">
    {smallCommitId}
  </a>
</Badge>

