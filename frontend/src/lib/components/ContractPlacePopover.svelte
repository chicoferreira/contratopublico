<script lang="ts">
  import { Badge } from "$lib/components/ui/badge";
  import { Popover, PopoverContent, PopoverTrigger } from "$lib/components/ui/popover";
  import type { MatchingRange } from "$lib/types/api";
  import Highlighted from "./Highlighted.svelte";
  import InfoPopover from "./InfoPopover.svelte";

  let {
    place,
    ranges: matchingRanges = [],
  }: { place: string; ranges?: MatchingRange[] | undefined } = $props();
</script>

<Popover>
  <PopoverTrigger>
    <Badge variant="secondary" class="h-6 min-h-6">
      <Highlighted
        content={place.length > 30
          ? place.substring(0, 30) + "..."
          : place}
        ranges={matchingRanges} />
    </Badge>
  </PopoverTrigger>
  <PopoverContent>
    <InfoPopover title="Localização da Execução do Contrato">
      <div>Este contrato foi executado em {place}.</div>
    </InfoPopover>
  </PopoverContent>
</Popover>
