<script lang="ts">
  import type { Contract } from "$lib/types/api";
  import { Building, CalendarDays, Signature, FileText } from "@lucide/svelte";
  import ContractCardInfoRow from "./ContractCardInfoRow.svelte";

  let { contract }: { contract: Contract } = $props();
</script>

<div
  class="bg-base-100 border border-neutral-content px-6 py-5 rounded-box
"
>
  <!-- TODO: SIMPLIFY THIS CSS MESS -->
  <div class="pb-3">
    <div
      class="flex flex-col md:flex-row md:items-start md:justify-between gap-2"
    >
      <div class="flex items-baseline gap-2">
        <h3 class="font-semibold text-lg leading-tight text-base-content">
          {contract.objectBriefDescription}
        </h3>
        <span class="text-xs text-base-content/50 shrink-0">#{contract.id}</span
        >
      </div>
      <div class="text-green-700 text-lg font-semibold lg:shrink-0">
        {contract.initialContractualPrice}
      </div>
    </div>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2">
    <div class="md:space-y-2">
      <ContractCardInfoRow
        Icon={FileText}
        label="Contratante"
        value={contract.contracting}
      />
      <ContractCardInfoRow
        Icon={Building}
        label="Contratado"
        value={contract.contracted}
      />
    </div>

    <div class="md:space-y-2">
      <ContractCardInfoRow
        Icon={CalendarDays}
        label="Data de Publicação"
        value={contract.publicationDate}
      />
      <ContractCardInfoRow
        Icon={Signature}
        label="Data do Contrato"
        value={contract.signingDate != null
          ? contract.signingDate
          : "Não informado"}
      />
    </div>
  </div>
</div>
