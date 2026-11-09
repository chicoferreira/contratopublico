<script lang="ts">
  import type { Component, Snippet } from "svelte";

  let {
    icon: Icon,
    title,
    children,
  }: { icon?: Component; title: Snippet; children: Snippet } = $props();
</script>

<div class="bg-muted/50 space-y-0.5 rounded-md border px-3 py-2">
  <div class="text-foreground flex items-center gap-1.5 font-semibold">
    {#if Icon}
      <Icon class="size-4" aria-hidden="true" />
    {/if}
    {@render title()}
  </div>
  <div class="space-y-1.5">{@render children()}</div>
</div>
