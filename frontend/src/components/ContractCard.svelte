<script lang="ts">
  import type { Contract } from "$lib/types/api";
  import { Building, CalendarDays, Signature, FileText } from "@lucide/svelte";
  import ContractCardInfoRow from "./ContractCardInfoRow.svelte";

  let { contract }: { contract: Contract } = $props();

  function formatMoney(value: number) {
    return new Intl.NumberFormat("pt-PT", {
      style: "currency",
      currency: "EUR",
    }).format(value / 100);
  }
</script>

<div class="bg-background border-neutral-content rounded-md border px-6 py-5">
  <!-- TODO: SIMPLIFY THIS CSS MESS -->
  <div class="pb-3">
    <div
      class="flex flex-col gap-2 md:flex-row md:items-start md:justify-between">
      <div class="flex items-baseline gap-2">
        <h3 class="text-base-content text-lg leading-tight font-semibold">
          {contract.objectBriefDescription}
        </h3>
        <span class="text-muted-foreground shrink-0 text-xs"
          >#{contract.id}</span>
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
        value={contract.contracting} />
      <ContractCardInfoRow
        Icon={Building}
        label="Contratado"
        value={contract.contracted} />
    </div>

    <div class="md:space-y-2">
      <ContractCardInfoRow
        Icon={CalendarDays}
        label="Data de Publicação"
        value={new Date(contract.publicationDate).toLocaleDateString()} />
      <ContractCardInfoRow
        Icon={Signature}
        label="Data do Contrato"
        value={contract.signingDate != null
          ? new Date(contract.signingDate).toLocaleDateString()
          : "Não informado"} />
    </div>
  </div>
</div>
