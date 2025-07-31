<script lang="ts">
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import { buttonVariants } from "$lib/components/ui/button/index.js";
  import { ChevronDown, ArrowUpDown } from "@lucide/svelte";
  import { cn } from "$lib/utils.js";
  import { Sort } from "$lib/types/api";

  let { sortBy = $bindable() }: { sortBy: Sort.SortBy } = $props();

  function getFieldLabel(sortBy: Sort.SortBy): string {
    switch (sortBy.field) {
      case "id":
        return "Identificador";
      case "publicationDate":
        return "Data de Publicação";
      case "signingDate":
        return "Data do Contrato";
      case "price":
        return "Preço";
    }
  }

  function getDirectionLabel(sortBy: Sort.SortBy): string {
    switch (sortBy.direction) {
      case "ascending":
        return "Ascendente";
      case "descending":
        return "Descendente";
    }
  }

  function getSortLabel(sortBy: Sort.SortBy): string {
    return `${getFieldLabel(sortBy)} (${getDirectionLabel(sortBy)})`;
  }

  function sortByEquals(a: Sort.SortBy, b: Sort.SortBy): boolean {
    return a.field === b.field && a.direction === b.direction;
  }
</script>

<DropdownMenu.Root>
  <DropdownMenu.Trigger
    class={cn(
      buttonVariants({ variant: "outline" }),
      "min-w-[200px] cursor-pointer justify-between",
    )}>
    <div class="flex items-center gap-2">
      <ArrowUpDown class="h-4 w-4" />
      {getSortLabel(sortBy)}
    </div>
    <ChevronDown class="h-4 w-4 opacity-50" />
  </DropdownMenu.Trigger>

  <DropdownMenu.Content class="w-[250px]" align="start">
    <DropdownMenu.Label>Ordenar por</DropdownMenu.Label>

    <DropdownMenu.Separator />
    {#each Sort.fields as field}
      {#each Sort.directions as direction}
        {@const option = { field, direction }}
        <DropdownMenu.Item
          onclick={() => (sortBy = option)}
          class={sortByEquals(sortBy, option) ? "bg-accent font-semibold" : ""}>
          {getSortLabel(option)}
        </DropdownMenu.Item>
      {/each}
    {/each}
  </DropdownMenu.Content>
</DropdownMenu.Root>
