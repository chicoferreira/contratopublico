<script lang="ts">
  import { Badge } from "$lib/components/ui/badge/index.js";
  import * as Collapsible from "$lib/components/ui/collapsible/index.js";
  import Button from "$lib/components/ui/button/button.svelte";
  import { ChevronDown, X } from "@lucide/svelte";
  import type { Filters } from "$lib/types/api";
  import type { Component } from "svelte";
  import { blur, fade, slide } from "svelte/transition";

  type Props = {
    title: string;
    IconComponent: Component;
    filter: Filters;
    fields: (keyof Filters)[];
    children: any;
  };

  let { title, IconComponent, filter, fields, children }: Props = $props();

  let hasActiveFilters = $derived(fields.some((field) => filter[field] !== undefined));
  // svelte-ignore state_referenced_locally
  let isOpen = $state(hasActiveFilters);
</script>

<Collapsible.Root bind:open={isOpen}>
  <Collapsible.Trigger
    class="group flex w-full items-center justify-between rounded-lg transition-colors hover:cursor-pointer">
    <div class="flex items-center gap-2">
      <IconComponent class="text-muted-foreground h-4 w-4" />
      <span class="font-medium group-hover:underline">{title}</span>
      {#if hasActiveFilters}
        <div transition:fade={{ duration: 100 }}>
          <Badge variant="secondary" class="hidden md:block">Com filtros</Badge>
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
            <div class="pt-4">
              {@render children()}
            </div>
          </div>
        </div>
      {/if}
    {/snippet}
  </Collapsible.Content>
</Collapsible.Root>
