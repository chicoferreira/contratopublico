<script lang="ts">
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import { buttonVariants } from "$lib/components/ui/button/index.js";
  import { ChevronDown, ArrowUpDown } from "@lucide/svelte";
  import { cn } from "$lib/utils.js";
  import type { SortBy as SortByType } from "$lib/types/api";

  let { sortBy = $bindable() }: { sortBy: SortByType } = $props();

  const sortOptions = [
    {
      field: "publicationDate" as const,
      direction: "descending" as const,
      label: "Data de Publicação (Mais Recente)",
    },
    {
      field: "publicationDate" as const,
      direction: "ascending" as const,
      label: "Data de Publicação (Mais Antiga)",
    },
    {
      field: "signingDate" as const,
      direction: "descending" as const,
      label: "Data do Contrato (Mais Recente)",
    },
    {
      field: "signingDate" as const,
      direction: "ascending" as const,
      label: "Data do Contrato (Mais Antiga)",
    },
    {
      field: "price" as const,
      direction: "descending" as const,
      label: "Preço (Maior para Menor)",
    },
    {
      field: "price" as const,
      direction: "ascending" as const,
      label: "Preço (Menor para Maior)",
    },
    {
      field: "id" as const,
      direction: "descending" as const,
      label: "ID (Decrescente)",
    },
    {
      field: "id" as const,
      direction: "ascending" as const,
      label: "ID (Crescente)",
    },
  ];

  function getCurrentSortLabel(): string {
    if (!sortBy) return "Ordenar por";

    const option = sortOptions.find(
      (opt) => opt.field === sortBy.field && opt.direction === sortBy.direction,
    );

    return option?.label || "Ordenar por";
  }

  function handleSortSelect(option: (typeof sortOptions)[0]) {
    sortBy = {
      field: option.field,
      direction: option.direction,
    };
  }
</script>

<DropdownMenu.Root>
  <DropdownMenu.Trigger 
    class={cn(
      buttonVariants({ variant: "outline" }), 
      "justify-between min-w-[200px]"
    )}
  >
    <div class="flex items-center gap-2">
      <ArrowUpDown class="h-4 w-4" />
      {getCurrentSortLabel()}
    </div>
    <ChevronDown class="h-4 w-4 opacity-50" />
  </DropdownMenu.Trigger>

  <DropdownMenu.Content class="w-[250px]" align="start">
    <DropdownMenu.Label>Ordenar por</DropdownMenu.Label>

    {#each sortOptions as option}
      <DropdownMenu.Item
        onclick={() => handleSortSelect(option)}
        class={sortBy?.field === option.field &&
        sortBy?.direction === option.direction
          ? "bg-accent"
          : ""}
      >
        {option.label}
      </DropdownMenu.Item>
    {/each}
  </DropdownMenu.Content>
</DropdownMenu.Root>
