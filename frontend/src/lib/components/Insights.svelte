<script lang="ts">
  import { onMount } from "svelte";
  import { blur } from "svelte/transition";
  import type { Statistics } from "$lib/types/api";
  import { Skeleton } from "$lib/components/ui/skeleton/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import InfoPopover from "./InfoPopover.svelte";
  import Link from "./Link.svelte";
  import { isRecentFeature } from "$lib";
  import { CHANGELOG } from "$lib/changelog";
  import { formatMoney, formatNumber } from "$lib/utils";

  let { statistics }: { statistics: Statistics } = $props();

  type Insight =
    | { type: "spent"; value: number; days: number }
    | { type: "contracts"; value: number; days: number }
    | { type: "announcement"; text: string };

  const insights: Insight[] = $derived([
    { type: "spent", value: statistics.totalSpentLast365Days, days: 365 },
    { type: "spent", value: statistics.totalSpentLast30Days, days: 30 },
    { type: "spent", value: statistics.totalSpentLast7Days, days: 7 },
    { type: "contracts", value: statistics.contractsLast365Days, days: 365 },
    { type: "contracts", value: statistics.contractsLast30Days, days: 30 },
    { type: "contracts", value: statistics.contractsLast7Days, days: 7 },
    ...CHANGELOG.filter((entry) => entry.announcement && isRecentFeature(entry.date)).map(
      (entry): Insight => ({ type: "announcement", text: entry.announcement! }),
    ),
  ]);

  let index = $state<number>();

  const current = $derived(index === undefined ? undefined : insights[index]);

  onMount(() => {
    const ROTATION_INTERVAL_MS = 10000;

    const pick = () => (index = Math.floor(Math.random() * insights.length));
    pick();
    const interval = setInterval(pick, ROTATION_INTERVAL_MS);
    return () => clearInterval(interval);
  });
</script>

{#snippet statisticsInfo()}
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
{/snippet}

<div class="text-muted-foreground text-left">
  {#if !current}
    <Skeleton class="inline-block h-4 w-64" />
  {:else}
    {#key index}
      <div in:blur={{ duration: 300 }}>
        {#if current.type === "spent"}
          <Popover.Root>
            <Popover.Trigger class="text-left">
              Foram gastos
              <span class="text-primary font-semibold">{formatMoney(current.value)}</span>
              em contratos públicos nos últimos {current.days} dias.
            </Popover.Trigger>
            <Popover.Content>{@render statisticsInfo()}</Popover.Content>
          </Popover.Root>
        {:else if current.type === "contracts"}
          <Popover.Root>
            <Popover.Trigger class="text-left">
              Foram realizados
              <span class="text-primary font-semibold">{formatNumber(current.value)}</span>
              contratos públicos nos últimos {current.days} dias.
            </Popover.Trigger>
            <Popover.Content>{@render statisticsInfo()}</Popover.Content>
          </Popover.Root>
        {:else if current.type === "announcement"}
          <a href="/changelog" class="hover:underline">
            <span class="text-primary font-semibold">Novo:</span>
            {current.text}
          </a>
        {/if}
      </div>
    {/key}
  {/if}
</div>
