<script lang="ts" generics="T">
  const {
    title,
    elements,
    fallback,
    renderElement,
  }: {
    title?: string;
    elements: T[];
    fallback: string;
    renderElement: (elem: T) => any;
  } = $props();
</script>

<div class="space-y-2">
  {#if title}
    <p class="text-muted-foreground text-xs font-semibold uppercase">
      {title}
      {#if elements.length > 1}
        ({elements.length})
      {/if}
    </p>
  {/if}
  {#if elements.length}
    <div class="space-y-2">
      {#each elements as elem}
        {@render renderElement(elem)}
      {/each}
    </div>
  {:else}
    <p class="text-muted-foreground text-sm">{fallback}</p>
  {/if}
</div>
