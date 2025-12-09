<script lang="ts">
  import { Popover, PopoverContent, PopoverTrigger } from "$lib/components/ui/popover";
  import { formatMoney } from "$lib/utils";
  import InfoPopover from "./InfoPopover.svelte";
  import Link from "./Link.svelte";

  const { baseGovUrl, initialContractualPrice } = $props();
</script>

<Popover>
  <PopoverTrigger
    class="w-fit text-lg font-semibold text-green-600 transition hover:text-green-700 lg:shrink-0">
    {formatMoney(initialContractualPrice)}
  </PopoverTrigger>
  <PopoverContent>
    <InfoPopover title="Valor Contratual Inicial">
      <div>
        <p>Valor inicial estabelecido no momento da contratação.</p>
        <p>Este valor pode ser alterado durante a execução do contrato.</p>
      </div>
      <div>
        <p>
          <Link url={baseGovUrl}>Consulte sempre o valor atual no BASE.</Link>
        </p>
      </div>
    </InfoPopover>
  </PopoverContent>
</Popover>
