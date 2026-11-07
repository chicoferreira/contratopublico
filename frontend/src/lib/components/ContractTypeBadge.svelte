<script lang="ts">
  import { CONTRACT_TYPE_LABELS, type ContractType, type MatchingRange } from "$lib/types/api";
  import { BriefcaseBusiness } from "@lucide/svelte";
  import Highlighted from "./Highlighted.svelte";
  import InfoBadge from "./InfoBadge.svelte";
  import InfoPopoverItem from "./InfoPopoverItem.svelte";
  import Link from "./Link.svelte";
  import { CONTRACT_TYPE_ARTICLES, CONTRACT_TYPE_DESCRIPTIONS } from "$lib/contract-types";

  let {
    type,
    mixed = false,
    ranges,
    index,
  }: { type: ContractType; mixed?: boolean; ranges?: MatchingRange[]; index?: number } = $props();

  const description = $derived(CONTRACT_TYPE_DESCRIPTIONS[type]);
  const article = $derived(CONTRACT_TYPE_ARTICLES[type]);
</script>

<InfoBadge title="Tipo de Contrato">
  {#snippet badge()}
    <BriefcaseBusiness aria-hidden="true" />
    <Highlighted content={CONTRACT_TYPE_LABELS[type]} {ranges} {index} />
  {/snippet}
  <p>
    Classificação do contrato segundo o Código dos Contratos Públicos, consoante o que a entidade
    pública contrata.
  </p>
  <InfoPopoverItem icon={BriefcaseBusiness}>
    {#snippet title()}
      <Highlighted content={CONTRACT_TYPE_LABELS[type]} {ranges} {index} />
    {/snippet}
    {description}
  </InfoPopoverItem>
  {#if mixed}
    <p>
      Este contrato tem mais do que um tipo porque junta prestações de tipos diferentes (contrato
      misto).
    </p>
  {/if}
  <Link
    url="https://www.base.gov.pt/Base4/pt/documentacao/codigo-dos-contratos-publicos-consolidado/"
    title="Consultar o Código dos Contratos Públicos">
    {article
      ? `Definido no ${article} do Código dos Contratos Públicos`
      : "Saiba mais no Código dos Contratos Públicos"}
  </Link>
</InfoBadge>
