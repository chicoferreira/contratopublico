<script lang="ts">
  import { type SearchableContract, type SearchedContract } from "$lib/types/api";
  import { Building, CalendarDays, Signature, FileText, ExternalLink } from "@lucide/svelte";
  import ContractCardInfoRow from "./ContractCardInfoRow.svelte";
  import Highlighted from "./Highlighted.svelte";
  import ContractProcedureTypeBadge from "./ContractProcedureTypeBadge.svelte";
  import ContractTypeBadge from "./ContractTypeBadge.svelte";
  import ContractPrice from "./ContractPrice.svelte";
  import ContractNifDescriptionPopover from "./ContractNifDescriptionPopover.svelte";
  import ContractCpvBadge from "./ContractCpvBadge.svelte";
  import ContractPlaceBadge from "./ContractPlaceBadge.svelte";
  import { getBaseGovContractUrl } from "$lib";
  import { formatDate } from "$lib/utils";

  let { contract }: { contract: SearchedContract } = $props();

  function renderHighlightedField(field: keyof SearchableContract) {
    return {
      content: contract[field]?.toString(),
      ranges: contract.matchingRanges[field],
    };
  }
</script>

<div class="bg-card rounded-md border px-6 py-5">
  <!-- TODO: SIMPLIFY THIS CSS MESS -->
  <div class="pb-3">
    <div class="flex flex-col gap-2 md:flex-row md:items-start md:justify-between">
      <div class="space-y-1">
        <a
          class="text-base-content group block min-w-0 text-lg leading-tight font-semibold text-pretty"
          href={"/contract/" + contract.id}
          title={`Ver detalhes do contrato #${contract.id}`}
          aria-label={`Ver detalhes do contrato #${contract.id}`}>
          <Highlighted
            class="min-w-0 group-hover:underline"
            {...renderHighlightedField("objectBriefDescription")} />
          <ExternalLink
            aria-hidden="true"
            class="text-muted-foreground/80 group-hover:text-primary inline h-4 w-4 align-[-0.125em] transition-colors" />
        </a>
        <div class="flex flex-wrap items-start gap-x-2 gap-y-1">
          {#if contract.procedureType}
            <ContractProcedureTypeBadge
              type={contract.procedureType}
              titleRanges={contract.matchingRanges["procedureTypeTitle"]}
              subtitleRanges={contract.matchingRanges["procedureTypeSubtitle"]} />
          {/if}

          {#each contract.contractTypes as type, index (type)}
            <ContractTypeBadge
              {type}
              {index}
              mixed={contract.contractTypes.length > 1}
              ranges={contract.matchingRanges["contractTypeLabels"]} />
          {/each}

          {#each contract.executionPlaces as place, index (index)}
            <ContractPlaceBadge
              {place}
              {index}
              total={contract.executionPlaces.length}
              ranges={contract.matchingRanges} />
          {/each}

          {#each contract.cpvs as cpv, index (cpv.code)}
            <ContractCpvBadge {cpv} {index} ranges={contract.matchingRanges} />
          {/each}
        </div>
      </div>
      <ContractPrice
        initialContractualPrice={contract.initialContractualPrice}
        baseGovUrl={getBaseGovContractUrl(contract.id)} />
    </div>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2">
    <div class="md:space-y-2">
      <ContractCardInfoRow
        Icon={FileText}
        label={contract.contracting.length === 1 ? "Contratante" : "Contratantes"}>
        {#snippet popoverContent()}
          <p>Entidade pública responsável pela contratação e registo do contrato.</p>
          <p>
            Também conhecida como
            <span class="text-primary font-semibold"> Entidade adjudicante </span>.
          </p>
        {/snippet}

        {#snippet value()}
          {#each contract.contracting as contracting, index (contracting.id)}
            <div>
              <Highlighted
                content={contracting.description}
                ranges={contract.matchingRanges["contracting.description"]}
                {index} />

              <ContractNifDescriptionPopover>
                <Highlighted
                  content={contracting.nif}
                  ranges={contract.matchingRanges["contracting.nif"]}
                  {index} />
              </ContractNifDescriptionPopover>
            </div>
          {/each}
        {/snippet}
      </ContractCardInfoRow>
      <ContractCardInfoRow
        Icon={Building}
        label={contract.contracted.length === 1 ? "Contratado" : "Contratados"}>
        {#snippet popoverContent()}
          <p>Entidade selecionada para a prestação de serviços.</p>
          <p>
            Também conhecida como
            <span class="text-primary font-semibold"> Entidade adjudicatária </span>.
          </p>
        {/snippet}
        {#snippet value()}
          {#each contract.contracted as contracted, index (contracted.id)}
            <div>
              <Highlighted
                content={contracted.description}
                ranges={contract.matchingRanges["contracted.description"]}
                {index} />

              <ContractNifDescriptionPopover>
                <Highlighted
                  content={contracted.nif}
                  ranges={contract.matchingRanges["contracted.nif"]}
                  {index} />
              </ContractNifDescriptionPopover>
            </div>
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
