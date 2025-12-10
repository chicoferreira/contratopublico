<script lang="ts">
  import { Badge } from "$lib/components/ui/badge";
  import { Popover, PopoverContent, PopoverTrigger } from "$lib/components/ui/popover";
  import type { MatchingRange } from "$lib/types/api";
  import Highlighted from "./Highlighted.svelte";
  import InfoPopover from "./InfoPopover.svelte";

  let {
    executionPlace,
    ranges: matchingRanges = [],
  }: { executionPlace: string; ranges?: MatchingRange[] | undefined } = $props();
</script>

<Popover>
  <PopoverTrigger>
    <Badge variant="secondary" class="h-6 min-h-6">
      <Highlighted
        content={executionPlace.length > 30
          ? executionPlace.substring(0, 30) + "..."
          : executionPlace}
        ranges={matchingRanges} />
    </Badge>
  </PopoverTrigger>
  <PopoverContent>
    <InfoPopover title="Localização da Execução do Contrato">
      <div>Este contrato foi executado em {executionPlace}.</div>
    </InfoPopover>
  </PopoverContent>
</Popover>
