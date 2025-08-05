<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import Input from "$lib/components/ui/input/input.svelte";
  import Label from "$lib/components/ui/label/label.svelte";
  import { X } from "@lucide/svelte";
  import { fade } from "svelte/transition";

  let {
    value = $bindable(),
    labelContent,
    IconComponent,
    descriptionContent,
    onClear,
    ...extraInputClasses
  } = $props();

  let id = $props.id();
</script>

<div class="space-y-2">
  <div class="flex h-6 items-center gap-1">
    <Label for={id} class="text-sm font-medium">{labelContent}</Label>
    {#if value}
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
  <div class="relative">
    <IconComponent
      class="text-muted-foreground pointer-events-none absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2" />
    <Input bind:value {...extraInputClasses} {id} class="h-9 pl-10" />
  </div>
  <p class="text-muted-foreground text-xs">{descriptionContent}</p>
</div>
