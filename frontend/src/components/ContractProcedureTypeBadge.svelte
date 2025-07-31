<script lang="ts">
  import { Badge } from "$lib/components/ui/badge";
  import {
    Popover,
    PopoverContent,
    PopoverTrigger,
  } from "$lib/components/ui/popover";
  import type { MatchingRange } from "$lib/types/api";
  import { cn } from "$lib/utils";
  import Highlighted from "./Highlighted.svelte";
  import Link from "./Link.svelte";
  import InfoPopover from "./InfoPopover.svelte";
  import contractTypesData from "./procedure-types.json";

  function getBadgeStyle(contractType: string) {
    return (
      contractTypesData[contractType as keyof typeof contractTypesData] || {
        displayText: contractType,
        className: "bg-gray-700",
        description: null,
      }
    );
  }

  let {
    type,
    highlightRanges,
  }: { type: string; highlightRanges: MatchingRange[] } = $props();

  const colorVariants = {
    blue: "bg-blue-600 dark:bg-blue-600/70",
    sky: "bg-sky-600 dark:bg-sky-600/70",
    cyan: "bg-cyan-600 dark:bg-cyan-600/70",
    green: "bg-green-600 dark:bg-green-600/70",
    emerald: "bg-emerald-600 dark:bg-emerald-600/70",
    lime: "bg-lime-600 dark:bg-lime-600/70",
    teal: "bg-teal-600 dark:bg-teal-600/70",
    purple: "bg-purple-600 dark:bg-purple-600/70",
    violet: "bg-violet-600 dark:bg-violet-600/70",
    indigo: "bg-indigo-600 dark:bg-indigo-600/70",
    slate: "bg-slate-600 dark:bg-slate-600/70",
    pink: "bg-pink-600 dark:bg-pink-600/70",
    rose: "bg-rose-600 dark:bg-rose-600/70",
    orange: "bg-orange-600 dark:bg-orange-600/70",
    amber: "bg-amber-600 dark:bg-amber-600/70",
    yellow: "bg-yellow-600 dark:bg-yellow-600/70",
    stone: "bg-stone-600 dark:bg-stone-600/70",
    fuchsia: "bg-fuchsia-600 dark:bg-fuchsia-600/70",
    neutral: "bg-neutral-600 dark:bg-neutral-600/70",
    zinc: "bg-zinc-600 dark:bg-zinc-600/70",
    red: "bg-red-600 dark:bg-red-600/70",
    gray: "bg-gray-600 dark:bg-gray-600/70",
  };

  const badgeConfig = $derived(getBadgeStyle(type));
</script>

<Popover>
  <PopoverTrigger>
    <Badge
      class={cn(
        colorVariants[badgeConfig.color as keyof typeof colorVariants],
        "cursor-pointer font-semibold text-white",
      )}>
      <Highlighted content={badgeConfig.displayText} ranges={highlightRanges} />
    </Badge>
  </PopoverTrigger>
  <PopoverContent>
    <InfoPopover title={badgeConfig.displayText}>
      {#snippet content()}
        <div>
          {#if badgeConfig.description}
            <div>
              {@html badgeConfig.description}
            </div>
          {:else}
            <div>
              <p>Descrição não disponível para este tipo de procedimento.</p>
              <p>
                Contribua com uma descrição no
                <Link
                  showIcon={false}
                  url="https://github.com/chicoferreira/contratopublico/edit/main/frontend/src/components/procedure-types.json">
                  GitHub
                </Link>.
              </p>
            </div>
          {/if}
        </div>

        <div>
          <Link
            url="https://www.base.gov.pt/Base4/pt/documentacao/caracteristicas-dos-procedimentos/"
            title="Saiba mais sobre as características dos procedimentos.">
            Saiba mais sobre as características dos procedimentos.
          </Link>
        </div>
      {/snippet}
    </InfoPopover>
  </PopoverContent>
</Popover>
