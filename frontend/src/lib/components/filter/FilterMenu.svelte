<script lang="ts">
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import { buttonVariants } from "$lib/components/ui/button/index.js";
  import { cn } from "$lib/utils.js";
  import { ChevronDown } from "@lucide/svelte";
  import type { Component, Snippet } from "svelte";
  import FilterField from "./FilterField.svelte";

  let {
    labelContent,
    descriptionContent,
    IconComponent,
    active,
    onClear,
    chips,
    children: menu,
  }: {
    labelContent: string;
    descriptionContent: string;
    IconComponent: Component;
    active: boolean;
    onClear: () => void;
    chips: Snippet;
    children: Snippet;
  } = $props();
</script>

<FilterField {labelContent} {descriptionContent} {active} {onClear}>
  {#snippet children(id)}
    <DropdownMenu.Root>
      <div
        class={cn(
          buttonVariants({ variant: "outline" }),
          "relative h-auto min-h-9 w-full flex-wrap justify-start py-1.5 font-normal",
        )}>
        <DropdownMenu.Trigger {id} class="absolute inset-0 cursor-pointer rounded-md" />
        <IconComponent class="text-muted-foreground" />
        {#if active}
          {@render chips()}
        {:else}
          <span class="text-muted-foreground">Todos os tipos</span>
        {/if}
        <ChevronDown class="ml-auto opacity-50" />
      </div>
      <DropdownMenu.Content
        align="start"
        class="w-[min(28rem,var(--bits-dropdown-menu-content-available-width))]">
        {@render menu()}
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  {/snippet}
</FilterField>
