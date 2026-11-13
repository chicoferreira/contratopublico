<script lang="ts">
  import { Badge } from "$lib/components/ui/badge/index.js";
  import { isRecentFeature } from "$lib";

  type Props = {
    addedDate?: string;
    link?: boolean;
  };

  let { addedDate, link = true }: Props = $props();
</script>

{#if addedDate && isRecentFeature(addedDate)}
  <Badge href={link ? "/changelog" : undefined}>Novo</Badge>
{/if}
