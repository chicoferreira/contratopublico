<script lang="ts">
  import type { MatchingRange, ProcedureType } from "$lib/types/api";
  import { cn } from "$lib/utils";
  import Highlighted from "./Highlighted.svelte";
  import InfoBadge from "./InfoBadge.svelte";
  import InfoPopoverItem from "./InfoPopoverItem.svelte";
  import Link from "./Link.svelte";
  import { COMPETITION, PROCEDURE_TYPES } from "$lib/procedure-types";

  let {
    type,
    titleRanges,
    subtitleRanges,
  }: { type: ProcedureType; titleRanges?: MatchingRange[]; subtitleRanges?: MatchingRange[] } =
    $props();

  const procedureType = $derived(PROCEDURE_TYPES[type]);
</script>

{#snippet dot(color: string)}
  <span class={cn("size-2 shrink-0 rounded-full", color)} aria-hidden="true"></span>
{/snippet}

{#snippet name()}
  {@render dot(COMPETITION[procedureType.competition].dot)}
  <Highlighted class="truncate" content={procedureType.title} ranges={titleRanges} />
  {#if procedureType.subtitle}
    <span class="text-muted-foreground truncate font-normal">
      · <Highlighted content={procedureType.subtitle} ranges={subtitleRanges} />
    </span>
  {/if}
{/snippet}

<InfoBadge title="Tipo de Procedimento" variant="secondary">
  {#snippet badge()}
    {@render name()}
  {/snippet}
  <p>Forma como a entidade pública escolheu com quem celebrar o contrato.</p>
  <InfoPopoverItem>
    {#snippet title()}
      {@render name()}
    {/snippet}
    {#if procedureType.baseName}
      <p class="text-muted-foreground/60 text-[11px]" title="Nome original no Portal BASE">
        Nome no Portal BASE: «{procedureType.baseName}»
      </p>
    {/if}
    <p>{procedureType.description}</p>
    {#if procedureType.usage}
      <p>{procedureType.usage}</p>
    {/if}
    {#if procedureType.legalBasis}
      <Link url={procedureType.legalBasis.url} title="Consultar o {procedureType.legalBasis.text}">
        Definido no {procedureType.legalBasis.text}
      </Link>
    {/if}
  </InfoPopoverItem>
  <ul class="flex flex-wrap gap-x-3 gap-y-1 text-xs">
    {#each Object.entries(COMPETITION) as [level, { label, dot: color }] (level)}
      {@const current = level === procedureType.competition}
      <li
        class={cn("flex items-center gap-1.5", current && "text-foreground font-semibold")}
        aria-current={current}>
        {@render dot(color)}
        {label}
      </li>
    {/each}
  </ul>
  {#if !procedureType.legalBasis}
    <Link
      url="https://www.base.gov.pt/Base4/pt/documentacao/caracteristicas-dos-procedimentos/"
      title="Saiba mais sobre as características dos procedimentos">
      Saiba mais sobre as características dos procedimentos
    </Link>
  {/if}
</InfoBadge>
