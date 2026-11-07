<script lang="ts">
  import { CONTRACT_TYPE_LABELS, type ContractType, type MatchingRange } from "$lib/types/api";
  import { BriefcaseBusiness } from "@lucide/svelte";
  import Highlighted from "./Highlighted.svelte";
  import InfoBadge from "./InfoBadge.svelte";
  import InfoPopoverItem from "./InfoPopoverItem.svelte";
  import Link from "./Link.svelte";

  const DETAILS: Record<ContractType, { description: string; article?: string }> = {
    goodsAcquisition: {
      description: "A entidade pública compra bens móveis a um fornecedor.",
      article: "artigo 437.º",
    },
    servicesAcquisition: {
      description:
        "A entidade pública adquire a prestação de um ou vários tipos de serviços mediante o pagamento de um preço.",
      article: "artigo 450.º",
    },
    publicWorksConcession: {
      description:
        "A entidade pública encarrega um parceiro de executar uma obra pública. Em troca, o parceiro fica com o direito de explorar a obra, por vezes acompanhado do pagamento de um preço, e assume o risco dessa exploração.",
      article: "artigo 407.º",
    },
    publicServicesConcession: {
      description:
        "A entidade pública encarrega um parceiro de gerir e explorar um serviço. Em troca, o parceiro fica com o direito de explorar o serviço, por vezes acompanhado do pagamento de um preço, e assume o risco dessa exploração.",
      article: "artigo 407.º",
    },
    publicWorks: {
      description:
        "Execução, ou conceção e execução, de uma obra pública, como a construção, reabilitação, conservação ou demolição de um imóvel.",
      article: "artigo 343.º",
    },
    goodsLease: {
      description:
        "Um locador proporciona à entidade pública o uso temporário de bens móveis, mediante retribuição.",
      article: "artigo 431.º",
    },
    partnership: {
      description: "Contrato de sociedade celebrado pela entidade pública com outros parceiros.",
    },
    other: {
      description: "Contrato que não se enquadra em nenhum dos outros tipos.",
    },
  };

  let {
    type,
    mixed = false,
    ranges,
    index,
  }: { type: ContractType; mixed?: boolean; ranges?: MatchingRange[]; index?: number } = $props();

  const { description, article } = $derived(DETAILS[type]);
</script>

<InfoBadge title="Tipo de contrato">
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
