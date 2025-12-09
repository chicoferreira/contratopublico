<script lang="ts">
  import { cn } from "$lib/utils";

  const { children, class: className = "" } = $props();
</script>

<div class={cn("bg-card rounded-lg border px-5 py-4.5 shadow-sm", className)}>
  {@render children()}
</div>
