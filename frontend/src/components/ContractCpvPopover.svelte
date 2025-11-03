<script lang="ts">
  import { Popover, PopoverContent, PopoverTrigger } from "$lib/components/ui/popover";
  import Badge from "$lib/components/ui/badge/badge.svelte";
  import InfoPopover from "./InfoPopover.svelte";
  import Highlighted from "./Highlighted.svelte";
  import Link from "./Link.svelte";

  let { cpv, index, ranges } = $props();
</script>

<Popover>
  <PopoverTrigger>
    <Badge variant="outline">
      <Highlighted content={cpv.code} ranges={ranges["cpvs.code"]} {index} />
    </Badge>
  </PopoverTrigger>
  <PopoverContent>
    <InfoPopover title="Vocabulário Comum para os Contratos Públicos (CPV)">
      <p>
        Sistema que classifica e identifica a categoria dos contratos públicos, tornando as
        referências usadas pelas entidades mais padronizadas.
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
          <dd class="leading-snug flex-1 min-w-0 break-words">
            <Highlighted content={cpv.designation} ranges={ranges["cpvs.designation"]} {index} />
          </dd>
        </div>
      </dl>
      <Link url="https://ted.europa.eu/pt/simap/cpv" title="Saiba mais sobre o CPV">
        Saiba mais sobre CPVs
      </Link>
    </InfoPopover>
  </PopoverContent>
</Popover>
