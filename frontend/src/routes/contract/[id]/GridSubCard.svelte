<script lang="ts">
  import { cn } from "$lib/utils";

  const { children, class: className = "" } = $props();
</script>

<div class={cn(className, "bg-muted/50 rounded-lg px-4 py-3.5")}>
  {@render children()}
</div>
