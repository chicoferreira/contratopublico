<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import Label from "$lib/components/ui/label/label.svelte";
  import { X } from "@lucide/svelte";
  import type { Snippet } from "svelte";
  import { fade } from "svelte/transition";
  import NewBadge from "$lib/components/NewBadge.svelte";

  type Props = {
    labelContent: string;
    addedDate?: string;
    descriptionContent: string;
    active: unknown;
    onClear: () => void;
    children: Snippet<[string]>;
  };

  let { labelContent, addedDate, descriptionContent, active, onClear, children }: Props = $props();

  let id = $props.id();
</script>

<div class="space-y-2">
  <div class="flex h-6 items-center gap-1">
    <Label for={id} class="text-sm font-medium">{labelContent}</Label>
    <NewBadge {addedDate} />
    {#if active}
      <div transition:fade={{ duration: 100 }}>
        <Button
          variant="ghost"
          size="icon"
          onclick={onClear}
          class="text-muted-foreground hover:text-foreground h-6 w-6 p-0">
          <X class="!h-4 !w-4" />
        </Button>
      </div>
    {/if}
  </div>
  {@render children(id)}
  <p class="text-muted-foreground text-xs">{descriptionContent}</p>
</div>
