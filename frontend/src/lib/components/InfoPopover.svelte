<script lang="ts">
  let { title, children } = $props();
</script>

<div class="max-w-xs space-y-0 break-words sm:max-w-sm md:max-w-md">
  <div class="text-base font-semibold">{title}</div>

  <div class="text-muted-foreground space-y-2 text-sm">
    {@render children?.()}
  </div>
</div>
