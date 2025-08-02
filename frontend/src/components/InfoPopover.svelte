<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    title,
    content,
  }: {
    title: string;
    content: Snippet<[]>;
  } = $props();
</script>

<div class="max-w-xs space-y-0 break-words sm:max-w-sm md:max-w-md">
  <div class="text-base font-semibold">{title}</div>

  <div class="text-muted-foreground space-y-2 text-sm">
    {@render content()}
  </div>
</div>
