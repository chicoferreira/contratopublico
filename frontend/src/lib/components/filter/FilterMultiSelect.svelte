<script lang="ts" generics="T extends string">
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import { buttonVariants } from "$lib/components/ui/button/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import { cn } from "$lib/utils.js";
  import { ChevronDown, X } from "@lucide/svelte";
  import type { Component } from "svelte";
  import FilterField from "./FilterField.svelte";

  type Props = {
    value: T[] | undefined;
    options: Record<T, { label: string; description?: string }>;
    labelContent: string;
    descriptionContent: string;
    placeholder: string;
    IconComponent: Component;
  };

  let {
    value = $bindable(),
    options,
    labelContent,
    descriptionContent,
    placeholder,
    IconComponent,
  }: Props = $props();

  const keys = $derived(Object.keys(options) as T[]);

  function toggle(option: T, checked: boolean) {
    const selected = keys.filter((o) => (o === option ? checked : value?.includes(o)));
    value = selected.length ? selected : undefined;
  }
</script>

<FilterField {labelContent} {descriptionContent} active={value} onClear={() => (value = undefined)}>
  {#snippet children(id)}
    <DropdownMenu.Root>
      <div
        class={cn(
          buttonVariants({ variant: "outline" }),
          "relative h-auto min-h-9 w-full flex-wrap justify-start py-1.5 font-normal",
        )}>
        <DropdownMenu.Trigger {id} class="absolute inset-0 cursor-pointer rounded-md" />
        <IconComponent class="text-muted-foreground" />
        {#each value ?? [] as v}
          <Badge class="h-6 pr-1">
            {options[v].label}
            <button
              type="button"
              aria-label="Remover {options[v].label}"
              class="relative cursor-pointer opacity-60 hover:opacity-100"
              onclick={() => toggle(v, false)}>
              <X class="size-3" />
            </button>
          </Badge>
        {/each}
        {#if !value}
          <span class="text-muted-foreground">{placeholder}</span>
        {/if}
        <ChevronDown class="ml-auto opacity-50" />
      </div>
      <DropdownMenu.Content align="start">
        {#each keys as option}
          <DropdownMenu.CheckboxItem
            class="cursor-pointer"
            closeOnSelect={false}
            checked={value?.includes(option) ?? false}
            onCheckedChange={(checked) => toggle(option, checked)}>
            <div class="max-w-md space-y-0.5">
              <div class="flex items-center gap-1.5 font-semibold">
                <IconComponent class="size-4" />
                {options[option].label}
              </div>
              {#if options[option].description}
                <p class="text-muted-foreground">{options[option].description}</p>
              {/if}
            </div>
          </DropdownMenu.CheckboxItem>
        {/each}
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  {/snippet}
</FilterField>
