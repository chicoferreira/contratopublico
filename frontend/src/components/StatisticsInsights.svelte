<script lang="ts">
  import { onMount } from "svelte";
  import { blur, fade } from "svelte/transition";
  import type { Statistics } from "$lib/types/api";
  import { Skeleton } from "$lib/components/ui/skeleton/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import InfoPopover from "./InfoPopover.svelte";
  import Link from "./Link.svelte";

  const moneyFormatter = new Intl.NumberFormat("pt-PT", {
    style: "currency",
    currency: "EUR",
  });

  const numberFormatter = new Intl.NumberFormat("pt-PT");

  function formatMoney(value: number) {
    return moneyFormatter.format(value / 100);
  }

  function formatNumber(value: number) {
    return numberFormatter.format(value);
  }

  let { statistics }: { statistics: Statistics } = $props();

  const insight_types = [
    "LAST_365_SPENT",
    "LAST_30_SPENT",
    "LAST_7_SPENT",
    "LAST_365_CONTRACTS",
    "LAST_30_CONTRACTS",
    "LAST_7_CONTRACTS",
  ] as const;

  function randomInsightType(): (typeof insight_types)[number] {
    const randomIndex = Math.floor(Math.random() * insight_types.length);
    return insight_types[randomIndex];
  }

  let currentInsightType: (typeof insight_types)[number] | undefined = $state();

  onMount(() => {
    const INSIGHT_ROTATION_INTERVAL_MS = 10000;

    currentInsightType = randomInsightType();

    const interval = setInterval(() => {
      currentInsightType = randomInsightType();
    }, INSIGHT_ROTATION_INTERVAL_MS);

    return () => clearInterval(interval);
  });
</script>

<Popover.Root>
  <Popover.Trigger>
    <div class="text-muted-foreground text-left transition" transition:fade={{ duration: 300 }}>
      {#if !currentInsightType}
        <Skeleton class="inline-block h-4 w-64" />
      {/if}
      {#key currentInsightType}
        <span class="flex-1" in:blur={{ duration: 300 }}>
          {#if currentInsightType === "LAST_365_SPENT"}
            Foram gastos
            <span class="text-primary font-semibold">
              {formatMoney(statistics.totalSpentLast365Days)}
            </span>
            em contratos públicos nos últimos 365 dias.
          {:else if currentInsightType === "LAST_30_SPENT"}
            Foram gastos
            <span class="text-primary font-semibold">
              {formatMoney(statistics.totalSpentLast30Days)}
            </span>
            em contratos públicos nos últimos 30 dias.
          {:else if currentInsightType === "LAST_7_SPENT"}
            Foram gastos
            <span class="text-primary font-semibold">
              {formatMoney(statistics.totalSpentLast7Days)}
            </span>
            em contratos públicos nos últimos 7 dias.
          {:else if currentInsightType === "LAST_365_CONTRACTS"}
            Foram realizados
            <span class="text-primary font-semibold">
              {formatNumber(statistics.contractsLast365Days)}
            </span>
            contratos públicos nos últimos 365 dias.
          {:else if currentInsightType === "LAST_30_CONTRACTS"}
            Foram realizados
            <span class="text-primary font-semibold">
              {formatNumber(statistics.contractsLast30Days)}
            </span>
            contratos públicos nos últimos 30 dias.
          {:else if currentInsightType === "LAST_7_CONTRACTS"}
            Foram realizados
            <span class="text-primary font-semibold">
              {formatNumber(statistics.contractsLast7Days)}
            </span>
            contratos públicos nos últimos 7 dias.
          {/if}
        </span>
      {/key}
    </div>
  </Popover.Trigger>
  <Popover.Content>
    <InfoPopover title="Página de Estatísticas">
      <p>
        Uma página de estatísticas com vários gráficos para exploração dos dados está em
        desenvolvimento.
      </p>
      <p>
        Podes acompanhar ou contribuir através do
        <Link url="https://github.com/chicoferreira/contratopublico/issues/10">
          <span class="italic">tracking</span>
          da <span class="italic">issue</span>
        </Link>.
      </p>
    </InfoPopover>
  </Popover.Content>
</Popover.Root>
