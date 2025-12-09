<script lang="ts">
  import { Separator } from "$lib/components/ui/separator";
  import GridCard from "./GridCard.svelte";

  const { icon: CardIcon, title, compact = false, children } = $props();
</script>

<GridCard class={compact ? "" : "space-y-3"}>
  <div class="text-muted-foreground flex items-center gap-2 text-sm font-semibold uppercase">
    <CardIcon class="h-4 w-4" />
    {title}
  </div>
  {#if !compact}
    <Separator />
  {/if}
  {@render children()}
</GridCard>
