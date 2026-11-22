<script lang="ts">
  import { Tag } from "@lucide/svelte";
  import InfoBadge from "./InfoBadge.svelte";
  import ContractCpvItem from "./ContractCpvItem.svelte";
  import Highlighted from "./Highlighted.svelte";
  import Link from "./Link.svelte";

  let { cpv, index, ranges } = $props();
</script>

<InfoBadge title="Vocabulário Comum para os Contratos Públicos (CPV)" variant="secondary">
  {#snippet badge()}
    <Tag aria-hidden="true" />
    <Highlighted content={cpv.code} ranges={ranges["cpvs.code"]} {index} />
    <Highlighted
      class="text-muted-foreground max-w-48 truncate font-normal"
      content={cpv.designation}
      ranges={ranges["cpvs.designation"]}
      {index} />
  {/snippet}
  <p>
    Sistema que classifica e identifica a categoria dos contratos públicos, tornando as referências
    usadas pelas entidades mais padronizadas.
  </p>
  <ContractCpvItem {cpv} {index} {ranges} />
  <Link url="https://ted.europa.eu/pt/simap/cpv" title="Saiba mais sobre o CPV">
    Saiba mais sobre CPVs
  </Link>
</InfoBadge>
