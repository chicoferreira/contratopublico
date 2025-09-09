<script lang="ts">
  import type { Contract, MatchingRanges } from "$lib/types/api";
  import { Building, CalendarDays, Signature, FileText } from "@lucide/svelte";
  import ContractCardInfoRow from "./ContractCardInfoRow.svelte";
  import Highlighted from "./Highlighted.svelte";
  import ContractTypeBadge from "./ContractProcedureTypeBadge.svelte";
  import Link from "./Link.svelte";
  import ContractPrice from "./ContractPrice.svelte";

  let { contract }: { contract: Contract & MatchingRanges } = $props();

  function renderHighlightedField(field: keyof Contract & keyof MatchingRanges["matchingRanges"]) {
    return {
      content: contract[field]?.toString(),
      ranges: contract.matchingRanges[field],
    };
  }

  const baseGovUrl = $derived(
    `https://www.base.gov.pt/Base4/pt/detalhe/?type=contratos&id=${contract.id}`,
  );

  function formatDate(dateString: string): string {
    return new Date(dateString).toLocaleDateString("pt-PT", {
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
    });
  }
</script>

<div class="bg-card rounded-md border px-6 py-5">
  <!-- TODO: SIMPLIFY THIS CSS MESS -->
  <div class="pb-3">
    <div class="flex flex-col gap-2 md:flex-row md:items-start md:justify-between">
      <div class="space-y-1">
        <h3 class="text-base-content text-lg leading-tight font-semibold">
          <Highlighted {...renderHighlightedField("objectBriefDescription")} />
        </h3>
        <div class="flex flex-wrap items-center gap-x-2 gap-y-1">
          <Link url={baseGovUrl} title="Ver detalhes no base.gov.pt">
            base.gov.pt (#<Highlighted {...renderHighlightedField("id")} />)
          </Link>

          <ContractTypeBadge
            type={contract.contractingProcedureType}
            highlightRanges={renderHighlightedField("contractingProcedureType").ranges} />
        </div>
      </div>
      <ContractPrice initialContractualPrice={contract.initialContractualPrice} {baseGovUrl} />
    </div>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2">
    <div class="md:space-y-2">
      <ContractCardInfoRow
        Icon={FileText}
        label={contract.contracting.length > 0 ? "Contratante" : "Contratantes"}>
        {#snippet popoverContent()}
          <p>Entidade pública responsável pela contratação e registo do contrato.</p>
          <p>
            Também conhecida como
            <span class="text-primary font-semibold"> Entidade adjudicante </span>.
          </p>
        {/snippet}

        {#snippet value()}
          {#each contract.contracting as contracting, index}
            <p>
              <Highlighted
                content={contracting.description}
                ranges={contract.matchingRanges["contracting.description"]}
                {index} />
            </p>
          {/each}
        {/snippet}
      </ContractCardInfoRow>
      <ContractCardInfoRow
        Icon={Building}
        label={contract.contracted.length > 0 ? "Contratado" : "Contratados"}>
        {#snippet popoverContent()}
          <p>Entidade selecionada para a prestação de serviços.</p>
          <p>
            Também conhecida como
            <span class="text-primary font-semibold"> Entidade adjudicatária </span>.
          </p>
        {/snippet}
        {#snippet value()}
          {#each contract.contracted as contracted, index}
            <p>
              <Highlighted
                content={contracted.description}
                ranges={contract.matchingRanges["contracted.description"]}
                {index} />
            </p>
          {/each}
        {/snippet}
      </ContractCardInfoRow>
    </div>

    <div class="md:space-y-2">
      <ContractCardInfoRow Icon={CalendarDays} label="Data de Publicação">
        {#snippet popoverContent()}
          <p>Data de publicação do contrato na plataforma BASE.</p>
        {/snippet}
        {#snippet value()}
          {contract.publicationDate != null
            ? formatDate(contract.publicationDate)
            : "Não informado"}
        {/snippet}
      </ContractCardInfoRow>
      <ContractCardInfoRow Icon={Signature} label="Data do Contrato">
        {#snippet popoverContent()}
          <p>Data de assinatura e formalização do contrato.</p>
        {/snippet}
        {#snippet value()}
          {contract.signingDate != null ? formatDate(contract.signingDate) : "Não informado"}
        {/snippet}
      </ContractCardInfoRow>
    </div>
  </div>
</div>
