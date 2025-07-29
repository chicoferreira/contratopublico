<script lang="ts">
  import { Badge } from "$lib/components/ui/badge";
  import { Popover } from "$lib/components/ui/popover";
  import PopoverContent from "$lib/components/ui/popover/popover-content.svelte";
  import PopoverTrigger from "$lib/components/ui/popover/popover-trigger.svelte";
  import type { MatchingRange } from "$lib/types/api";
  import { cn } from "$lib/utils";
  import Highlighted from "./Highlighted.svelte";
  import Link from "./Link.svelte";
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

  const badgeConfig = $derived(getBadgeStyle(type));
</script>

<Popover>
  <PopoverTrigger>
    <Badge class={cn(badgeConfig.className, "border-transparent text-white")}>
      <Highlighted content={badgeConfig.displayText} ranges={highlightRanges} />
    </Badge>
  </PopoverTrigger>
  <PopoverContent class="space-y-0">
    <div class="text-base font-semibold">{badgeConfig.displayText}</div>

    <div class="space-y-2">
      <div>
        {#if badgeConfig.description}
          {@html badgeConfig.description}
        {:else}
          <div class="text-muted-foreground">
            <p>
              Ainda não há uma descrição disponível para este tipo de
              procedimento.
            </p>
            <p>
              Contribui com uma proposta de descrição no
              <Link
                class="text-blue-500"
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
          title="Saiba mais sobre as características dos procedimentos."
          class="text-blue-500">
          Saiba mais sobre as características dos procedimentos.
        </Link>
      </div>
    </div>
  </PopoverContent>
</Popover>
