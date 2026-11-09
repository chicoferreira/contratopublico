<script lang="ts">
  import { Gavel } from "@lucide/svelte";
  import { CONTRACT_TYPES } from "$lib/contract-types";
  import type { ContractType } from "$lib/types/api";
  import FilterChip from "./FilterChip.svelte";
  import FilterMenu from "./FilterMenu.svelte";
  import FilterOption from "./FilterOption.svelte";

  let { value = $bindable() }: { value: ContractType[] | undefined } = $props();

  const types = Object.keys(CONTRACT_TYPES) as ContractType[];

  function toggle(type: ContractType, checked: boolean) {
    const selected = types.filter((key) => (key === type ? checked : value?.includes(key)));
    value = selected.length ? selected : undefined;
  }
</script>

<FilterMenu
  labelContent="Tipo de Contrato"
  descriptionContent="Mostra contratos com pelo menos um dos tipos de contrato selecionados"
  IconComponent={Gavel}
  active={!!value}
  onClear={() => (value = undefined)}>
  {#snippet chips()}
    {#each value ?? [] as type (type)}
      {@const { label, icon: Icon } = CONTRACT_TYPES[type]}
      <FilterChip {label} onRemove={() => toggle(type, false)}>
        <Icon aria-hidden="true" />
        {label}
      </FilterChip>
    {/each}
  {/snippet}
  {#each types as type (type)}
    {@const Icon = CONTRACT_TYPES[type].icon}
    <FilterOption
      label={CONTRACT_TYPES[type].label}
      description={CONTRACT_TYPES[type].description}
      checked={value?.includes(type) ?? false}
      onCheckedChange={(checked) => toggle(type, checked)}>
      {#snippet leading()}
        <Icon class="size-4" aria-hidden="true" />
      {/snippet}
    </FilterOption>
  {/each}
</FilterMenu>
