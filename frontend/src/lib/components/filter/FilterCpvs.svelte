<script lang="ts">
  import Highlighted from "$lib/components/Highlighted.svelte";
  import { MAX_CPV_FILTERS } from "$lib";
  import { codeMatchRanges, designationMatchRanges, loadCpvTree, searchCpvs } from "$lib/cpv";
  import { Tag } from "@lucide/svelte";
  import FilterTree from "./FilterTree.svelte";

  let { value = $bindable() }: { value: string[] | undefined } = $props();
</script>

<FilterTree
  bind:value
  max={MAX_CPV_FILTERS}
  load={loadCpvTree}
  search={searchCpvs}
  labelContent="CPV (Vocabulário Comum para os Contratos Públicos)"
  addedDate="2026-11-13"
  descriptionContent="Mostra contratos com pelo menos um dos códigos selecionados ou das suas subcategorias"
  IconComponent={Tag}
  placeholder="Designação ou código (ex: obras, 45000000)..."
  treeLabel="CPVs"
  countLabel="códigos"
  loadingText="A carregar CPVs..."
  errorText="Não foi possível carregar os CPVs."
  emptyText="Nenhum CPV encontrado.">
  {#snippet row(node, query, matched)}
    <Highlighted
      class="font-semibold tabular-nums"
      content={node.id}
      ranges={codeMatchRanges(node.id, query)} />
    <Highlighted
      content={node.name}
      ranges={matched ? designationMatchRanges(node.name, query) : []} />
  {/snippet}
  {#snippet chip(code, node)}
    {code}
    {#if node}
      <span class="text-muted-foreground max-w-48 truncate font-normal">{node.name}</span>
    {/if}
  {/snippet}
</FilterTree>
