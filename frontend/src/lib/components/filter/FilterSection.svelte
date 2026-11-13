<script lang="ts">
  import { Badge } from "$lib/components/ui/badge/index.js";
  import * as Collapsible from "$lib/components/ui/collapsible/index.js";
  import { ChevronDown, X } from "@lucide/svelte";
  import NewBadge from "$lib/components/NewBadge.svelte";
  import type { Filters } from "$lib/types/api";
  import { untrack, type Component } from "svelte";
  import { blur, fade, slide } from "svelte/transition";

  type Props = {
    title: string;
    IconComponent: Component;
    filter: Filters;
    fields: (keyof Filters)[];
    addedDate?: string;
    children: any;
  };

  let { title, IconComponent, filter, fields, addedDate, children }: Props = $props();

  let hasActiveFilters = $derived(fields.some((field) => filter[field] !== undefined));
  let isOpen = $state(untrack(() => hasActiveFilters));
</script>

<Collapsible.Root bind:open={isOpen}>
  <Collapsible.Trigger
    class="group flex w-full cursor-pointer items-center justify-between rounded-lg transition-colors">
    <div class="flex items-center gap-2">
      <IconComponent class="text-muted-foreground h-4 w-4" />
      <span class="font-medium group-hover:underline">{title}</span>
      <NewBadge {addedDate} link={false} />
      {#if hasActiveFilters}
        <div transition:fade={{ duration: 100 }}>
          <Badge variant="secondary" class="hidden md:inline-flex">Com filtros</Badge>
        </div>
      {/if}
    </div>
    <div class="flex items-center gap-2">
      <ChevronDown
        class="text-muted-foreground h-4 w-4 transition-transform {isOpen ? 'rotate-180' : ''}" />
    </div>
  </Collapsible.Trigger>
  <Collapsible.Content forceMount class="px-6 pb-1">
    {#snippet child({ props, open })}
      {#if open}
        <div transition:slide={{ duration: 200 }}>
          <div transition:blur={{ duration: 200 }}>
            <div class="px-6 pt-4">
              {@render children()}
            </div>
          </div>
        </div>
      {/if}
    {/snippet}
  </Collapsible.Content>
</Collapsible.Root>
