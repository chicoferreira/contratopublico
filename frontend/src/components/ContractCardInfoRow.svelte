<script lang="ts">
  import type { ComponentType } from "svelte";

  let { Icon, label, value } = $props();
</script>

<div class="flex items-center gap-3">
  <Icon class="h-5 w-5 text-base-content/60" />
  <div>
    <span class="text-base-content/70">{label}:</span>
    <div class="font-medium text-base-content">
      {value}
    </div>
  </div>
</div>
