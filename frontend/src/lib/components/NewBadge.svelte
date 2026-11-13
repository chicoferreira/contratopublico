<script lang="ts">
  import { Badge } from "$lib/components/ui/badge/index.js";

  const NEW_FOR_DAYS = 30;

  type Props = {
    addedDate?: string;
    link?: boolean;
  };

  let { addedDate, link = true }: Props = $props();

  const expiresAt = $derived(
    addedDate ? Date.parse(addedDate) + NEW_FOR_DAYS * 24 * 60 * 60 * 1000 : 0,
  );
</script>

{#if Date.now() < expiresAt}
  <Badge href={link ? "/changelog" : undefined}>Novo</Badge>
{/if}
