<script lang="ts">
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import { cn } from "$lib/utils.js";
  import { Scale } from "@lucide/svelte";
  import type { ProcedureType } from "$lib/types/api";
  import { COMPETITION, PROCEDURE_TYPES, type Competition } from "$lib/procedure-types";
  import FilterChip from "./FilterChip.svelte";
  import FilterMenu from "./FilterMenu.svelte";
  import FilterOption from "./FilterOption.svelte";

  let { value = $bindable() }: { value: ProcedureType[] | undefined } = $props();

  const keys = Object.keys(PROCEDURE_TYPES) as ProcedureType[];
  const competitions = Object.keys(COMPETITION) as Competition[];

  const groups = Object.entries(Object.groupBy(keys, (key) => PROCEDURE_TYPES[key].title)).map(
    ([title, types]) => ({
      title,
      types: types!,
      competitions: [...new Set(types!.map((t) => PROCEDURE_TYPES[t].competition))],
    }),
  );

  function count(types: ProcedureType[]) {
    return types.filter((t) => value?.includes(t)).length;
  }

  function set(types: ProcedureType[], checked: boolean) {
    const selected = keys.filter((k) => (types.includes(k) ? checked : value?.includes(k)));
    value = selected.length ? selected : undefined;
  }

  function selection(types: ProcedureType[]) {
    const selected = count(types);
    return {
      checked: selected === types.length,
      indeterminate: selected > 0 && selected < types.length,
      onCheckedChange: (checked: boolean) => set(types, checked),
    };
  }

  function variantLabel(type: ProcedureType) {
    return PROCEDURE_TYPES[type].subtitle ?? "Regime Geral";
  }

  const selectedChips = $derived(
    groups.flatMap((group) => {
      if (group.types.length > 1 && count(group.types) === group.types.length) {
        return [{ ...group, detail: "(todas)" }];
      }
      return group.types
        .filter((type) => value?.includes(type))
        .map((type) => ({
          title: group.title,
          types: [type],
          competitions: [PROCEDURE_TYPES[type].competition],
          detail:
            group.types.length > 1 || PROCEDURE_TYPES[type].subtitle
              ? `· ${variantLabel(type)}`
              : undefined,
        }));
    }),
  );
</script>

{#snippet dots(competitions: Competition[])}
  {#each competitions as competition (competition)}
    <span
      class={cn("size-2 shrink-0 rounded-full", COMPETITION[competition].dot)}
      aria-hidden="true"></span>
  {/each}
{/snippet}

<FilterMenu
  labelContent="Tipo de Procedimento"
  addedDate="2026-11-09"
  descriptionContent="Mostra contratos com um dos tipos de procedimento selecionados"
  IconComponent={Scale}
  active={!!value}
  onClear={() => (value = undefined)}>
  {#snippet chips()}
    {#each selectedChips as chip (chip.types[0])}
      <FilterChip label="{chip.title} {chip.detail ?? ''}" onRemove={() => set(chip.types, false)}>
        {@render dots(chip.competitions)}
        {chip.title}
        {#if chip.detail}
          <span class="text-muted-foreground font-normal">{chip.detail}</span>
        {/if}
      </FilterChip>
    {/each}
  {/snippet}
  <DropdownMenu.Label class="text-muted-foreground text-xs font-medium">
    Por concorrência
  </DropdownMenu.Label>
  {#each competitions as competition (competition)}
    <DropdownMenu.CheckboxItem
      class="cursor-pointer"
      closeOnSelect={false}
      {...selection(keys.filter((k) => PROCEDURE_TYPES[k].competition === competition))}>
      {@render dots([competition])}
      {COMPETITION[competition].label}
    </DropdownMenu.CheckboxItem>
  {/each}
  <DropdownMenu.Separator />
  <DropdownMenu.Label class="text-muted-foreground text-xs font-medium">
    Por procedimento
  </DropdownMenu.Label>
  {#each groups as group (group.title)}
    {#if group.types.length === 1}
      {@const type = group.types[0]}
      <FilterOption
        label={group.title}
        subtitle={PROCEDURE_TYPES[type].subtitle}
        description={PROCEDURE_TYPES[type].description}
        {...selection(group.types)}>
        {#snippet leading()}
          {@render dots(group.competitions)}
        {/snippet}
      </FilterOption>
    {:else}
      {@const selected = count(group.types)}
      <FilterOption
        label={group.title}
        count="{selected ? `${selected}/` : ''}{group.types.length}"
        {...selection(group.types)}>
        {#snippet leading()}
          {@render dots(group.competitions)}
        {/snippet}
        {#each group.types as type (type)}
          <FilterOption
            label={variantLabel(type)}
            nested
            description={PROCEDURE_TYPES[type].description}
            {...selection([type])}>
            {#snippet leading()}
              {@render dots([PROCEDURE_TYPES[type].competition])}
            {/snippet}
          </FilterOption>
        {/each}
      </FilterOption>
    {/if}
  {/each}
</FilterMenu>
