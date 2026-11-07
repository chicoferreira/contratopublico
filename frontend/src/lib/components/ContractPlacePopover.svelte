<script lang="ts">
  import type { MatchingRange } from "$lib/types/api";
  import { MapPin } from "@lucide/svelte";
  import Highlighted from "./Highlighted.svelte";
  import InfoBadge from "./InfoBadge.svelte";

  let {
    place,
    ranges: matchingRanges = [],
  }: { place: string; ranges?: MatchingRange[] | undefined } = $props();
</script>

<InfoBadge title="Localização da Execução do Contrato" variant="secondary">
  {#snippet badge()}
    <MapPin aria-hidden="true" />
    <Highlighted
      content={place.length > 30 ? place.substring(0, 30) + "..." : place}
      ranges={matchingRanges} />
  {/snippet}
  <div>Este contrato foi executado em {place}.</div>
</InfoBadge>
