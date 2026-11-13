<script lang="ts">
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import { cn } from "$lib/utils.js";
  import { ChevronRight } from "@lucide/svelte";
  import type { Snippet } from "svelte";
  import { blur, slide } from "svelte/transition";

  let {
    label,
    subtitle,
    leading,
    description,
    count,
    checked,
    indeterminate = false,
    onCheckedChange,
    nested = false,
    children,
  }: {
    label: string;
    subtitle?: string;
    leading: Snippet;
    description?: string;
    count?: string;
    checked: boolean;
    indeterminate?: boolean;
    onCheckedChange: (checked: boolean) => void;
    nested?: boolean;
    children?: Snippet;
  } = $props();

  let open = $state(false);
</script>

{#snippet reveal(content: Snippet)}
  <div transition:slide={{ duration: 200 }}>
    <div transition:blur={{ duration: 200 }}>
      {@render content()}
    </div>
  </div>
{/snippet}

{#snippet descriptionText()}
  <p class="text-muted-foreground pt-0.5">{description}</p>
{/snippet}

<div class="flex items-start gap-1">
  <DropdownMenu.CheckboxItem
    class="flex-1 cursor-pointer"
    closeOnSelect={false}
    {checked}
    {indeterminate}
    {onCheckedChange}>
    <div class="max-w-md">
      <div class={cn("flex items-center gap-1.5", !nested && "font-semibold")}>
        {@render leading()}
        {label}
        {#if subtitle}
          <span class="text-muted-foreground font-normal">· {subtitle}</span>
        {/if}
      </div>
      {#if description && open}
        {@render reveal(descriptionText)}
      {/if}
    </div>
  </DropdownMenu.CheckboxItem>
  <DropdownMenu.Item
    class="h-8 cursor-pointer gap-1 px-1.5 py-0"
    closeOnSelect={false}
    aria-expanded={open}
    aria-label="{open ? 'Esconder' : 'Mostrar'} {children ? 'variantes' : 'descrição'} de {label}"
    onSelect={() => (open = !open)}>
    {#if count}
      <span class="text-muted-foreground text-xs">{count}</span>
    {/if}
    <ChevronRight class={cn("transition-transform duration-200", open && "rotate-90")} />
  </DropdownMenu.Item>
</div>

{#if children && open}
  <div class="ml-6">
    {@render reveal(children)}
  </div>
{/if}
