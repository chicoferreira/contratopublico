<script lang="ts">
  import * as Popover from "$lib/components/ui/popover/index.js";
  import InfoPopover from "./InfoPopover.svelte";
  import Link from "./Link.svelte";

  let { children } = $props();
</script>

<Popover.Root>
  <Popover.Trigger>
    <div class="group flex items-center gap-1 transition-colors">
      <span
        class="text-muted-foreground/80 group-hover:text-muted-foreground text-sm font-semibold transition-colors">
        ({@render children()})
      </span>
    </div>
  </Popover.Trigger>
  <Popover.Content>
    <InfoPopover title="Número de Identificação Fiscal (NIF)">
      <p>
        Número único usado para identificar uma pessoa ou entidade perante a Autoridade Tributária e
        Aduaneira.
      </p>
      <p>
        Por proteção de dados (RGPD), o NIF de pessoas singulares não é disponibilizado publicamente
        (<Link url="https://diariodarepublica.pt/dr/detalhe/lei/58-2019-123815982" showIcon={false}>
          Artigo 27.º da Lei n.º 58/2019
        </Link>).
      </p>
    </InfoPopover>
  </Popover.Content>
</Popover.Root>
