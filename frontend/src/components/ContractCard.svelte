<script lang="ts">
  import type { Contract } from "$lib/types/api";
  import {
    Building,
    CalendarDays,
    Signature,
    FileText,
    ExternalLink,
  } from "@lucide/svelte";
  import ContractCardInfoRow from "./ContractCardInfoRow.svelte";

  let { contract }: { contract: Contract } = $props();

  function formatMoney(value: number) {
    return new Intl.NumberFormat("pt-PT", {
      style: "currency",
      currency: "EUR",
    }).format(value / 100);
  }

  function getBaseGovUrl(id: number): string {
    return `https://www.base.gov.pt/Base4/pt/detalhe/?type=contratos&id=${id}`;
  }

  function formatDate(dateString: string): string {
    return new Date(dateString).toLocaleDateString("pt-PT", {
      year: "numeric",
      month: "2-digit", 
      day: "2-digit"
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
          {contract.objectBriefDescription}
        </h3>
        <a
          href={getBaseGovUrl(contract.id)}
          target="_blank"
          rel="noopener noreferrer"
          class="group flex w-fit shrink-0 items-center gap-[6px] transition-opacity hover:opacity-80"
          title="Ver detalhes no base.gov.pt">
          <span
            class="text-muted-foreground text-sm transition-colors group-hover:text-blue-500 group-hover:underline">
            base.gov.pt (#{contract.id})
          </span>
          <ExternalLink
            size={15}
            class="text-muted-foreground transition-colors group-hover:text-blue-500" />
        </a>
      </div>
      <div class="text-lg font-semibold text-green-700 lg:shrink-0">
        {formatMoney(contract.initialContractualPrice)}
      </div>
    </div>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2">
    <div class="md:space-y-2">
      <ContractCardInfoRow
        Icon={FileText}
        label="Contratante"
        value={contract.contracting}>
        {#snippet popoverContent()}
          <p>Órgão público que celebra e regista o contrato no BASE.</p>
          <p>
            Também conhecido como
            <strong class="font-semibold">Entidade adjudicante</strong>.
          </p>
        {/snippet}
      </ContractCardInfoRow>
      <ContractCardInfoRow
        Icon={Building}
        label="Contratado"
        value={contract.contracted}>
        {#snippet popoverContent()}
          <p>Entidade vencedora do concurso que será contratada.</p>
          <p>
            Também conhecido como
            <strong class="font-semibold">Entidade adjudicatária</strong>.
          </p>
        {/snippet}
      </ContractCardInfoRow>
    </div>

    <div class="md:space-y-2">
      <ContractCardInfoRow
        Icon={CalendarDays}
        label="Data de Publicação"
        value={formatDate(contract.publicationDate)}>
        {#snippet popoverContent()}
          <p>Data em que o contrato foi publicado no BASE.</p>
        {/snippet}
      </ContractCardInfoRow>
      <ContractCardInfoRow
        Icon={Signature}
        label="Data do Contrato"
        value={contract.signingDate != null
          ? formatDate(contract.signingDate)
          : "Não informado"}>
        {#snippet popoverContent()}
          <p>Data em que o contrato foi assinado/celebrado.</p>
        {/snippet}
      </ContractCardInfoRow>
    </div>
  </div>
</div>
