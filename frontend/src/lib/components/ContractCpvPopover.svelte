<script lang="ts">
  import InfoBadge from "./InfoBadge.svelte";
  import Highlighted from "./Highlighted.svelte";
  import Link from "./Link.svelte";

  let { cpv, index, ranges } = $props();
</script>

<InfoBadge title="Vocabulário Comum para os Contratos Públicos (CPV)" variant="outline">
  {#snippet badge()}
    <Highlighted content={cpv.code} ranges={ranges["cpvs.code"]} {index} />
  {/snippet}
  <p>
    Sistema que classifica e identifica a categoria dos contratos públicos, tornando as referências
    usadas pelas entidades mais padronizadas.
  </p>
  <dl class="text-base-content">
    <div class="flex flex-wrap items-baseline gap-2">
      <dt class="text-primary text-xs font-semibold tracking-wide uppercase">Código</dt>
      <dd class="leading-snug font-medium">
        <Highlighted content={cpv.code} ranges={ranges["cpvs.code"]} {index} />
      </dd>
    </div>

    <div class="flex flex-wrap items-baseline gap-2">
      <dt class="text-primary text-xs font-semibold tracking-wide uppercase">Designação</dt>
      <dd class="min-w-0 flex-1 leading-snug break-words">
        <Highlighted content={cpv.designation} ranges={ranges["cpvs.designation"]} {index} />
      </dd>
    </div>
  </dl>
  <Link url="https://ted.europa.eu/pt/simap/cpv" title="Saiba mais sobre o CPV">
    Saiba mais sobre CPVs
  </Link>
</InfoBadge>
