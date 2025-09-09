<script lang="ts">
  import { Badge } from "$lib/components/ui/badge/index.js";
  const { pr } = $props();

  const url = $derived(`https://github.com/chicoferreira/contratopublico/pull/${pr}`);
  const prId = $derived(`#${pr}`);
</script>

<Badge variant="secondary" class="bg-yellow-500 text-white dark:bg-yellow-600">
  <a href={url} target="_blank" class="text-white hover:text-white/80">
    {prId}
  </a>
</Badge>
