<script lang="ts">
  import type { ComponentType } from "svelte";

  let { Icon, label, value } = $props();
</script>

<div class="flex items-center gap-3">
  <Icon class="text-muted-foreground/70 h-6 w-6 shrink-0" />
  <div>
    <span class="text-muted-foreground/70 text-sm font-semibold uppercase">
      {label}
    </span>
    <div class="font-medium break-words">
      {value}
    </div>
  </div>
</div>
