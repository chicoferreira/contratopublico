<script lang="ts">
  import type { ContractType, MatchingRange } from "$lib/types/api";
  import { BriefcaseBusiness } from "@lucide/svelte";
  import Highlighted from "./Highlighted.svelte";
  import InfoBadge from "./InfoBadge.svelte";
  import InfoPopoverItem from "./InfoPopoverItem.svelte";
  import Link from "./Link.svelte";
  import { CONTRACT_TYPES } from "$lib/contract-types";

  let {
    type,
    mixed = false,
    ranges,
    index,
  }: { type: ContractType; mixed?: boolean; ranges?: MatchingRange[]; index?: number } = $props();

  const contractType = $derived(CONTRACT_TYPES[type]);

  const CCP_URL =
    "https://www.base.gov.pt/Base4/pt/documentacao/codigo-dos-contratos-publicos-consolidado/";
</script>

<InfoBadge title="Tipo de Contrato">
  {#snippet badge()}
    <BriefcaseBusiness aria-hidden="true" />
    <Highlighted content={contractType.label} {ranges} {index} />
  {/snippet}
  <p>
    Classificação do contrato segundo o Código dos Contratos Públicos, consoante o que a entidade
    pública contrata.
  </p>
  <InfoPopoverItem icon={BriefcaseBusiness}>
    {#snippet title()}
      <Highlighted content={contractType.label} {ranges} {index} />
    {/snippet}
    <p>{contractType.description}</p>
    {#if contractType.article}
      <Link url={CCP_URL} title="Consultar o Código dos Contratos Públicos">
        Definido no {contractType.article} do Código dos Contratos Públicos
      </Link>
    {/if}
  </InfoPopoverItem>
  {#if mixed}
    <p>
      Este contrato tem mais do que um tipo porque junta prestações de tipos diferentes (contrato
      misto).
    </p>
  {/if}
  {#if !contractType.article}
    <Link url={CCP_URL} title="Consultar o Código dos Contratos Públicos">
      Saiba mais no Código dos Contratos Públicos
    </Link>
  {/if}
</InfoBadge>
