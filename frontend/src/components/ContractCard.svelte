<script lang="ts">
  import type { Contract, MatchingRanges } from "$lib/types/api";
  import { Building, CalendarDays, Signature, FileText } from "@lucide/svelte";
  import ContractCardInfoRow from "./ContractCardInfoRow.svelte";
  import Highlighted from "./Highlighted.svelte";
  import ContractTypeBadge from "./ContractProcedureTypeBadge.svelte";
  import Link from "./Link.svelte";
  import {
    Popover,
    PopoverContent,
    PopoverTrigger,
  } from "$lib/components/ui/popover";

  let { contract }: { contract: Contract & MatchingRanges } = $props();

  function renderHighlightedField(
    field: keyof Contract & keyof MatchingRanges["matchingRanges"],
  ) {
    return {
      content: contract[field]?.toString(),
      ranges: contract.matchingRanges[field],
    };
  }

  function formatMoney(value: number) {
    return new Intl.NumberFormat("pt-PT", {
      style: "currency",
      currency: "EUR",
    }).format(value / 100);
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

<div class="bg-background border-neutral-content rounded-md border px-6 py-5">
  <!-- TODO: SIMPLIFY THIS CSS MESS -->
  <div class="pb-3">
    <div
      class="flex flex-col gap-2 md:flex-row md:items-start md:justify-between">
      <div class="space-y-1">
        <h3 class="text-base-content text-lg leading-tight font-semibold">
          <Highlighted {...renderHighlightedField("objectBriefDescription")} />
        </h3>
        <div class="flex flex-wrap items-center gap-x-2 gap-y-1">
          <Link
            url={baseGovUrl}
            title="Ver detalhes no base.gov.pt"
            external={true}>
            base.gov.pt (#<Highlighted {...renderHighlightedField("id")} />)
          </Link>

          <ContractTypeBadge
            type={contract.contractingProcedureType}
            highlightRanges={renderHighlightedField("contractingProcedureType")
              .ranges} />
        </div>
      </div>
      <div class="text-lg font-semibold text-green-700 lg:shrink-0">
        <Popover>
          <PopoverTrigger>
            {formatMoney(contract.initialContractualPrice)}
          </PopoverTrigger>
          <PopoverContent class="space-y-0">
            <div class="text-base font-semibold">Valor Contratual Inicial</div>

            <div class="space-y-2">
              <div class="text-muted-foreground">
                <p>Valor inicial estabelecido no momento da contratação.</p>
                <p>
                  Este valor pode ser alterado durante a execução do contrato.
                </p>
              </div>
              <div>
                <p>
                  <Link class="text-blue-500" url={baseGovUrl}>
                    Consulte sempre o valor atual no BASE.
                  </Link>
                </p>
              </div>
            </div>
          </PopoverContent>
        </Popover>
      </div>
    </div>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2">
    <div class="md:space-y-2">
      <ContractCardInfoRow Icon={FileText} label="Contratante">
        {#snippet popoverContent()}
          <p>
            Entidade pública responsável pela contratação e registo do contrato
            no BASE.
          </p>
          <p>
            Também conhecida como
            <strong class="font-semibold">Entidade adjudicante</strong>.
          </p>
        {/snippet}

        {#snippet value()}
          <Highlighted {...renderHighlightedField("contracting")} />
        {/snippet}
      </ContractCardInfoRow>
      <ContractCardInfoRow Icon={Building} label="Contratado">
        {#snippet popoverContent()}
          <p>
            Entidade selecionada para a prestação de serviços ou fornecimento.
          </p>
          <p>
            Também conhecida como
            <strong class="font-semibold">Entidade adjudicatária</strong>.
          </p>
        {/snippet}
        {#snippet value()}
          <Highlighted {...renderHighlightedField("contracted")} />
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
          {contract.signingDate != null
            ? formatDate(contract.signingDate)
            : "Não informado"}
        {/snippet}
      </ContractCardInfoRow>
    </div>
  </div>
</div>
