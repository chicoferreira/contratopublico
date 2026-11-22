<script lang="ts">
  import Highlighted from "$lib/components/Highlighted.svelte";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { MAX_LOCATION_FILTERS } from "$lib";
  import { loadLocationTree, placeDetail, placeKind, PORTUGAL } from "$lib/locations";
  import { nameMatchRanges, searchByName, type TreeNode } from "$lib/tree";
  import { MapPin } from "@lucide/svelte";
  import FilterTree from "./FilterTree.svelte";

  let { value = $bindable() }: { value: string[] | undefined } = $props();

  function placeLabel(node: TreeNode) {
    const detail = placeDetail(node);
    return detail ? `${node.name} ${detail}` : node.name;
  }
</script>

<FilterTree
  bind:value
  max={MAX_LOCATION_FILTERS}
  load={loadLocationTree}
  search={searchByName}
  labelContent="Locais"
  addedDate="2026-11-22"
  descriptionContent="Mostra contratos com pelo menos um dos locais selecionados ou dos seus subníveis, sem incluir locais mais abrangentes nem considerar a sede das entidades"
  IconComponent={MapPin}
  placeholder="País, distrito ou concelho (ex: Espanha, Oeiras)..."
  treeLabel="Locais de execução"
  countLabel="locais"
  loadingText="A carregar locais..."
  errorText="Não foi possível carregar os locais."
  emptyText="Nenhum local encontrado."
  initiallyExpanded={[PORTUGAL]}
  nodeLabel={placeLabel}>
  {#snippet row(node, query, matched)}
    {@const kind = placeKind(node)}
    <Highlighted content={node.name} ranges={matched ? nameMatchRanges(node.name, query) : []} />
    {#if kind}
      <span class="text-muted-foreground">{kind}</span>
    {/if}
  {/snippet}
  {#snippet chip(id, node, loading)}
    {@const detail = node ? placeDetail(node) : undefined}
    {#if loading}
      <Skeleton class="bg-muted-foreground/20 h-3 w-16" />
    {:else}
      <span class="truncate">{node?.name ?? id}</span>
      {#if detail}
        <span class="text-muted-foreground shrink-0 font-normal">{detail}</span>
      {/if}
    {/if}
  {/snippet}
</FilterTree>
