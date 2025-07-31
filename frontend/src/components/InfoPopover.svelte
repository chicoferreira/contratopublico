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

<div class="w-fit max-w-sm space-y-0">
  <div class="text-base font-semibold">{title}</div>

  <div class="text-muted-foreground space-y-2 text-sm">
    {@render content()}
  </div>
</div>
