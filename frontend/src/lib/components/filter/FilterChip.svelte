<script lang="ts">
  import { Badge } from "$lib/components/ui/badge/index.js";
  import { X } from "@lucide/svelte";
  import type { Snippet } from "svelte";

  let { label, onRemove, children }: { label: string; onRemove: () => void; children: Snippet } =
    $props();
</script>

<Badge variant="secondary" class="border-border h-6 pr-1">
  {@render children()}
  <button
    type="button"
    aria-label="Remover {label}"
    class="relative cursor-pointer opacity-60 hover:opacity-100"
    onclick={onRemove}>
    <X class="size-3" />
  </button>
</Badge>
