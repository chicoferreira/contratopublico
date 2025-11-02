<script lang="ts">
  const { children } = $props();
</script>

<span class="text-muted-foreground">
  {@render children?.()}
</span>
