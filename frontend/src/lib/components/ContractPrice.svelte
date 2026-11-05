<script lang="ts">
  import { Popover, PopoverContent, PopoverTrigger } from "$lib/components/ui/popover";
  import { formatMoney } from "$lib/utils";
  import InfoPopover from "./InfoPopover.svelte";
  import Link from "./Link.svelte";

  const { baseGovUrl, initialContractualPrice } = $props();
  const income = $derived(initialContractualPrice < 0);
</script>

<Popover>
  <PopoverTrigger
    class="w-fit text-lg font-semibold text-green-600 transition hover:text-green-700 lg:shrink-0">
    {formatMoney(initialContractualPrice)}
  </PopoverTrigger>
  <PopoverContent>
    <InfoPopover title="Valor Contratual Inicial">
      <p>Valor inicial estabelecido no momento da contratação.</p>
      {#if income}
        <p>
          Como é um <strong>valor negativo</strong>, a entidade contratante recebe este valor em vez
          de o pagar (por exemplo numa concessão).
        </p>
      {/if}
      <p>Este valor pode ser alterado durante a execução do contrato.</p>
      <p>
        <Link url={baseGovUrl}>Consulte sempre o valor atual no BASE.</Link>
      </p>
    </InfoPopover>
  </PopoverContent>
</Popover>
