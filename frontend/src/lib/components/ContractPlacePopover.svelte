<script lang="ts">
  import type { MatchingRange } from "$lib/types/api";
  import Highlighted from "./Highlighted.svelte";
  import InfoBadge from "./InfoBadge.svelte";

  let {
    place,
    ranges: matchingRanges = [],
  }: { place: string; ranges?: MatchingRange[] | undefined } = $props();
</script>

<InfoBadge title="Localização da Execução do Contrato" variant="secondary" class="h-6 min-h-6">
  {#snippet badge()}
    <Highlighted
      content={place.length > 30 ? place.substring(0, 30) + "..." : place}
      ranges={matchingRanges} />
  {/snippet}
  <div>Este contrato foi executado em {place}.</div>
</InfoBadge>
